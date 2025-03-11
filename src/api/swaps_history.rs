use axum::{extract::{Query, State}, Json};
use mongodb::{bson::{doc, Bson}, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use futures::stream::StreamExt; // Needed for Cursor to use `.next()`
use crate::db::models::{SwapsHistoryDocument, SwapsHistory};

#[derive(Debug, Deserialize)]
pub struct SwapsHistoryParams {
    pub interval: Option<String>, // "hour", "day", "week", etc.
    pub from: Option<i64>,        // Start timestamp
    pub to: Option<i64>,          // End timestamp
    pub page: Option<usize>,      // Page for pagination
    pub limit: Option<usize>,     // Limit for pagination
    pub sort: Option<String>,     // Sort field (e.g., "startTime", "endTime")
    pub order: Option<String>,    // Sort order ("asc" or "desc")
}

#[derive(Debug, Serialize)]
pub struct SwapsHistoryMetaResponse {
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
}

#[derive(Debug, Serialize)]
pub struct SwapsHistoryResponse {
    pub meta: SwapsHistoryMetaResponse,
    pub intervals: Vec<SwapsHistory>,
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

pub async fn get_swaps_history(
    State(db): State<Arc<Database>>,
    Query(params): Query<SwapsHistoryParams>,
) -> Json<SwapsHistoryResponse> {
    let collection: Collection<SwapsHistoryDocument> = db.collection("swaps_history");

    let interval_seconds = params.interval.as_deref().and_then(interval_to_seconds).unwrap_or(3600);

    let from = params.from.unwrap_or(0);
    let to = params.to.unwrap_or(i64::MAX);

    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // Convert pagination values to BSON types
    let bson_skip = Bson::Int64(((page - 1) * limit) as i64);
    let bson_limit = Bson::Int64(limit as i64);

    // Define the sort field and order (ascending or descending)
    let sort_field = params.sort.unwrap_or_else(|| "startTime".to_string());
    let sort_order = match params.order.as_deref() {
        Some("desc") => -1, // descending
        _ => 1, // ascending (default)
    };

    let mut sort_doc = doc! {};
    sort_doc.insert(sort_field, Bson::Int32(sort_order));

    let mut pipeline = vec![];

    // **Initial Match: Filter documents based on `from` and `to` time range**
    pipeline.push(doc! {
        "$match": {
            "meta.startTime": { "$lte": to },
            "meta.endTime": { "$gte": from }
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

    // **Group by interval boundaries (aggregate intervals)**
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
            "toAssetCount": { "$sum": "$intervals.toAssetCount" },
            "toRuneCount": { "$sum": "$intervals.toRuneCount" },
            "toTradeCount": { "$sum": "$intervals.toTradeCount" },
            "fromTradeCount": { "$sum": "$intervals.fromTradeCount" },
            "toSecuredCount": { "$sum": "$intervals.toSecuredCount" },
            "fromSecuredCount": { "$sum": "$intervals.fromSecuredCount" },
            "synthMintCount": { "$sum": "$intervals.synthMintCount" },
            "synthRedeemCount": { "$sum": "$intervals.synthRedeemCount" },
            "totalCount": { "$sum": "$intervals.totalCount" },
            "toAssetVolume": { "$sum": "$intervals.toAssetVolume" },
            "toRuneVolume": { "$sum": "$intervals.toRuneVolume" },
            "toTradeVolume": { "$sum": "$intervals.toTradeVolume" },
            "fromTradeVolume": { "$sum": "$intervals.fromTradeVolume" },
            "toSecuredVolume": { "$sum": "$intervals.toSecuredVolume" },
            "fromSecuredVolume": { "$sum": "$intervals.fromSecuredVolume" },
            "synthMintVolume": { "$sum": "$intervals.synthMintVolume" },
            "synthRedeemVolume": { "$sum": "$intervals.synthRedeemVolume" },
            "totalVolume": { "$sum": "$intervals.totalVolume" },
            "runePriceUSD": { "$avg": "$intervals.runePriceUSD" },
            "startTime": { "$min": "$intervals.startTime" },
            "endTime": { "$max": "$intervals.endTime" }
        }
    });

    // **Sort results by the specified sort field**
    pipeline.push(doc! { "$sort": sort_doc });

    // **Skip based on page and limit**
    pipeline.push(doc! { "$skip": bson_skip });

    // **Limit the number of records based on `limit`**
    pipeline.push(doc! { "$limit": bson_limit });

    // **Execute the aggregation pipeline**
    let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
    let mut intervals = Vec::new();
    let mut meta_start_time = None;
    let mut meta_end_time = None;

    // **Process the aggregated data**
    while let Some(Ok(doc)) = cursor.next().await {
        let interval = SwapsHistory {
            start_time: doc.get_i64("startTime").unwrap_or(0),
            end_time: doc.get_i64("endTime").unwrap_or(0),
            to_asset_count: doc.get_i32("toAssetCount").unwrap_or(0),
            to_rune_count: doc.get_i32("toRuneCount").unwrap_or(0),
            to_trade_count: doc.get_i32("toTradeCount").unwrap_or(0),
            from_trade_count: doc.get_i32("fromTradeCount").unwrap_or(0),
            to_secured_count: doc.get_i32("toSecuredCount").unwrap_or(0),
            from_secured_count: doc.get_i32("fromSecuredCount").unwrap_or(0),
            synth_mint_count: doc.get_i32("synthMintCount").unwrap_or(0),
            synth_redeem_count: doc.get_i32("synthRedeemCount").unwrap_or(0),
            total_count: doc.get_i32("totalCount").unwrap_or(0),
            to_asset_volume: doc.get_f64("toAssetVolume").unwrap_or(0.0),
            to_rune_volume: doc.get_f64("toRuneVolume").unwrap_or(0.0),
            to_trade_volume: doc.get_f64("toTradeVolume").unwrap_or(0.0),
            from_trade_volume: doc.get_f64("fromTradeVolume").unwrap_or(0.0),
            to_secured_volume: doc.get_f64("toSecuredVolume").unwrap_or(0.0),
            from_secured_volume: doc.get_f64("fromSecuredVolume").unwrap_or(0.0),
            synth_mint_volume: doc.get_f64("synthMintVolume").unwrap_or(0.0),
            synth_redeem_volume: doc.get_f64("synthRedeemVolume").unwrap_or(0.0),
            total_volume: doc.get_f64("totalVolume").unwrap_or(0.0),
            rune_price_usd: doc.get_f64("runePriceUSD").unwrap_or(0.0),
        };

        if intervals.is_empty() {
            meta_start_time = Some(interval.start_time);
        }
        meta_end_time = Some(interval.end_time);
        intervals.push(interval);
    }

    // **Determine the start and end times for the pagination response**
    let meta = SwapsHistoryMetaResponse {
        start_time: meta_start_time.unwrap_or(from),
        end_time: meta_end_time.unwrap_or(to),
    };

    Json(SwapsHistoryResponse { meta, intervals })
}