use lettre::{
    Message,
    SmtpTransport,
    Transport,
    message::{header, Attachment, SinglePart},
    transport::smtp::authentication::Credentials,
};
use std::env; 

use crate::{
    errors::AppError,
    models::Receipt,
};

#[derive(Clone)]
pub struct EmailService;

impl EmailService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send_receipt_email(&self, recipient_email: &str, receipt: &Receipt, pdf_data: Vec<u8>) -> Result<(), AppError> {
        let filename = format!("receipt_{}.pdf", receipt.id);
        let email_subject = format!("Your Receipt - ID: {}", receipt.id);
        let email_body = format!(
            "Dear User,\n\nPlease find attached your receipt (ID: {}) for the amount of ${:.2}.\n\nTitle: {}\nDescription: {}\nDate: {}\nCategory: {}\n\nThank you!",
            receipt.id,
            receipt.amount,
            receipt.title, // Added Title
            receipt.description.as_deref().unwrap_or("N/A"), // Handle Option<String>
            receipt.date.format("%Y-%m-%d %H:%M:%S UTC"), // Formatted date
            receipt.category.as_deref().unwrap_or("N/A") // Handle Option<String>
        );

        let email = Message::builder()
            .from("noreply@example.com".parse().map_err(|e| AppError::InternalServerError(format!("Invalid sender email format: {}", e)))?)
            .to(recipient_email.parse().map_err(|e| AppError::InternalServerError(format!("Invalid recipient email format: {}", e)))?)
            .subject(email_subject)
            .multipart(
                lettre::message::MultiPart::mixed()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(email_body),
                    )
                    .singlepart(
                        Attachment::new(filename)
                            .body(pdf_data, "application/pdf".parse().unwrap()),
                    ),
            )
            .map_err(|e| AppError::InternalServerError(format!("Failed to build email: {}", e)))?;

        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());
        let smtp_user = env::var("SMTP_USER").unwrap_or_else(|_| "".to_string());
        let smtp_pass = env::var("SMTP_PASS").unwrap_or_else(|_| "".to_string());
        let smtp_port_str = env::var("SMTP_PORT").unwrap_or_else(|_| "1025".to_string());
        let smtp_port: u16 = smtp_port_str.parse().map_err(|_| AppError::InternalServerError(format!("Invalid SMTP_PORT: {}", smtp_port_str)))?;

        let mailer_builder = if smtp_user.is_empty() || smtp_pass.is_empty() {
            SmtpTransport::builder_dangerous(&smtp_host)
                .port(smtp_port)
        } else {
            let creds = Credentials::new(smtp_user, smtp_pass);
            SmtpTransport::relay(&smtp_host)
                .map_err(AppError::LettreSmtpError)?
                .credentials(creds)
                .port(smtp_port)
        };

        let mailer = mailer_builder.build();

        mailer.send(&email).map_err(AppError::LettreSmtpError)?;

        Ok(())
    }
}

