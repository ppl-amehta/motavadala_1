use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use sqlx::SqlitePool;
use crate::services::{
    user_service::UserService,
    receipt_service::ReceiptService,
    pdf_service::PdfService,
    email_service::EmailService,
};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    // pub password_hash: String, // Should not be exposed in User struct sent to client
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub name: String,
    pub role: String,
}

#[derive(Deserialize, Debug, Validate)] // Added Validate here
pub struct UpdateUserProfilePayload {
    #[validate(email)]
    pub email: Option<String>,
    pub name: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Receipt {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub file_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Validate)] // Added Validate
pub struct NewReceipt {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(range(min = 0.01))]
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Deserialize, Debug, Validate)] // Added Validate
pub struct UpdateReceiptPayload {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    #[validate(range(min = 0.01))]
    pub amount: Option<f64>,
    pub date: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub file_url: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

pub struct AppState {
    pub db_pool: SqlitePool,
    pub user_service: Arc<UserService>,
    pub receipt_service: Arc<ReceiptService>,
    pub pdf_service: Arc<PdfService>,
    pub email_service: Arc<EmailService>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub role: String,
    pub exp: usize,
}

