use axum::{Router, routing::get};
use crate::db::mongo::connect_to_mongo;
use crate::config::settings::Settings;
use mongodb::{Database, bson::doc};
use tracing::{info, Level};
use tracing_subscriber;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use crate::utils::midgard_fetch::fetch_and_store_data;
use futures::stream::StreamExt;

mod config;
mod db;
mod utils;
#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load settings
    let settings = Settings::new();
    let db_client = connect_to_mongo(&settings.mongo_uri).await;
    let db = db_client.database("midgard-vault");

    // Test Connection: Fetch Database Names
    match db_client.list_database_names(None, None).await {
        Ok(databases) => println!("✅ MongoDB Connected! Databases: {:?}", databases),
        Err(e) => println!("❌ MongoDB Connection Failed: {:?}", e),
    };

    let db = Arc::new(db);

    // Fetch and store initial data (last 6 months)
    fetch_and_store_data(Arc::clone(&db)).await;

    // Verify stored data by fetching the latest entries
    verify_data(Arc::clone(&db)).await;

    // Schedule hourly updates
    let db_clone = Arc::clone(&db);
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            fetch_and_store_data(Arc::clone(&db_clone)).await;
        }
    });
}

/// Verifies data by fetching and printing the last 3 records from each collection
async fn verify_data(db: Arc<Database>) {
    let collections = vec![
        ("depth_history", "Depth History"),
        ("earnings_history", "Earnings History"),
        ("swaps_history", "Swaps History"),
        ("rune_pool_history", "Rune Pool History"),
    ];

    for (collection_name, display_name) in collections {
        let collection = db.collection::<mongodb::bson::Document>(collection_name);
        match collection.find(None, None).await {
            Ok(mut cursor) => {
                let mut count = 0;
                println!("📌 Last 3 records from `{}`:", display_name);
                while let Some(doc) = cursor.next().await {
                    match doc {
                        Ok(document) => {
                            // println!("{:#?}", document);
                            count += 1;
                        }
                        Err(e) => println!("❌ Error reading `{}`: {:?}", display_name, e),
                    }
                    if count >= 3 { break; }
                }
                if count == 0 {
                    println!("❌ No records found in `{}`", display_name);
                }
            }
            Err(e) => println!("❌ Failed to fetch `{}`: {:?}", display_name, e),
        }
    }
}