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
    let six_months_ago = now - (6 * 30 * 24 * 3600);
    // let six_months_ago = now - (1 * 24);
    // let six_months_ago = now - (1 * 24 * 3600);

    // Fetch & store each dataset
    // fetch_and_store_depth_history(&client, &db, six_months_ago, now).await;
    fetch_and_store_earnings_history(&client, &db, six_months_ago, now).await;
    // fetch_and_store_swaps_history(&client, &db, six_months_ago, now).await;
    // fetch_and_store_rune_pool_history(&client, &db, six_months_ago, now).await;
}

/// Fetch and store depth history
// async fn fetch_and_store_depth_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
//     let collection: Collection<DepthHistoryDocument> = db.collection("depth_history");
//     fetch_paginated_data(client, &collection, "depths/BTC.BTC", start_time, end_time).await;
// }

/// Fetch and store earnings history
async fn fetch_and_store_earnings_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
    let collection: Collection<EarningsHistoryDocument> = db.collection("earnings_history");
    fetch_paginated_data(client, &collection, "earnings", start_time, end_time).await;
}

/// Fetch and store swaps history
// async fn fetch_and_store_swaps_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
//     let collection: Collection<SwapsHistoryDocument> = db.collection("swaps_history");
//     fetch_paginated_data(client, &collection, "swaps", start_time, end_time).await;
// }

/// Fetch and store rune pool history
async fn fetch_and_store_rune_pool_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
    let collection: Collection<RunePoolHistoryDocument> = db.collection("rune_pool_history");
    fetch_paginated_data(client, &collection, "runepool", start_time, end_time).await;
}



//  Fetch paginated data from Midgard and store it in MongoDB
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

        println!("🔄 Fetching data from: {}", url);  // ✅ Debugging log

        match client.get(&url).send().await {
            Ok(response) => {
                match response.text().await {
                    Ok(body) => {
                        println!("🔍 Response from {}:\n {}", endpoint, body);
                        match serde_json::from_str::<Value>(&body) {
                            Ok(json) => {
                                match serde_json::from_value::<T>(json.clone()) {
                                    Ok(doc) => {
                                        match collection.insert_one(&doc, None).await {
                                            Ok(_) => println!("✅ Inserted document into {}", endpoint),
                                            Err(e) => println!("❌ Failed to insert record into {}: {:?}", endpoint, e),
                                        }
                                    }
                                    Err(e) => println!("❌ Failed to deserialize full response for {}: {:?}", endpoint, e),
                                }

                                // // ✅ Use `meta.endTime` as `from=` for pagination
                                // if let Some(meta) = json.get("meta") {
                                //     if let Some(new_start_time) = meta.get("endTime").and_then(|v| v.as_i64()) {
                                //         if new_start_time > current_time {  // ✅ Ensure we are moving forward
                                //             current_time = new_start_time;  // ✅ Correctly update `from=`
                                //         } else {
                                //             println!("🚨 Warning: Pagination stopped early for {}", endpoint);
                                //             break;  // ❌ Prevent infinite loop
                                //         }
                                //     } else {
                                //         println!("🚨 Warning: No `meta.endTime` found for {}", endpoint);
                                //         break;  // ❌ Stop if `meta.endTime` is missing
                                //     }
                                // }
                                if let Some(meta) = json.get("meta") {
                                    if let Some(end_time_str) = meta.get("endTime").and_then(|v| v.as_str()) {
                                        if let Ok(new_start_time) = end_time_str.parse::<i64>() {
                                            if new_start_time > current_time {
                                                current_time = new_start_time;
                                            } else {
                                                println!("🚨 Warning: Pagination stopped early for {}", endpoint);
                                                break;
                                            }
                                        } else {
                                            println!("🚨 Warning: Failed to parse `meta.endTime` as i64 for {}", endpoint);
                                            break;
                                        }
                                    } else {
                                        println!("🚨 Warning: No `meta.endTime` found for {}", endpoint);
                                        break;
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
