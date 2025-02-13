use  mongodb::{options::ClientOptions,Client};
use tracing::info;

pub async fn connect_to_mongo(mongo_uri:&str)->Client{
    let client_options = ClientOptions::parse(mongo_uri)
    .await
    .expect("Failed to parse MongoDB URI");

    let client = Client::with_options(client_options)
    .expect("Failed to create MongoDB client");

    info!("✅ Connected to MongoDB");
    client
}