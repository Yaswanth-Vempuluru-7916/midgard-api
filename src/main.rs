use axum::{Router, routing::get};
use db::mongo::connect_to_mongo;
use config::settings::Settings;
// use routes::register_routes; // Commented out for now
// use std::sync::Arc;
// use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber;

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

    // let db_client = Arc::new(db_client); // No need for Arc if we are just testing DB

    // Start the server (commented out since we are only testing DB connection)
    // let app = register_routes(db_client.clone());
    // let addr = format!("0.0.0.0:{}", settings.port);
    // let listener = TcpListener::bind(&addr).await.expect("Failed to bind server");
    // info!("🚀 Server running on {}", addr);
    // axum::serve(listener, app).await.unwrap();
}
