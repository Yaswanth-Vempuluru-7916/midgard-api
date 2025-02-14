use axum::{extract::{Query, State}, Json};
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use futures::stream::StreamExt;
use crate::db::models::{DepthHistoryDocument, DepthHistoryMeta};

#[derive(Debug, Deserialize)]
pub struct DepthHistoryParams {
    pub interval: Option<String>, // e.g., "hour", "day"
    pub count: Option<usize>,     // Number of records to return
    pub from: Option<i64>,        // Start timestamp
    pub to: Option<i64>,          // End timestamp
}

#[derive(Debug, Serialize)]
pub struct DepthHistoryResponse {
    pub meta: DepthHistoryMeta,
    pub intervals: Vec<crate::db::models::DepthHistory>,
}

/// Converts interval type to seconds
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

/// Handles GET /api/depth-history
pub async fn get_depth_history(
    State(db): State<Arc<Database>>,  
    Query(params): Query<DepthHistoryParams>,
) -> Json<DepthHistoryResponse> {
    let collection: Collection<DepthHistoryDocument> = db.collection("depth_history");

    let count = params.count.unwrap_or(10).min(400);
    let interval_seconds = params.interval.as_deref().and_then(interval_to_seconds).unwrap_or(3600);
    let mut query = doc! {};

    if let Some(from) = params.from {
        query.insert("meta.startTime", doc! { "$gte": from });
    }
    if let Some(to) = params.to {
        query.insert("meta.endTime", doc! { "$lte": to });
    }

    let mut cursor = collection.find(query, None).await.unwrap();
    let mut intervals = Vec::new();
    let mut meta_start_time = None;
    let mut meta_end_time = None;

    while let Some(Ok(doc)) = cursor.next().await {
        for interval in doc.intervals {
            if intervals.len() < count && (meta_end_time.unwrap_or(0) + interval_seconds) <= interval.end_time {
                if meta_start_time.is_none() {
                    meta_start_time = Some(interval.start_time);
                }
                meta_end_time = Some(interval.end_time);
                intervals.push(interval);
            } else {
                break;
            }
        }
        if intervals.len() >= count {
            break;
        }
    }

    let meta = DepthHistoryMeta {
        start_time: meta_start_time.unwrap_or(0),
        end_time: meta_end_time.unwrap_or(0),
        ..Default::default()
    };

    Json(DepthHistoryResponse { meta, intervals })
}
