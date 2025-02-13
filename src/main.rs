use axum::{Router, routing::get};
use db::mongo::connect_to_mongo;
use config::settings::Settings;
use mongodb::Collection;
// use routes::register_routes; // Commented out for now
// use std::sync::Arc;
// use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber;

use db::models::DepthHistory;
// use mongodb::bson::doc;

mod config;
mod db;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Load settings
    let settings = Settings::new();
    let db_client = connect_to_mongo(&settings.mongo_uri).await;

    // 🔥 Test Connection: Fetch Database Names
    match db_client.list_database_names(None, None).await {
        Ok(databases) => {
            println!("✅ MongoDB Connected! Databases: {:?}", databases);
        }
        Err(e) => {
            println!("❌ MongoDB Connection Failed: {:?}", e);
        }
    };

    let db = db_client.database("midgard-vault");
    let collection = db.collection("depth_history");

    let test_data = DepthHistory {
        id: None,
        asset_depth: "100000".to_string(),
        asset_price: 2000.50,
        asset_price_usd: 2700.75,
        liquidity_units: "500000".to_string(),
        members_count: 1500,
        rune_depth: "750000".to_string(),
        start_time: 1738627200,
        end_time: 1738713600,
    };

    let insert_result = collection.insert_one(test_data, None).await;
    
    match insert_result {
        Ok(res) => println!("✅ Test Data Inserted: {:?}", res.inserted_id),
        Err(e) => println!("❌ Failed to Insert Data: {:?}", e),
    }



    // let db_client = Arc::new(db_client); // No need for Arc if we are just testing DB

    // Start the server (commented out since we are only testing DB connection)
    // let app = register_routes(db_client.clone());
    // let addr = format!("0.0.0.0:{}", settings.port);
    // let listener = TcpListener::bind(&addr).await.expect("Failed to bind server");
    // info!("🚀 Server running on {}", addr);
    // axum::serve(listener, app).await.unwrap();
}
