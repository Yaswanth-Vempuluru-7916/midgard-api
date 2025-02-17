use axum::{Router, routing::get, extract::State};
use std::sync::Arc;
use mongodb::Database;

mod depth_history;
mod earnings_history;
mod swaps_history;
mod runepool_history;

pub fn create_api_router(db: Arc<Database>) -> Router {
    Router::new()
        .route("/api/depth-history", get(depth_history::get_depth_history).with_state(db.clone()))
        .route("/api/earnings-history", get(earnings_history::get_earnings_history).with_state(db.clone()))
        // .route("/api/swaps-history", get(swaps_history::get_swaps_history).with_state(db.clone()))
        .route("/api/rune-pool-history", get(runepool_history::get_rune_pool_history).with_state(db))
}
