use dotenvy::dotenv;
use std::env;

#[allow(dead_code)]
pub struct Settings {
    pub mongo_uri: String,
    pub port: u16,
}

impl Settings{
    pub fn new() -> Self {

        dotenv().ok(); // Load .env file

        Self {
            mongo_uri: env::var("MONGO_URI").expect("MONGO_URI to be set"),
            port : env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse() //Converts the string "8080" to a u16 (integer).
            .expect("PORT must be a valid number")            
        }
    }
}