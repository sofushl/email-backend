use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub user: String,
    pub password: String,
    pub server: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            user: env::var("USER").expect("Missing USER"),
            password: env::var("PASSWORD").expect("Missing PASSWORD"),
            server: env::var("SERVER").expect("Missing SERVER"),
            port: env::var("PORT").expect("Missing PORT"),
        }
    }
}
