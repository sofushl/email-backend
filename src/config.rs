use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub email: String,
    pub password: String,
    pub server: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            email: env::var("EMAIL").expect("Missing EMAIL"),
            password: env::var("PASSWORD").expect("Missing PASSWORD"),
            server: env::var("SERVER").expect("Missing SERVER"),
            port: env::var("PORT").expect("Missing PORT"),
        }
    }
}
