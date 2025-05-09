use axum::{extract::{State, Path, Query, Extension}, response::{IntoResponse, AppendHeaders}, Json, http::{StatusCode, header::HeaderValue}};
use std::sync::Arc;
use serde::Deserialize;
use validator::Validate;

use crate::{
    auth::AuthenticatedUser,
    errors::AppError,
    models::{AppState, User, NewReceipt, UpdateReceiptPayload, Receipt},
};

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

// Create a new receipt
#[axum::debug_handler]
pub async fn create_receipt_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Json(payload): Json<NewReceipt>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let receipt = state.receipt_service.create_receipt(auth_user.id.clone(), payload).await?;
    Ok((StatusCode::CREATED, Json(receipt)))
}

// Get all receipts for the authenticated user (paginated)
#[axum::debug_handler]
pub async fn get_user_receipts_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Query(pagination): Query<Pagination>,
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1).max(1);
    let per_page = pagination.per_page.unwrap_or(10).max(1);
    let receipts = state.receipt_service.get_receipts_by_user_id(&auth_user.id, page, per_page).await?;
    Ok((StatusCode::OK, Json(receipts)))
}

// Get a specific receipt by ID
#[axum::debug_handler]
pub async fn get_receipt_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(receipt_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let receipt_option = state.receipt_service.get_receipt_by_id(&receipt_id).await?;
    match receipt_option {
        Some(r) => {
            if r.user_id == auth_user.id || auth_user.role == "admin" {
                Ok((StatusCode::OK, Json(r)))
            } else {
                Err(AppError::AuthError("You are not authorized to view this receipt".to_string()))
            }
        }
        None => Err(AppError::NotFound("Receipt not found".to_string())),
    }
}

// Update a specific receipt by ID
#[axum::debug_handler]
pub async fn update_receipt_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(receipt_id): Path<String>,
    Json(payload): Json<UpdateReceiptPayload>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let updated_receipt = state.receipt_service.update_receipt(&receipt_id, &auth_user.id, payload).await?;
    Ok((StatusCode::OK, Json(updated_receipt)))
}

// Delete a specific receipt by ID
#[axum::debug_handler]
pub async fn delete_receipt_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(receipt_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    state.receipt_service.delete_receipt(&receipt_id, &auth_user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Handler for generating PDF receipt
#[axum::debug_handler]
pub async fn generate_receipt_pdf_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(receipt_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let receipt_option: Option<Receipt> = state.receipt_service.get_receipt_by_id(&receipt_id).await?;
    match receipt_option {
        Some(r) => {
            if r.user_id != auth_user.id && auth_user.role != "admin" {
                return Err(AppError::AuthError("You are not authorized to generate PDF for this receipt".to_string()));
            }
            let pdf_data = state.pdf_service.generate_receipt_pdf(&r).await?;
            let content_disposition_str = format!("attachment; filename=\"receipt_{}.pdf\"", receipt_id);
            let content_disposition_val = HeaderValue::from_str(&content_disposition_str)
                .map_err(|_| AppError::InternalServerError("Failed to create Content-Disposition header".to_string()))?;
            
            let headers = AppendHeaders([
                (axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/pdf")),
                (axum::http::header::CONTENT_DISPOSITION, content_disposition_val),
            ]);
            Ok((StatusCode::OK, headers, pdf_data))
        }
        None => Err(AppError::NotFound("Receipt not found".to_string())),
    }
}

// Handler for emailing receipt
#[axum::debug_handler]
pub async fn email_receipt_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(receipt_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let receipt_option: Option<Receipt> = state.receipt_service.get_receipt_by_id(&receipt_id).await?;
    let user_option: Option<User> = state.user_service.get_user_by_id(&auth_user.id).await?;
    let user = user_option.ok_or_else(|| AppError::NotFound("Authenticated user not found for email".to_string()))?;

    match receipt_option {
        Some(r) => {
            if r.user_id != auth_user.id && auth_user.role != "admin" {
                return Err(AppError::AuthError("You are not authorized to email this receipt".to_string()));
            }
            let pdf_data = state.pdf_service.generate_receipt_pdf(&r).await?;
            state.email_service.send_receipt_email(&user.email, &r, pdf_data).await?;
            Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Receipt emailed successfully" }))))
        }
        None => Err(AppError::NotFound("Receipt not found".to_string())),
    }
}

