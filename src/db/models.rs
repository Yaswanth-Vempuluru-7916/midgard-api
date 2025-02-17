use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::utils::conversion::{deserialize_string_to_number,deserialize_string_to_number_i32,deserialize_string_to_number_i64};

/// Represents a single depth history interval
#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistory {
    #[serde(rename = "assetDepth", deserialize_with = "deserialize_string_to_number")]
    pub asset_depth: f64,

    #[serde(rename = "assetPrice", deserialize_with = "deserialize_string_to_number")]
    pub asset_price: f64,

    #[serde(rename = "assetPriceUSD", deserialize_with = "deserialize_string_to_number")]
    pub asset_price_usd: f64,

    #[serde(rename = "liquidityUnits", deserialize_with = "deserialize_string_to_number")]
    pub liquidity_units: f64,

    #[serde(rename = "membersCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub members_count: i32,  

    #[serde(rename = "runeDepth", deserialize_with = "deserialize_string_to_number")]
    pub rune_depth: f64,

    #[serde(rename = "startTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub start_time: i64, 

    #[serde(rename = "endTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub end_time: i64, 

    #[serde(rename = "synthSupply", deserialize_with = "deserialize_string_to_number")]
    pub synth_supply: f64,

    #[serde(rename = "synthUnits", deserialize_with = "deserialize_string_to_number")]
    pub synth_units: f64,

    #[serde(rename = "units", deserialize_with = "deserialize_string_to_number")]
    pub total_units: f64,

    #[serde(rename = "luvi", deserialize_with = "deserialize_string_to_number")]
    pub luvi: f64,
}

/// Represents metadata for depth history
#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistoryMeta {
    #[serde(rename = "endAssetDepth", deserialize_with = "deserialize_string_to_number")]
    pub end_asset_depth: f64,

    #[serde(rename = "endLPUnits", deserialize_with = "deserialize_string_to_number")]
    pub end_lp_units: f64,

    #[serde(rename = "endMemberCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub end_member_count: i32,  

    #[serde(rename = "endRuneDepth", deserialize_with = "deserialize_string_to_number")]
    pub end_rune_depth: f64,

    #[serde(rename = "endSynthUnits", deserialize_with = "deserialize_string_to_number")]
    pub end_synth_units: f64,

    #[serde(rename = "endTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub end_time: i64, 

    #[serde(rename = "luviIncrease", deserialize_with = "deserialize_string_to_number")]
    pub luvi_increase: f64,

    #[serde(rename = "priceShiftLoss", deserialize_with = "deserialize_string_to_number")]
    pub price_shift_loss: f64,

    #[serde(rename = "startAssetDepth", deserialize_with = "deserialize_string_to_number")]
    pub start_asset_depth: f64,

    #[serde(rename = "startLPUnits", deserialize_with = "deserialize_string_to_number")]
    pub start_lp_units: f64,

    #[serde(rename = "startMemberCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub start_member_count: i32,  

    #[serde(rename = "startRuneDepth", deserialize_with = "deserialize_string_to_number")]
    pub start_rune_depth: f64,

    #[serde(rename = "startSynthUnits", deserialize_with = "deserialize_string_to_number")]
    pub start_synth_units: f64,

    #[serde(rename = "startTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub start_time: i64, 
}

/// **New Structure: Stores `meta` and `intervals` in One Document**
#[derive(Debug, Serialize, Deserialize)]
pub struct DepthHistoryDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "meta")]
    pub meta: DepthHistoryMeta,

    #[serde(rename = "intervals")]
    pub intervals: Vec<DepthHistory>,
}


/// Represents a single earnings history interval
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsHistory {
    #[serde(rename = "startTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub start_time: i64,  

    #[serde(rename = "endTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub end_time: i64,  

    #[serde(rename = "liquidityFees", deserialize_with = "deserialize_string_to_number")]
    pub liquidity_fees: f64,

    #[serde(rename = "blockRewards", deserialize_with = "deserialize_string_to_number")]
    pub block_rewards: f64,

    #[serde(rename = "earnings", deserialize_with = "deserialize_string_to_number")]
    pub earnings: f64,

    #[serde(rename = "bondingEarnings", deserialize_with = "deserialize_string_to_number")]
    pub bonding_earnings: f64,

    #[serde(rename = "liquidityEarnings", deserialize_with = "deserialize_string_to_number")]
    pub liquidity_earnings: f64,

    #[serde(rename = "avgNodeCount", deserialize_with = "deserialize_string_to_number")]
    pub avg_node_count: f64,  

    #[serde(rename = "runePriceUSD", deserialize_with = "deserialize_string_to_number")]
    pub rune_price_usd: f64,

    #[serde(rename = "pools")]
    pub pools: Vec<EarningsPool>,
}

