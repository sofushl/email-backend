use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub email: String,
    pub receiver: String,
    pub password: String,
    pub server: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            email: env::var("EMAIL").expect("Missing EMAIL"),
            receiver: env::var("RECEIVER").expect("Missing RECEIVER"),
            password: env::var("PASSWORD").expect("Missing PASSWORD"),
            server: env::var("SERVER").expect("Missing SERVER"),
            port: env::var("PORT").expect("Missing PORT"),
        }
    }
}
