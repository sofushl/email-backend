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

    let localhost = format!("127.0.0.1:{}", &config.port);

    let listener = tokio::net::TcpListener::bind(&localhost)
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on {:?}", &localhost);

    axum::serve(listener, backend)
        .await
        .expect("failed to start server")
}
