use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailRequest {
    pub email: String,
    pub subject: String,
    pub message: String,
}
