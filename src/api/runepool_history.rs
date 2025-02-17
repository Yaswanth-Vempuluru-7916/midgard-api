use axum::{extract::{Query, State}, Json};
use mongodb::{bson::{doc, Bson}, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::models::{RunePoolHistoryDocument, RunePoolHistory, RunePoolHistoryMeta};
use futures::stream::StreamExt; // Bring in StreamExt to use `.next()`

#[derive(Debug, Deserialize)]
pub struct RunePoolHistoryParams {
    pub interval: Option<String>, // "hour", "day", "week", etc.
    pub count: Option<usize>,     // Number of intervals (max 400)
    pub from: Option<i64>,        // Start timestamp
    pub to: Option<i64>,          // End timestamp
    pub page: Option<usize>,      // Page number for pagination
    pub limit: Option<usize>,     // Limit the number of items per page
    pub sort: Option<String>,     // Sorting field (e.g., "startTime", "endTime")
    pub order: Option<String>,    // Sort order ("asc", "desc")
    pub filters: Option<String>,  // Filtering conditions (e.g., "count>10")
}

#[derive(Debug, Serialize)]
pub struct RunePoolHistoryMetaResponse {
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
}

#[derive(Debug, Serialize)]
pub struct RunePoolHistoryResponse {
    pub meta: RunePoolHistoryMetaResponse,
    pub intervals: Vec<RunePoolHistory>,
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

pub async fn get_rune_pool_history(
    State(db): State<Arc<Database>>,
    Query(params): Query<RunePoolHistoryParams>,
) -> Json<RunePoolHistoryResponse> {
    let collection: Collection<RunePoolHistoryDocument> = db.collection("rune_pool_history");

    let count = params.count.unwrap_or(10).min(400); // Handle count with max limit of 400
    let interval_seconds = params.interval.as_deref().and_then(interval_to_seconds).unwrap_or(3600);

    let from = params.from.unwrap_or(0);
    let to = params.to.unwrap_or(i64::MAX);

    let page = params.page.unwrap_or(1); // Default page is 1
    let limit = params.limit.unwrap_or(10); // Default limit is 10

    let skip = (page - 1) * limit;

    let mut pipeline = vec![];

    // **Initial Match: Filter documents based on `from` and `to` time range**
    pipeline.push(doc! {
        "$match": {
            "meta.startTime": { "$lte": to },
            "meta.endTime": { "$gte": from }
        }
    });

    // **Apply Filters (if any)**
    if let Some(filters) = params.filters {
        // Example: Filters could be a simple condition like "count>10"
        // This is a basic example and can be extended to more complex filter logic
        let filter_doc = doc! {
            "$expr": {
                "$gt": [
                    { "$sum": "$intervals.count" },
                    10
                ]
            }
        };
        pipeline.push(filter_doc);
    }

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
            "count": { "$sum": "$intervals.count" },
            "units": { "$sum": "$intervals.units" },
            "startTime": { "$min": "$intervals.startTime" },
            "endTime": { "$max": "$intervals.endTime" }
        }
    });

    // **Sort by the given sorting field and order (if specified)**
    if let Some(sort) = &params.sort {
        let sort_doc = if let Some(order) = &params.order {
            match order.as_str() {
                "desc" => doc! { sort.clone(): -1 },
                _ => doc! { sort.clone(): 1 }, // Default is ascending
            }
        } else {
            doc! { "startTime": 1 } // Default sorting by startTime in ascending order
        };
        pipeline.push(doc! { "$sort": sort_doc });
    }

    // **Limit the number of records based on `count`**
    pipeline.push(doc! { "$limit": count as i64 });

    // **Skip for pagination**
    pipeline.push(doc! { "$skip": skip as i64 });

    // **Execute the aggregation pipeline**
    let mut cursor = collection.aggregate(pipeline, None).await.unwrap();
    let mut intervals = Vec::new();
    let mut meta_start_time = None;
    let mut meta_end_time = None;

    // **Process the aggregated data**
    while let Some(Ok(doc)) = cursor.next().await {
        let interval = RunePoolHistory {
            start_time: doc.get_i64("startTime").unwrap_or(0),
            end_time: doc.get_i64("endTime").unwrap_or(0),
            count: doc.get_i32("count").unwrap_or(0),
            units: doc.get_f64("units").unwrap_or(0.0),
        };

        if intervals.is_empty() {
            meta_start_time = Some(interval.start_time);
        }
        meta_end_time = Some(interval.end_time);
        intervals.push(interval);
    }

    // **Determine the start and end times for the pagination response**
    let meta = RunePoolHistoryMetaResponse {
        start_time: meta_start_time.unwrap_or(from),
        end_time: meta_end_time.unwrap_or(to),
    };

    Json(RunePoolHistoryResponse { meta, intervals })
}
