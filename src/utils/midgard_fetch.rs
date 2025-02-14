use reqwest::Client;
use mongodb::{bson::doc, Collection, Database};
use chrono::{Utc, Duration};
use std::sync::Arc;
use serde_json::Value;
use futures::stream::StreamExt;  // ✅ Ensure this is imported


use crate::db::models::{DepthHistoryDocument, EarningsHistoryDocument, RunePoolHistoryDocument, SwapsHistoryDocument};

const MIDGARD_BASE_URL: &str = "https://midgard.ninerealms.com/v2/history";

/// Fetches and stores only new hourly data, avoiding duplicates
pub async fn fetch_and_store_data(db: Arc<Database>) {
    let client = Client::new();
    let now = Utc::now().timestamp();

    // ✅ Find the latest timestamp in MongoDB, fallback to 6 months ago if empty
    // let last_stored_time = get_last_stored_timestamp(&db, "depth_history").await.unwrap_or(1739512851);
    let last_stored_time = get_last_stored_timestamp(&db, "depth_history").await.unwrap_or(now - (6 * 30 * 24 * 3600));

    println!("🔄 Fetching new data from: {}", last_stored_time);

    // ✅ Fetch only missing data
    fetch_and_store_depth_history(&client, &db, last_stored_time, now).await;
    fetch_and_store_earnings_history(&client, &db, last_stored_time, now).await;
    fetch_and_store_swaps_history(&client, &db, last_stored_time, now).await;
    fetch_and_store_rune_pool_history(&client, &db, last_stored_time, now).await;
}

/// Fetch the latest stored `endTime` from MongoDB to resume fetching efficiently
pub async fn get_last_stored_timestamp(db: &Arc<Database>, collection_name: &str) -> Option<i64> {
    let collection: Collection<mongodb::bson::Document> = db.collection(collection_name);

    // ✅ Find the most recent document sorted by `meta.endTime` in descending order
    let filter = None;
    let sort = doc! { "meta.endTime": -1 };  // Sort in descending order (latest first)
    let find_options = mongodb::options::FindOneOptions::builder().sort(sort).build();

    match collection.find_one(filter, find_options).await {
        Ok(Some(document)) => {
            if let Some(end_time) = document.get("meta").and_then(|meta| {
                meta.as_document()?.get("endTime")?.as_i64()
            }) {
                println!("✅ Last stored `meta.endTime` found: {}", end_time);
                return Some(end_time);
            } else {
                println!("⚠️ `meta.endTime` not found in the latest document.");
            }
        }
        Ok(None) => println!("⚠️ No documents found in `{}`. Fetching from 6 months ago...", collection_name),
        Err(e) => println!("❌ Error fetching `{}` latest timestamp: {:?}", collection_name, e),
    }

    None  // If no records exist, return None (fetch from 6 months ago)
}

/// Fetch and store depth history
async fn fetch_and_store_depth_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
    let collection: Collection<DepthHistoryDocument> = db.collection("depth_history");
    fetch_paginated_data(client, &collection, "depths/BTC.BTC", start_time, end_time).await;
}

/// Fetch and store earnings history
async fn fetch_and_store_earnings_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
    let collection: Collection<EarningsHistoryDocument> = db.collection("earnings_history");
    fetch_paginated_data(client, &collection, "earnings", start_time, end_time).await;
}

/// Fetch and store swaps history
async fn fetch_and_store_swaps_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
    let collection: Collection<SwapsHistoryDocument> = db.collection("swaps_history");
    fetch_paginated_data(client, &collection, "swaps", start_time, end_time).await;
}

/// Fetch and store rune pool history
async fn fetch_and_store_rune_pool_history(client: &Client, db: &Arc<Database>, start_time: i64, end_time: i64) {
    let collection: Collection<RunePoolHistoryDocument> = db.collection("rune_pool_history");
    fetch_paginated_data(client, &collection, "runepool", start_time, end_time).await;
}

/// Fetch paginated data from Midgard and store it in MongoDB
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
                        // println!("🔍 Response from {}:\n {}", endpoint, body);
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
