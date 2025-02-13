use axum::{Router, routing::get};
use db::{models::{DepthHistoryDocument, DepthHistoryMeta, DepthHistory, 
    EarningsHistoryDocument, EarningsHistoryMeta, EarningsHistory, EarningsPool,
    SwapsHistoryDocument, SwapsHistoryMeta, SwapsHistory,
    RunePoolHistoryDocument, RunePoolHistoryMeta, RunePoolHistory}, 
    mongo::connect_to_mongo};
use config::settings::Settings;
use mongodb::Collection;
use tracing::{info, Level};
use tracing_subscriber;

mod config;
mod db;
mod routes;
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

    // ✅ Collection for Depth History (Now includes Meta and Intervals)
    let collection: Collection<DepthHistoryDocument> = db.collection("depth_history");

    // Sample intervals
    let intervals = vec![
        DepthHistory {
            asset_depth: 385944259888.0,  // f64
            asset_price: 2007.2427797297962,
            asset_price_usd: 2732.9098833099024,
            liquidity_units: 90317181890093.0,  // f64
            members_count: 1645,  // i32
            rune_depth: 774683829038348.0,  // f64
            start_time: 1738627200,  // i64
            end_time: 1738713600,  // i64
            synth_supply: 490373992813.0,  // f64
            synth_units: 157324730559584.0,  // f64
            total_units: 247641912449677.0,  // f64
            luvi: 0.06982330295403419,  // f64
        },
        DepthHistory {
            asset_depth: 375581321665.0,
            asset_price: 2120.4047898324657,
            asset_price_usd: 2787.607366299535,
            liquidity_units: 90311811528026.0,
            members_count: 1645,
            rune_depth: 796384433430074.0,
            start_time: 1738713600,
            end_time: 1738800000,
            synth_supply: 490373992813.0,
            synth_units: 169817833442434.0,
            total_units: 260129644970460.0,
            luvi: 0.06648497747767623,
        }
    ];

    // Sample metadata
    let meta = DepthHistoryMeta {
        end_asset_depth: 398619512182.0,  // f64
        end_lp_units: 90221130719980.0,  // f64
        end_member_count: 1639,  // i32
        end_rune_depth: 752574065265830.0,  // f64
        end_synth_units: 144174446608157.0,  // f64
        end_time: 1739491200,  // i64
        luvi_increase: 1.0240854994847588,  // f64
        price_shift_loss: 0.9999441445210553,  // f64
        start_asset_depth: 393593518870.0,  // f64
        start_lp_units: 90317181890093.0,  // f64
        start_member_count: 1645,  // i32
        start_rune_depth: 758960649366807.0,  // f64
        start_synth_units: 149215803887445.0,  // f64
        start_time: 1738627200,  // i64
    };

    // ✅ Store everything in one document
    let document = DepthHistoryDocument {
        id: None,
        meta,
        intervals,
    };

    let insert_result = collection.insert_one(&document, None).await;

    match insert_result {
        Ok(res) => println!("✅ DepthHistoryDocument Inserted: {:?}", res.inserted_id),
        Err(e) => println!("❌ Failed to Insert DepthHistoryDocument: {:?}", e),
    }

    // ✅ Collection for Earnings History
    let earnings_collection: Collection<EarningsHistoryDocument> = db.collection("earnings_history");

    // Sample earnings pools
    let sample_pools = vec![
        EarningsPool {
            pool: "BTC.BTC".to_string(),
            asset_liquidity_fees: 1250.5,
            rune_liquidity_fees: 2500.75,
            total_liquidity_fees_rune: 3751.25,
            saver_earning: 100.0,
            rewards: 500.0,
            earnings: 4351.25,
        },
        EarningsPool {
            pool: "ETH.ETH".to_string(),
            asset_liquidity_fees: 1000.0,
            rune_liquidity_fees: 2000.0,
            total_liquidity_fees_rune: 3000.0,
            saver_earning: 75.5,
            rewards: 400.0,
            earnings: 3475.5,
        },
    ];

    // Sample earnings intervals
    let earnings_intervals = vec![
        EarningsHistory {
            start_time: 1738627200,
            end_time: 1738713600,
            liquidity_fees: 5000.75,
            block_rewards: 1000.0,
            earnings: 6000.75,
            bonding_earnings: 2000.0,
            liquidity_earnings: 4000.75,
            avg_node_count: 100,
            rune_price_usd: 45.5,
            pools: sample_pools.clone(),
        },
        EarningsHistory {
            start_time: 1738713600,
            end_time: 1738800000,
            liquidity_fees: 5500.25,
            block_rewards: 1100.0,
            earnings: 6600.25,
            bonding_earnings: 2200.0,
            liquidity_earnings: 4400.25,
            avg_node_count: 102,
            rune_price_usd: 46.0,
            pools: sample_pools.clone(),
        },
    ];

    // Earnings metadata
    let earnings_meta = EarningsHistoryMeta {
        start_time: 1738627200,
        end_time: 1738800000,
        liquidity_fees: 10501.0,
        block_rewards: 2100.0,
        earnings: 12601.0,
        bonding_earnings: 4200.0,
        liquidity_earnings: 8401.0,
        avg_node_count: 101,
        rune_price_usd: 45.75,
        pools: sample_pools,
    };

    let earnings_document = EarningsHistoryDocument {
        id: None,
        meta: earnings_meta,
        intervals: earnings_intervals,
    };

    let earnings_insert_result = earnings_collection.insert_one(&earnings_document, None).await;

    match earnings_insert_result {
        Ok(res) => println!("✅ EarningsHistoryDocument Inserted: {:?}", res.inserted_id),
        Err(e) => println!("❌ Failed to Insert EarningsHistoryDocument: {:?}", e),
    }

    // ✅ Collection for Swaps History
    let swaps_collection: Collection<SwapsHistoryDocument> = db.collection("swaps_history");

    // Sample swaps intervals
    let swaps_intervals = vec![
        SwapsHistory {
            start_time: 1738627200,
            end_time: 1738713600,
            to_asset_count: 1000,
            to_rune_count: 1200,
            to_trade_count: 800,
            from_trade_count: 750,
            to_secured_count: 100,
            from_secured_count: 90,
            synth_mint_count: 50,
            synth_redeem_count: 45,
            total_count: 4035,
            to_asset_volume: 1000000.0,
            to_rune_volume: 1200000.0,
            to_trade_volume: 800000.0,
            from_trade_volume: 750000.0,
            to_secured_volume: 100000.0,
            from_secured_volume: 90000.0,
            synth_mint_volume: 50000.0,
            synth_redeem_volume: 45000.0,
            total_volume: 4035000.0,
            rune_price_usd: 45.5,
        },
        SwapsHistory {
            start_time: 1738713600,
            end_time: 1738800000,
            to_asset_count: 1100,
            to_rune_count: 1300,
            to_trade_count: 850,
            from_trade_count: 800,
            to_secured_count: 110,
            from_secured_count: 95,
            synth_mint_count: 55,
            synth_redeem_count: 50,
            total_count: 4360,
            to_asset_volume: 1100000.0,
            to_rune_volume: 1300000.0,
            to_trade_volume: 850000.0,
            from_trade_volume: 800000.0,
            to_secured_volume: 110000.0,
            from_secured_volume: 95000.0,
            synth_mint_volume: 55000.0,
            synth_redeem_volume: 50000.0,
            total_volume: 4360000.0,
            rune_price_usd: 46.0,
        },
    ];

    // Swaps metadata
    let swaps_meta = SwapsHistoryMeta {
        start_time: 1738627200,
        end_time: 1738800000,
        to_asset_count: 2100,
        to_rune_count: 2500,
        to_trade_count: 1650,
        from_trade_count: 1550,
        to_secured_count: 210,
        from_secured_count: 185,
        synth_mint_count: 105,
        synth_redeem_count: 95,
        total_count: 8395,
        to_asset_volume: 2100000.0,
        to_rune_volume: 2500000.0,
        to_trade_volume: 1650000.0,
        from_trade_volume: 1550000.0,
        to_secured_volume: 210000.0,
        from_secured_volume: 185000.0,
        synth_mint_volume: 105000.0,
        synth_redeem_volume: 95000.0,
        total_volume: 8395000.0,
        rune_price_usd: 45.75,
    };

    let swaps_document = SwapsHistoryDocument {
        id: None,
        meta: swaps_meta,
        intervals: swaps_intervals,
    };

    let swaps_insert_result = swaps_collection.insert_one(&swaps_document, None).await;

    match swaps_insert_result {
        Ok(res) => println!("✅ SwapsHistoryDocument Inserted: {:?}", res.inserted_id),
        Err(e) => println!("❌ Failed to Insert SwapsHistoryDocument: {:?}", e),
    }

    // ✅ Collection for RunePool History
    let runepool_collection: Collection<RunePoolHistoryDocument> = db.collection("runepool_history");

    // Sample RunePool intervals
    let runepool_intervals = vec![
        RunePoolHistory {
            start_time: 1738627200,
            end_time: 1738713600,
            depth: 1000000.0,
            count: 500,
            units: 2000000.0,
        },
        RunePoolHistory {
            start_time: 1738713600,
            end_time: 1738800000,
            depth: 1100000.0,
            count: 520,
            units: 2200000.0,
        },
    ];

    // RunePool metadata
    let runepool_meta = RunePoolHistoryMeta {
        start_time: 1738627200,
        end_time: 1738800000,
        start_units: 2000000.0,
        start_count: 500,
        end_units: 2200000.0,
        end_count: 520,
    };

    let runepool_document = RunePoolHistoryDocument {
        id: None,
        meta: runepool_meta,
        intervals: runepool_intervals,
    };

    let runepool_insert_result = runepool_collection.insert_one(&runepool_document, None).await;

    match runepool_insert_result {
        Ok(res) => println!("✅ RunePoolHistoryDocument Inserted: {:?}", res.inserted_id),
        Err(e) => println!("❌ Failed to Insert RunePoolHistoryDocument: {:?}", e),
    }

    // let db_client = Arc::new(db_client); // No need for Arc if we are just testing DB

    // Start the server (commented out since we are only testing DB connection)
    // let app = register_routes(db_client.clone());
    // let addr = format!("0.0.0.0:{}", settings.port);
    // let listener = TcpListener::bind(&addr).await.expect("Failed to bind server");
    // info!("🚀 Server running on {}", addr);
    // axum::serve(listener, app).await.unwrap();
}