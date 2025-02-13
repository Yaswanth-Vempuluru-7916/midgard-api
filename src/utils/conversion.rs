use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Converts a string or number to `f64`
pub fn deserialize_string_to_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
        Value::Number(num) => num.as_f64().ok_or(serde::de::Error::custom("Expected a valid f64")),
        _ => Err(serde::de::Error::custom("Expected a string or number")),
    }
}

/// Converts a string or number to `i32`
pub fn deserialize_string_to_number_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => s.parse::<i32>().map_err(serde::de::Error::custom),
        Value::Number(num) => num.as_i64().ok_or(serde::de::Error::custom("Expected a valid i32")).map(|x| x as i32),
        _ => Err(serde::de::Error::custom("Expected a string or number")),
    }
}

/// Converts a string or number to `i64`
pub fn deserialize_string_to_number_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => s.parse::<i64>().map_err(serde::de::Error::custom),
        Value::Number(num) => num.as_i64().ok_or(serde::de::Error::custom("Expected a valid i64")),
        _ => Err(serde::de::Error::custom("Expected a string or number")),
    }
}