/// Represents earnings per pool in an interval
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct EarningsPool {
    #[serde(rename = "pool")]
    pub pool: String,  // Pool name remains a `String`

    #[serde(rename = "assetLiquidityFees", deserialize_with = "deserialize_string_to_number")]
    pub asset_liquidity_fees: f64,

    #[serde(rename = "runeLiquidityFees", deserialize_with = "deserialize_string_to_number")]
    pub rune_liquidity_fees: f64,

    #[serde(rename = "totalLiquidityFeesRune", deserialize_with = "deserialize_string_to_number")]
    pub total_liquidity_fees_rune: f64,

    #[serde(rename = "saverEarning", deserialize_with = "deserialize_string_to_number")]
    pub saver_earning: f64,

    #[serde(rename = "rewards", deserialize_with = "deserialize_string_to_number")]
    pub rewards: f64,

    #[serde(rename = "earnings", deserialize_with = "deserialize_string_to_number")]
    pub earnings: f64,
}

/// Represents metadata for earnings history
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsHistoryMeta {
    #[serde(rename = "startTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub start_time: i64,  

    #[serde(rename = "endTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub end_time: i64,  

    #[serde(rename = "liquidityFees", deserialize_with = "deserialize_string_to_number")]
    pub liquidity_fees: f64,

    #[serde(rename = "blockRewards", deserialize_with = "deserialize_string_to_number")]
    pub block_rewards: f64,

    #[serde(rename = "earnings", deserialize_with = "deserialize_string_to_number")]
    pub earnings: f64,

    #[serde(rename = "bondingEarnings", deserialize_with = "deserialize_string_to_number")]
    pub bonding_earnings: f64,

    #[serde(rename = "liquidityEarnings", deserialize_with = "deserialize_string_to_number")]
    pub liquidity_earnings: f64,

    #[serde(rename = "avgNodeCount", deserialize_with = "deserialize_string_to_number")]
    pub avg_node_count: f64,  

    #[serde(rename = "runePriceUSD", deserialize_with = "deserialize_string_to_number")]
    pub rune_price_usd: f64,

    #[serde(rename = "pools")]
    pub pools: Vec<EarningsPool>,
}

/// **Final Structure: Stores `meta` and `intervals` in One Document**
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsHistoryDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "meta")]
    pub meta: EarningsHistoryMeta,

    #[serde(rename = "intervals")]
    pub intervals: Vec<EarningsHistory>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SwapsHistory {
    #[serde(rename = "startTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub start_time: i64,  

    #[serde(rename = "endTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub end_time: i64,  

    #[serde(rename = "toAssetCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub to_asset_count: i32,

    #[serde(rename = "toRuneCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub to_rune_count: i32,

    #[serde(rename = "toTradeCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub to_trade_count: i32,

    #[serde(rename = "fromTradeCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub from_trade_count: i32,

    #[serde(rename = "toSecuredCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub to_secured_count: i32,

    #[serde(rename = "fromSecuredCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub from_secured_count: i32,

    #[serde(rename = "synthMintCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub synth_mint_count: i32,

    #[serde(rename = "synthRedeemCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub synth_redeem_count: i32,

    #[serde(rename = "totalCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub total_count: i32,

    #[serde(rename = "toAssetVolume", deserialize_with = "deserialize_string_to_number")]
    pub to_asset_volume: f64,

    #[serde(rename = "toRuneVolume", deserialize_with = "deserialize_string_to_number")]
    pub to_rune_volume: f64,

    #[serde(rename = "toTradeVolume", deserialize_with = "deserialize_string_to_number")]
    pub to_trade_volume: f64,

    #[serde(rename = "fromTradeVolume", deserialize_with = "deserialize_string_to_number")]
    pub from_trade_volume: f64,

    #[serde(rename = "toSecuredVolume", deserialize_with = "deserialize_string_to_number")]
    pub to_secured_volume: f64,

    #[serde(rename = "fromSecuredVolume", deserialize_with = "deserialize_string_to_number")]
    pub from_secured_volume: f64,

    #[serde(rename = "synthMintVolume", deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_volume: f64,

    #[serde(rename = "synthRedeemVolume", deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_volume: f64,

    #[serde(rename = "totalVolume", deserialize_with = "deserialize_string_to_number")]
    pub total_volume: f64,

    #[serde(rename = "runePriceUSD", deserialize_with = "deserialize_string_to_number")]
    pub rune_price_usd: f64,
}

