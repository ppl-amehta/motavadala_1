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
    pub email: String, // This field will store the unique identifier, which can be username or email
    pub name: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct NewUser {
    // If allowing username for login, registration should also reflect this.
    // For now, keeping email validation for new user registration to ensure a valid email is captured if needed for other purposes.
    // Or, we can change this to username and add a separate optional email field.
    // For simplicity, let's assume 'email' field here is the primary identifier (username or email)
    #[validate(length(min = 1))] // Changed from email validation to simple length
    pub email: String, // This will be the username or email
    #[validate(length(min = 8))]
    pub password: String,
    pub name: String,
    pub role: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UpdateUserProfilePayload {
    #[validate(length(min = 1))] // Changed from email validation
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

#[derive(Deserialize, Debug, Validate)]
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

#[derive(Deserialize, Debug, Validate)]
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
    // Removed email validation, field name changed for clarity
    #[validate(length(min = 1))]
    pub username_or_email: String,
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

