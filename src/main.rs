use crate::db::mongo::connect_to_mongo;
use crate::config::settings::Settings;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::api::create_api_router; // Import API Router
use crate::utils::midgard_fetch::fetch_and_store_data; // Import the function to fetch and store data
use tracing_subscriber;
use tracing:: Level;

mod config;
mod db;
mod utils;
mod api;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)//you’ll only write down important updates—like "Server is running" or "Connected to the database"
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

    // ✅ Create API Router
    let app = create_api_router(Arc::clone(&db));

    // Start the scheduled job to fetch data every hour
    tokio::spawn({
        let db = Arc::clone(&db);
        async move {
            let mut interval = interval(Duration::from_secs(3600)); // Set the interval to 1 hour
            loop {
                interval.tick().await;
                println!("🔄 Fetching fresh data...");
                fetch_and_store_data(Arc::clone(&db)).await;
            }
        }
    });

    // ✅ Start Server
    let addr = format!("0.0.0.0:{}", settings.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server running on http://localhost:{}", settings.port);
    axum::serve(listener, app).await.unwrap();
}
