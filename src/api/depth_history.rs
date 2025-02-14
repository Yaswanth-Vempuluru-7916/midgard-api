use axum::{extract::{Query, State}, Json};
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use futures::stream::StreamExt;
use crate::db::models::{DepthHistoryDocument, DepthHistoryMeta};

#[derive(Debug, Deserialize)]
pub struct DepthHistoryParams {
    pub interval: Option<String>,
    pub count: Option<usize>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct DepthHistoryResponse {
    pub meta: DepthHistoryMeta,
    pub intervals: Vec<crate::db::models::DepthHistory>,
}

/// Handles GET /api/depth-history
pub async fn get_depth_history(
    State(db): State<Arc<Database>>,  // ✅ Fix: Use `State<Arc<Database>>`
    Query(params): Query<DepthHistoryParams>,
) -> Json<DepthHistoryResponse> {
    let collection: Collection<DepthHistoryDocument> = db.collection("depth_history");

    let count = params.count.unwrap_or(10).min(400);
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
            if intervals.len() < count {
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
