use axum::{extract::{Query, State}, Json};
use mongodb::{bson::{self, doc, Bson}, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use futures::stream::StreamExt;
use crate::db::models::{EarningsHistoryDocument, EarningsHistory, EarningsPool};

#[derive(Debug, Deserialize)]
pub struct EarningsHistoryParams {
    pub interval: Option<String>, // "hour", "day", "week", etc.
    pub count: Option<usize>,     // Number of intervals (max 400)
    pub from: Option<i64>,        // Start timestamp
    pub to: Option<i64>,          // End timestamp
    pub limit: Option<usize>,     // Limit for pagination
    pub page: Option<usize>,      // Pagination page
    pub sort_by: Option<String>,  // Sorting field
    pub order: Option<String>,    // "asc" or "desc"
    pub filters: Option<Vec<String>>, // Example: ["liquidityFees>1000", "blockRewards<500"]
}

#[derive(Debug, Serialize)]
pub struct EarningsHistoryMetaResponse {
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
}

#[derive(Debug, Serialize)]
pub struct EarningsHistoryResponse {
    pub meta: EarningsHistoryMetaResponse,
    pub intervals: Vec<EarningsHistory>,
}

fn interval_to_seconds(interval: &str) -> Option<i64> {
    match interval {
        "hour" => Some(3600),
        "day" => Some(86400),
        "week" => Some(86400 * 7),
        "month" => Some(86400 * 30),
        "quarter" => Some(86400 * 90),
        "year" => Some(86400 * 365),
        _ => None,
    }
}

pub async fn get_earnings_history(
    State(db): State<Arc<Database>>,  
    Query(params): Query<EarningsHistoryParams>,
) -> Json<EarningsHistoryResponse> {
    let collection: Collection<EarningsHistoryDocument> = db.collection("earnings_history");

    let _count = params.count.unwrap_or(10).min(400);
    let limit = params.limit.unwrap_or(10);
    let page = params.page.unwrap_or(1).max(1);
    let interval_seconds = params.interval.as_deref().and_then(interval_to_seconds).unwrap_or(3600);

    let from = params.from.map(|f| f - (f % interval_seconds)).unwrap_or(0);
    let to = params.to.unwrap_or(i64::MAX);

    let mut pipeline = vec![];

    // **Filter documents based on `from` and `to` time range**
    pipeline.push(doc! {
        "$match": {
            "intervals.startTime": { "$gte": from },
            "intervals.endTime": { "$lte": to }
        }
    });

    // **Unwind intervals array**
    pipeline.push(doc! { "$unwind": "$intervals" });

    // **Filter intervals to ensure they start after the rounded `from` timestamp**
    pipeline.push(doc! {
        "$match": {
            "intervals.startTime": { "$gte": from }
        }
    });

    // **Apply dynamic filters**
    if let Some(filters) = &params.filters {
        let mut filter_conditions = vec![];
        for filter in filters {
            let parts: Vec<&str> = filter.split(|c| c == '>' || c == '<' || c == '=').collect();
            if parts.len() == 2 {
                let field = parts[0].trim();
                let value: f64 = parts[1].trim().parse().unwrap_or(0.0);
                let operator = if filter.contains(">=") {
                    "$gte"
                } else if filter.contains("<=") {
                    "$lte"
                } else if filter.contains(">") {
                    "$gt"
                } else if filter.contains("<") {
                    "$lt"
                } else {
                    "$eq"
                };

                filter_conditions.push(doc! { format!("intervals.{}", field): { operator: value } });
            }
        }
        if !filter_conditions.is_empty() {
            pipeline.push(doc! { "$match": { "$and": filter_conditions } });
        }
    }

    // **Group by interval boundaries**
    pipeline.push(doc! {
        "$group": {
            "_id": {
                "intervalStart": {
                    "$subtract": [
                        "$intervals.startTime",
                        { "$mod": ["$intervals.startTime", interval_seconds] }
                    ]
                }
            },
            "liquidityFees": { "$sum": "$intervals.liquidityFees" },
            "blockRewards": { "$sum": "$intervals.blockRewards" },
            "earnings": { "$sum": "$intervals.earnings" },
            "bondingEarnings": { "$sum": "$intervals.bondingEarnings" },
            "liquidityEarnings": { "$sum": "$intervals.liquidityEarnings" },
            "avgNodeCount": { "$avg": "$intervals.avgNodeCount" },
            "runePriceUSD": { "$avg": "$intervals.runePriceUSD" },
            "startTime": { "$min": "$intervals.startTime" },
            "endTime": { "$max": "$intervals.endTime" },
            "pools": { "$push": "$intervals.pools" } // Push all pools into an array
        }
    });

    // **Sorting**
    if let Some(sort_by) = &params.sort_by {
        let sort_order = match params.order.as_deref() {
            Some("desc") => -1,
            _ => 1,
        };
        pipeline.push(doc! { "$sort": { sort_by: sort_order } });
    } else {
        pipeline.push(doc! { "$sort": { "_id.intervalStart": 1 } });
    }

    // **Pagination**
    let skip_count = (page - 1) * limit;
    pipeline.push(doc! { "$skip": skip_count as i64 });
    pipeline.push(doc! { "$limit": limit as i64 });

    // **Execute the pipeline**
    let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
    let mut intervals = Vec::new();
    let mut meta_start_time = None;
    let mut meta_end_time = None;

    while let Some(Ok(doc)) = cursor.next().await {
        // **Flatten the pools array**
        let pools: Vec<EarningsPool> = doc.get_array("pools")
            .map(|pools_array| {
                pools_array.iter()
                    .flat_map(|pool_bson| {
                        if let Bson::Array(pool_array) = pool_bson {
                            pool_array.iter()
                                .filter_map(|p| bson::from_bson::<EarningsPool>(p.clone()).ok())
                                .collect()
                        } else {
                            vec![]
                        }
                    })
                    .collect()
            })
            .unwrap_or_else(|_| Vec::new());


        let interval = EarningsHistory {
            liquidity_fees: doc.get_f64("liquidityFees").unwrap_or(0.0),
            block_rewards: doc.get_f64("blockRewards").unwrap_or(0.0),
            earnings: doc.get_f64("earnings").unwrap_or(0.0),
            bonding_earnings: doc.get_f64("bondingEarnings").unwrap_or(0.0),
            liquidity_earnings: doc.get_f64("liquidityEarnings").unwrap_or(0.0),
            avg_node_count: doc.get_f64("avgNodeCount").unwrap_or(0.0) as f64,
            rune_price_usd: doc.get_f64("runePriceUSD").unwrap_or(0.0),
            start_time: doc.get_i64("startTime").unwrap_or(0),
            end_time: doc.get_i64("endTime").unwrap_or(0),
            pools,
        };

        if intervals.is_empty() {
            meta_start_time = Some(interval.start_time);
        }
        meta_end_time = Some(interval.end_time);
        intervals.push(interval);
    }

    // **Build response meta**
    let meta = EarningsHistoryMetaResponse {
        start_time: meta_start_time.unwrap_or(from),
        end_time: meta_end_time.unwrap_or(to),
    };

    Json(EarningsHistoryResponse { meta, intervals })
}
