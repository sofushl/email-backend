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

    let string_list = vec!["0.0.0.0:".to_string(), config.port];
    let joined = string_list.join("");

    let listener = tokio::net::TcpListener::bind(&joined)
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on {:?}", &joined);

    axum::serve(listener, backend)
        .await
        .expect("failed to start server")
}
