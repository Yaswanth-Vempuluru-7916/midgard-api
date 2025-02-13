// utils/midgard_fetch.rs
use reqwest::Client;
use mongodb::{bson::doc, Collection, Database};
use chrono::{Utc, Duration};
use std::sync::Arc;
use serde_json::Value;

use crate::db::models::{DepthHistoryDocument, EarningsHistoryDocument, RunePoolHistoryDocument, SwapsHistoryDocument};

const MIDGARD_BASE_URL: &str = "https://midgard.ninerealms.com/v2/history";

/// Fetches and stores the last 6 months of hourly data
pub async fn fetch_and_store_data(db: Arc<Database>) {
    let client = Client::new();
    let now = Utc::now().timestamp();
    // let six_months_ago = now - (6 * 30 * 24 * 3600);
    let six_months_ago = now - (1 * 24 * 3600);

    // Fetch & store each dataset
    fetch_and_store_depth_history(&client, &db, six_months_ago, now).await;
    // fetch_and_store_earnings_history(&client, &db, six_months_ago, now).await;
    // fetch_and_store_swaps_history(&client, &db, six_months_ago, now).await;
    // fetch_and_store_rune_pool_history(&client, &db, six_months_ago, now).await;
}

/// Fetch and store depth history
async fn fetch_and_store_depth_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
    let collection: Collection<DepthHistoryDocument> = db.collection("depth_history");
    fetch_paginated_data(client, &collection, "depths/BTC.BTC", start_time, end_time).await;
}

/// Fetch and store earnings history
// async fn fetch_and_store_earnings_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
//     let collection: Collection<EarningsHistoryDocument> = db.collection("earnings_history");
//     fetch_paginated_data(client, &collection, "earnings", start_time, end_time).await;
// }

/// Fetch and store swaps history
// async fn fetch_and_store_swaps_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
//     let collection: Collection<SwapsHistoryDocument> = db.collection("swaps_history");
//     fetch_paginated_data(client, &collection, "swaps", start_time, end_time).await;
// }

/// Fetch and store rune pool history
// async fn fetch_and_store_rune_pool_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
//     let collection: Collection<RunePoolHistoryDocument> = db.collection("rune_pool_history");
//     fetch_paginated_data(client, &collection, "runepool", start_time, end_time).await;
// }



//  Fetch paginated data from Midgard and store it in MongoDB
// async fn fetch_paginated_data<T>(
//     client: &Client,
//     collection: &Collection<T>,
//     endpoint: &str,
//     mut start_time: i64,
//     end_time: i64,
// ) where
//     T: serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
// {
//     let mut current_time = start_time;  // ✅ Start pagination from 6 months ago

//     while current_time < end_time {  // ✅ Fetch data until `end_time` (now)
//         let url = format!(
//             "{}/{endpoint}?interval=hour&count=400&from={current_time}",
//             MIDGARD_BASE_URL
//         );

//         match client.get(&url).send().await {
//             Ok(response) => {
//                 match response.text().await {
//                     Ok(body) => {
//                         println!("🔍 Response from {}:\n {}", endpoint, body);
//                         match serde_json::from_str::<Value>(&body) {
//                             Ok(json) => {
//                                 if let Some(intervals) = json.get("intervals").and_then(|v| v.as_array()) {
//                                     match serde_json::from_value::<Vec<T>>(Value::Array(intervals.clone())) {
//                                         Ok(docs) => {
//                                             let doc_count = docs.len();
//                                             if !docs.is_empty() {
//                                                 match collection.insert_many(docs, None).await {
//                                                     Ok(_) => println!("✅ Inserted {} records into {}", doc_count, endpoint),
//                                                     Err(e) => println!("❌ Failed to insert records into {}: {:?}", endpoint, e),
//                                                 }
//                                             }
//                                         }
//                                         Err(e) => println!("❌ Failed to deserialize data for {}: {:?}", endpoint, e),
//                                     }
//                                 }

//                                 // ✅ Use `meta.endTime` as `from=` for next request
//                                 if let Some(meta) = json.get("meta") {
//                                     if let Some(new_start_time) = meta.get("endTime").and_then(|v| v.as_i64()) {
//                                         current_time = new_start_time;  // ✅ Correct pagination
//                                     } else {
//                                         break;  // ✅ Stop if `endTime` is missing
//                                     }
//                                 }
//                             }
//                             Err(e) => println!("❌ Failed to parse JSON for {}: {:?}", endpoint, e),
//                         }
//                     }
//                     Err(e) => println!("❌ Failed to read response body from {}: {:?}", endpoint, e),
//                 }
//             }
//             Err(e) => {
//                 println!("❌ Failed to fetch {}: {:?}", endpoint, e);
//                 break;
//             }
//         }
//     }
// }


async fn fetch_paginated_data<T>(
    client: &Client,
    collection: &Collection<T>,
    endpoint: &str,
    mut start_time: i64,
    end_time: i64,
) where
    T: serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    let mut current_time = start_time;

    while current_time < end_time {
        let url = format!(
            "{}/{endpoint}?interval=hour&count=400&from={current_time}",
            MIDGARD_BASE_URL
        );

        match client.get(&url).send().await {
            Ok(response) => {
                match response.text().await {
                    Ok(body) => {
                        println!("🔍 Response from {}:\n {}", endpoint, body);
                        match serde_json::from_str::<Value>(&body) {
                            Ok(json) => {
                                // ✅ Deserialize the entire response, including `meta` and `intervals`
                                match serde_json::from_value::<T>(json.clone()) {
                                    Ok(doc) => {
                                        match collection.insert_one(&doc, None).await {
                                            Ok(_) => println!("✅ Inserted 1 document into {}", endpoint),
                                            Err(e) => println!("❌ Failed to insert record into {}: {:?}", endpoint, e),
                                        }
                                    }
                                    Err(e) => println!("❌ Failed to deserialize full response for {}: {:?}", endpoint, e),
                                }

                                // ✅ Use `meta.endTime` as `from=` for pagination
                                if let Some(meta) = json.get("meta") {
                                    if let Some(new_start_time) = meta.get("endTime").and_then(|v| v.as_i64()) {
                                        current_time = new_start_time;  // ✅ Correct pagination
                                    } else {
                                        break;  // ✅ Stop if `endTime` is missing
                                    }
                                }
                            }
                            Err(e) => println!("❌ Failed to parse JSON for {}: {:?}", endpoint, e),
                        }
                    }
                    Err(e) => println!("❌ Failed to read response body from {}: {:?}", endpoint, e),
                }
            }
            Err(e) => {
                println!("❌ Failed to fetch {}: {:?}", endpoint, e);
                break;
            }
        }
    }
}
