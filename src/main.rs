mod config;
mod error;
mod handlers;
mod models;
mod routes;

use dotenvy::dotenv;

use crate::config::Config;
use crate::routes::create_backend;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::load();

    let backend = create_backend();

    let listener = tokio::net::TcpListener::bind(&config.domain)
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on {:?}", &config.domain);

    axum::serve(listener, backend)
        .await
        .expect("failed to start server")
}
