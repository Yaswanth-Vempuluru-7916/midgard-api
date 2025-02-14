use axum::Router;
use crate::db::mongo::connect_to_mongo;
use crate::config::settings::Settings;
use mongodb::{Database, bson::doc};
use tracing::{info, Level};
use tracing_subscriber;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use crate::utils::midgard_fetch::fetch_and_store_data;
use futures::stream::StreamExt;
use tokio::net::TcpListener;
use crate::api::create_api_router; // ✅ Import API Router

mod config;
mod db;
mod utils;
mod api; // ✅ Register API module

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load settings
    let settings = Settings::new();
    let db_client = connect_to_mongo(&settings.mongo_uri).await;
    let db = Arc::new(db_client.database("midgard-vault"));

    // Test Connection: Fetch Database Names
    match db_client.list_database_names(None, None).await {
        Ok(databases) => println!("✅ MongoDB Connected! Databases: {:?}", databases),
        Err(e) => println!("❌ MongoDB Connection Failed: {:?}", e),
    };

    // // 🔴 Commented out: Fetching & storing initial data
    // fetch_and_store_data(Arc::clone(&db)).await;

    // // 🔴 Commented out: Verifying stored data
    // verify_data(Arc::clone(&db)).await;

    // ✅ Create API Router
    let app = create_api_router(Arc::clone(&db));

    // ✅ Start Server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
