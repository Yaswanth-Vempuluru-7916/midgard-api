use mongodb::bson::oid::ObjectId;
use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct DepthHistory{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id : Option<ObjectId>,
    pub asset_depth : String,
    pub asset_price : f64,
    pub asset_price_usd : f64,
    pub liquidity_units : String,
    pub members_count : i32,
    pub rune_depth : String,
    pub start_time  : i64,
    pub end_time  : i64,
}