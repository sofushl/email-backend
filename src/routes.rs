use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::{health_check, send_email};

pub fn create_backend() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/email", post(send_email))
}