/// Represents metadata for swaps history
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapsHistoryMeta {
    #[serde(rename = "startTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub start_time: i64,

    #[serde(rename = "endTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub end_time: i64,

    #[serde(rename = "toAssetCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub to_asset_count: i32,

    #[serde(rename = "toRuneCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub to_rune_count: i32,

    #[serde(rename = "toTradeCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub to_trade_count: i32,

    #[serde(rename = "fromTradeCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub from_trade_count: i32,

    #[serde(rename = "toSecuredCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub to_secured_count: i32,

    #[serde(rename = "fromSecuredCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub from_secured_count: i32,

    #[serde(rename = "synthMintCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub synth_mint_count: i32,

    #[serde(rename = "synthRedeemCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub synth_redeem_count: i32,

    #[serde(rename = "totalCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub total_count: i32,

    #[serde(rename = "toAssetVolume", deserialize_with = "deserialize_string_to_number")]
    pub to_asset_volume: f64,

    #[serde(rename = "toRuneVolume", deserialize_with = "deserialize_string_to_number")]
    pub to_rune_volume: f64,

    #[serde(rename = "toTradeVolume", deserialize_with = "deserialize_string_to_number")]
    pub to_trade_volume: f64,

    #[serde(rename = "fromTradeVolume", deserialize_with = "deserialize_string_to_number")]
    pub from_trade_volume: f64,

    #[serde(rename = "toSecuredVolume", deserialize_with = "deserialize_string_to_number")]
    pub to_secured_volume: f64,

    #[serde(rename = "fromSecuredVolume", deserialize_with = "deserialize_string_to_number")]
    pub from_secured_volume: f64,

    #[serde(rename = "synthMintVolume", deserialize_with = "deserialize_string_to_number")]
    pub synth_mint_volume: f64,

    #[serde(rename = "synthRedeemVolume", deserialize_with = "deserialize_string_to_number")]
    pub synth_redeem_volume: f64,

    #[serde(rename = "totalVolume", deserialize_with = "deserialize_string_to_number")]
    pub total_volume: f64,

    #[serde(rename = "runePriceUSD", deserialize_with = "deserialize_string_to_number")]
    pub rune_price_usd: f64,
}

/// **Final Structure: Stores `meta` and `intervals` in One Document**
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapsHistoryDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "meta")]
    pub meta: SwapsHistoryMeta,

    #[serde(rename = "intervals")]
    pub intervals: Vec<SwapsHistory>,
}


/// Represents a single RunePool history interval
#[derive(Debug, Serialize, Deserialize)]
pub struct RunePoolHistory {
    #[serde(rename = "startTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub start_time: i64,  

    #[serde(rename = "endTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub end_time: i64,  

    // #[serde(rename = "depth", deserialize_with = "deserialize_string_to_number")]
    // pub depth: f64,

    #[serde(rename = "count", deserialize_with = "deserialize_string_to_number_i32")]
    pub count: i32,  

    #[serde(rename = "units", deserialize_with = "deserialize_string_to_number")]
    pub units: f64,
}

/// Represents metadata for RunePool history
#[derive(Debug, Serialize, Deserialize)]
pub struct RunePoolHistoryMeta {
    #[serde(rename = "startTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub start_time: i64,  

    #[serde(rename = "endTime", deserialize_with = "deserialize_string_to_number_i64")]
    pub end_time: i64,  

    #[serde(rename = "startUnits", deserialize_with = "deserialize_string_to_number")]
    pub start_units: f64,

    #[serde(rename = "startCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub start_count: i32,  

    #[serde(rename = "endUnits", deserialize_with = "deserialize_string_to_number")]
    pub end_units: f64,

    #[serde(rename = "endCount", deserialize_with = "deserialize_string_to_number_i32")]
    pub end_count: i32,  
}

/// **Final Structure: Stores `meta` and `intervals` in One Document**
#[derive(Debug, Serialize, Deserialize)]
pub struct RunePoolHistoryDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(rename = "meta")]
    pub meta: RunePoolHistoryMeta,

    #[serde(rename = "intervals")]
    pub intervals: Vec<RunePoolHistory>,
}
