use axum::{Router, routing::get};

use crate::handlers::{health_check, send_email};

pub fn create_backend() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/email", get(send_email))
}
