use axum::{extract::{Query, State}, Json};
use mongodb::{bson::{doc, Document}, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use futures::stream::StreamExt;
use crate::db::models::{DepthHistoryDocument, DepthHistory};

#[derive(Debug, Deserialize)]
pub struct DepthHistoryParams {
    pub interval: Option<String>, // "hour", "day", "week", etc.
    pub count: Option<usize>,     // Number of intervals (only used with interval)
    pub from: Option<i64>,        // Start timestamp
    pub to: Option<i64>,          // End timestamp
}

/// **Response Meta**
#[derive(Debug, Serialize)]
pub struct DepthHistoryMetaResponse {
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
}

/// **Response Structure**
#[derive(Debug, Serialize)]
pub struct DepthHistoryResponse {
    pub meta: DepthHistoryMetaResponse,
    pub intervals: Vec<DepthHistory>,
}

/// Converts interval type to seconds
fn interval_to_seconds(interval: &str) -> Option<i64> {
    match interval {
        "5min" => Some(300),
        "hour" => Some(3600),
        "day" => Some(86400),
        "week" => Some(86400 * 7),
        "month" => Some(86400 * 30),
        "quarter" => Some(86400 * 90),
        "year" => Some(86400 * 365),
        _ => None,
    }
}

/// Handles GET /api/depth-history
pub async fn get_depth_history(
    State(db): State<Arc<Database>>,  
    Query(params): Query<DepthHistoryParams>,
) -> Json<DepthHistoryResponse> {
    let collection: Collection<DepthHistoryDocument> = db.collection("depth_history");

    let count = params.count.unwrap_or(10).min(400);
    let interval_seconds = params.interval.as_deref().and_then(interval_to_seconds).unwrap_or(3600);

    // **Round `from` down to the nearest interval boundary**
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
            "assetDepth": { "$sum": "$intervals.assetDepth" },
            "runeDepth": { "$sum": "$intervals.runeDepth" },
            "liquidityUnits": { "$sum": "$intervals.liquidityUnits" },
            "synthSupply": { "$sum": "$intervals.synthSupply" },
            "synthUnits": { "$sum": "$intervals.synthUnits" },
            "totalUnits": { "$sum": "$intervals.units" },
            "membersCount": { "$avg": "$intervals.membersCount" },
            "assetPrice": { "$avg": "$intervals.assetPrice" },
            "assetPriceUSD": { "$avg": "$intervals.assetPriceUSD" },
            "luvi": { "$avg": "$intervals.luvi" },
            "startTime": { "$min": "$intervals.startTime" },
            "endTime": { "$max": "$intervals.endTime" }
        }
    });

    // **Sort results by `startTime` in ascending order**
    pipeline.push(doc! { "$sort": { "_id.intervalStart": 1 } });

    // **Limit results based on `count`**
    pipeline.push(doc! { "$limit": count as i64 });

    // **Execute the pipeline**
    let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
    let mut intervals = Vec::new();
    let mut meta_start_time = None;
    let mut meta_end_time = None;

    while let Some(Ok(doc)) = cursor.next().await {
        let interval = DepthHistory {
            asset_depth: doc.get_f64("assetDepth").unwrap_or(0.0),
            asset_price: doc.get_f64("assetPrice").unwrap_or(0.0),
            asset_price_usd: doc.get_f64("assetPriceUSD").unwrap_or(0.0),
            liquidity_units: doc.get_f64("liquidityUnits").unwrap_or(0.0),
            members_count: doc.get_f64("membersCount").unwrap_or(0.0) as i32,
            rune_depth: doc.get_f64("runeDepth").unwrap_or(0.0),
            start_time: doc.get_i64("startTime").unwrap_or(0),
            end_time: doc.get_i64("endTime").unwrap_or(0),
            synth_supply: doc.get_f64("synthSupply").unwrap_or(0.0),
            synth_units: doc.get_f64("synthUnits").unwrap_or(0.0),
            total_units: doc.get_f64("totalUnits").unwrap_or(0.0),
            luvi: doc.get_f64("luvi").unwrap_or(0.0),
        };
        if intervals.is_empty() {
            meta_start_time = Some(interval.start_time);
        }
        meta_end_time = Some(interval.end_time);
        intervals.push(interval);
    }

    // **Build response meta**
    let meta = DepthHistoryMetaResponse {
        start_time: meta_start_time.unwrap_or(from),
        end_time: meta_end_time.unwrap_or(to),
    };

    Json(DepthHistoryResponse { meta, intervals })
}