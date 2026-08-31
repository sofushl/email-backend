use axum::{Json, response::IntoResponse};
use lettre::Transport;
use lettre::{Message, SmtpTransport, transport::smtp::authentication::Credentials};
use serde_json::json;

use crate::config::Config;
use crate::error::ApiError;
use crate::models::EmailRequest;

pub async fn send_email(Json(request): Json<EmailRequest>) -> Result<impl IntoResponse, ApiError> {
    println!("Email: {}", request.email);
    println!("Subject: {}", request.subject);
    println!("Message: {}", request.message);

    if request.message.trim().is_empty() {
        return Err(ApiError::InvalidInput(
            "message must not be empty".to_string(),
        ));
    }

    let config = Config::load();

    let mail = Message::builder()
        .from(config.email.parse().unwrap())
        .to(config.receiver.parse().unwrap())
        .subject(request.subject)
        .body(String::from(&request.message))
        .unwrap();

    //let confirmation = Message::builder()
    //    .from(config.email.parse().unwrap())
    //    .to(request.email.parse().unwrap())
    //    .subject("Your email was recieved")
    //    .body(format!(
    //        "Your message: \n {} \n was received",
    //        String::from(&request.message),
    //    ))
    //    .unwrap();

    let creds = Credentials::new(config.email, config.password);

    let mailer = SmtpTransport::relay(&config.server)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&mail) {
        Ok(_) => {
            println!("Email sent successfully!");
            //match mailer.send(&confirmation) {
            //    Ok(_) => {
            //        println!("Confirmation email sent successfully!");
            //    }
            //    Err(e) => {
            //        return Err(ApiError::InternalError(format!(
            //            "Could not send email {}",
            //            e
            //        )));
            //    }
            //}
        }
        Err(e) => {
            return Err(ApiError::InternalError(format!(
                "Could not send email {}",
                e
            )));
        }
    }

    Ok(Json(json!({"status":"sent"})))
}

pub async fn health_check() -> Result<impl IntoResponse, ApiError> {
    Ok(Json(json!({"status":"ok",
        "message":"Server is runing",
    })))
}
