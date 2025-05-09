use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    SqlxError(sqlx::Error),
    PasswordHashingError(String),
    JwtError(String),
    AuthError(String),
    NotFound(String),
    InternalServerError(String),
    ValidationError(validator::ValidationErrors),
    LettreError(lettre::error::Error),
    LettreSmtpError(lettre::transport::smtp::Error),
    PdfGenerationError(String),
    // Add other specific error types as needed
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::SqlxError(e) => write!(f, "Database error: {}", e),
            AppError::PasswordHashingError(e) => write!(f, "Password hashing error: {}", e),
            AppError::JwtError(e) => write!(f, "JWT error: {}", e),
            AppError::AuthError(e) => write!(f, "Authentication error: {}", e),
            AppError::NotFound(e) => write!(f, "Not found: {}", e),
            AppError::InternalServerError(e) => write!(f, "Internal server error: {}", e),
            AppError::ValidationError(e) => write!(f, "Validation error: {}", e),
            AppError::LettreError(e) => write!(f, "Email sending error: {}", e),
            AppError::LettreSmtpError(e) => write!(f, "SMTP error: {}", e),
            AppError::PdfGenerationError(e) => write!(f, "PDF generation error: {}", e),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::SqlxError(e) => {
                eprintln!("SQLx error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred".to_string())
            }
            AppError::PasswordHashingError(e) => {
                eprintln!("Password hashing error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred".to_string())
            }
            AppError::JwtError(e) => {
                eprintln!("JWT error: {:?}", e);
                (StatusCode::UNAUTHORIZED, "Invalid or expired token".to_string())
            }
            AppError::AuthError(e) => {
                (StatusCode::UNAUTHORIZED, e)
            }
            AppError::NotFound(e) => {
                (StatusCode::NOT_FOUND, e)
            }
            AppError::InternalServerError(e) => {
                eprintln!("Internal server error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred".to_string())
            }
            AppError::ValidationError(e) => {
                let messages: Vec<String> = e.field_errors().into_iter().map(|(field, errors)| {
                    let field_errors: Vec<String> = errors.iter().map(|err| {
                        format!("{}: {}", field, err.message.as_ref().unwrap_or(&std::borrow::Cow::from("validation failed")))
                    }).collect();
                    field_errors.join(", ")
                }).collect();
                (StatusCode::BAD_REQUEST, messages.join("; "))
            }
            AppError::LettreError(e) => {
                eprintln!("Lettre error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email".to_string())
            }
            AppError::LettreSmtpError(e) => {
                eprintln!("Lettre SMTP error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email due to SMTP issue".to_string())
            }
            AppError::PdfGenerationError(e) => {
                eprintln!("PDF Generation error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate PDF".to_string())
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

// Implement From traits for common error types to simplify error handling in handlers
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::SqlxError(err)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(err)
    }
}

impl From<lettre::error::Error> for AppError {
    fn from(err: lettre::error::Error) -> Self {
        AppError::LettreError(err)
    }
}

impl From<lettre::transport::smtp::Error> for AppError {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        AppError::LettreSmtpError(err)
    }
}

impl From<printpdf::Error> for AppError {
    fn from(err: printpdf::Error) -> Self {
        AppError::PdfGenerationError(format!("{:?}", err))
    }
}

