use axum::{extract::{State, Extension}, response::IntoResponse, Json, http::StatusCode};
use std::sync::Arc;
use validator::Validate;

use crate::{
    auth::AuthenticatedUser,
    errors::AppError,
    models::{AppState, UpdateUserProfilePayload}, // Changed NewUser to UpdateUserProfilePayload
};

// Get current user's profile
#[axum::debug_handler]
pub async fn get_user_profile_handler(State(state): State<Arc<AppState>>, Extension(auth_user): Extension<AuthenticatedUser>) -> Result<impl IntoResponse, AppError> {
    // Corrected method name to get_user_by_id
    let user_option = state.user_service.get_user_by_id(&auth_user.id).await?;
    match user_option {
        Some(u) => Ok((StatusCode::OK, Json(u))),
        None => Err(AppError::NotFound("User profile not found".to_string())),
    }
}

// Update current user's profile
#[axum::debug_handler]
pub async fn update_user_profile_handler(State(state): State<Arc<AppState>>, Extension(auth_user): Extension<AuthenticatedUser>, Json(payload): Json<UpdateUserProfilePayload>) -> Result<impl IntoResponse, AppError> {
    // Validate the payload
    payload.validate()?;
    // Corrected method name to update_user_profile and ensure auth_user.id is used
    let updated_user = state.user_service.update_user_profile(&auth_user.id, payload).await?;
    Ok((StatusCode::OK, Json(updated_user)))
}

