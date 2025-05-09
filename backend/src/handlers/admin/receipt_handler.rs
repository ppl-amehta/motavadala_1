use axum::{
    extract::State,
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use std::sync::Arc;
use crate::{
    models::AppState,
    errors::AppError,
};

#[axum::debug_handler]
pub async fn get_all_receipts_admin_handler(State(_state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    // In a real application, you would fetch all receipts from the receipt_service
    // For now, we'll return a placeholder response
    let receipts_placeholder = vec![
        serde_json::json!({ "id": "admin_receipt_1", "amount": 100.0, "description": "Admin Sample Receipt 1" }),
        serde_json::json!({ "id": "admin_receipt_2", "amount": 200.0, "description": "Admin Sample Receipt 2" }),
    ];
    Ok((StatusCode::OK, Json(receipts_placeholder)))
}

