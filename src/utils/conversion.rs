use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Converts a string or number to `f64`, logging the value before parsing
pub fn deserialize_string_to_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => {
            // println!("🔍 [DEBUG] Deserializing f64 field: '{}'", s);
            if s.is_empty() || s == "null" {
                Ok(0.0)
            } else {
                s.parse::<f64>().map_err(|e| {
                    // println!("🚨 [ERROR] Failed to parse f64: '{}' | Error: {:?}", s, e);
                    serde::de::Error::custom(format!("Invalid f64: {}", s))
                })
            }
        }
        Value::Number(num) => num.as_f64().ok_or_else(|| {
            // println!("🚨 [ERROR] Expected a valid f64, got: {:?}", num);
            serde::de::Error::custom("Expected a valid f64")
        }),
        _ => {
            // println!("🚨 [ERROR] Unexpected type for f64: {:?}", value);
            Ok(0.0)
        }
    }
}

/// Converts a string or number to `i32`, logging the value before parsing
pub fn deserialize_string_to_number_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => {
            // println!("🔍 [DEBUG] Deserializing i32 field: '{}'", s);
            if s.is_empty() || s == "null" {
                Ok(0)
            } else {
                s.parse::<i32>().map_err(|e| {
                    // println!("🚨 [ERROR] Failed to parse i32: '{}' | Error: {:?}", s, e);
                    serde::de::Error::custom(format!("Invalid i32: {}", s))
                })
            }
        }
        Value::Number(num) => num.as_i64().ok_or_else(|| {
            // println!("🚨 [ERROR] Expected a valid i32, got: {:?}", num);
            serde::de::Error::custom("Expected a valid i32")
        }).map(|x| x as i32),
        _ => {
            // println!("🚨 [ERROR] Unexpected type for i32: {:?}", value);
            Ok(0)
        }
    }
}

/// Converts a string or number to `i64`, logging the value before parsing
pub fn deserialize_string_to_number_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => {
            // println!("🔍 [DEBUG] Deserializing i64 field: '{}'", s);
            if s.is_empty() || s == "null" {
                Ok(0)
            } else {
                s.parse::<i64>().map_err(|e| {
                    // println!("🚨 [ERROR] Failed to parse i64: '{}' | Error: {:?}", s, e);
                    serde::de::Error::custom(format!("Invalid i64: {}", s))
                })
            }
        }
        Value::Number(num) => num.as_i64().ok_or_else(|| {
            // println!("🚨 [ERROR] Expected a valid i64, got: {:?}", num);
            serde::de::Error::custom("Expected a valid i64")
        }),
        _ => {
            // println!("🚨 [ERROR] Unexpected type for i64: {:?}", value);
            Ok(0)
        }
    }
}