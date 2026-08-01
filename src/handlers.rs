use axum::{Json, response::IntoResponse};
use lettre::Transport;
use lettre::message::Mailbox;
use lettre::{Message, SmtpTransport, transport::smtp::authentication::Credentials};
use serde_json::json;

use crate::config::Config;
use crate::error::ApiError;
use crate::models::EmailRequest;

pub async fn send_email(Json(request): Json<EmailRequest>) -> Result<impl IntoResponse, ApiError> {
    if request.name.trim().is_empty() {
        return Err(ApiError::InvalidInput("name must not be empty".to_string()));
    }
    if request.message.trim().is_empty() {
        return Err(ApiError::InvalidInput(
            "message must not be empty".to_string(),
        ));
    }

    let config = Config::load();

    let reply_to: Mailbox = request
        .email
        .parse()
        .map_err(|_| ApiError::InvalidInput(format!("invalid email address: {}", request.email)))?;

    let from: Mailbox = config.user.parse().map_err(|_| ApiError::InternalError)?;
    let to: Mailbox = "sofushl@proton.me"
        .parse()
        .map_err(|_| ApiError::InternalError)?;

    let mail = Message::builder()
        .from(from)
        .to(to)
        .reply_to(reply_to)
        .subject(request.subject)
        .body(request.message)
        .map_err(|_| ApiError::InternalError)?;

    let creds = Credentials::new(config.user, config.password);

    let mailer = SmtpTransport::relay(&config.server)
        .map_err(|_| ApiError::InternalError)?
        .credentials(creds)
        .build();

    mailer.send(&mail).map_err(|e| {
        eprintln!("Could not send email: {:?}", e);
        ApiError::InternalError
    })?;

    Ok(Json(json!({"status":"sent"})))
}

pub async fn health_check() -> Result<impl IntoResponse, ApiError> {
    Ok(Json(json!({"status":"ok",
        "message":"Server is runing",
    })))
}
