use axum::{
    extract::{State, Extension},
    http::StatusCode,
    response::{IntoResponse, Json},
};
// Removed unused: async_trait, middleware::Next, http::Request, Response, axum_core::response::Html
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

use crate::{
    auth::{create_auth_cookie, remove_auth_cookie, AuthenticatedUser},
    errors::AppError,
    models::{AppState, LoginRequest, NewUser, UpdateUserProfilePayload},
};

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterUserPayload {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub name: String,
}

#[axum::debug_handler]
pub async fn register_user_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserPayload>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let new_user_model = NewUser {
        email: payload.email,
        password: payload.password,
        name: payload.name,
        role: "user".to_string(),
    };

    let user = state.user_service.create_user(new_user_model).await?;
    Ok((StatusCode::CREATED, Json(user)))
}


#[axum::debug_handler]
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let user = state.user_service.authenticate_user(&payload.username_or_email, &payload.password).await?;
    let token = crate::auth::create_jwt(&user.id.to_string(), &user.role)
        .map_err(|e| AppError::InternalServerError(format!("Failed to create JWT: {}", e)))?;

    let headers = [(axum::http::header::SET_COOKIE, crate::auth::create_auth_cookie(&token).to_string())];
    Ok((StatusCode::OK, headers, Json(user)))
}

#[axum::debug_handler]
pub async fn logout_handler() -> Result<impl IntoResponse, AppError> {
    let cookie = remove_auth_cookie();
    let headers = [(axum::http::header::SET_COOKIE, cookie.to_string())];
    Ok((StatusCode::OK, headers, Json(serde_json::json!({ "message": "Logged out" }))))
}


#[axum::debug_handler]
pub async fn get_user_profile_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.get_user_by_id(&auth_user.id).await?;
    match user {
        Some(u) => Ok(Json(u)),
        None => Err(AppError::NotFound("User not found".to_string())),
    }
}


#[axum::debug_handler]
pub async fn update_user_profile_handler(
    State(state): State<Arc<AppState>>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Json(payload): Json<UpdateUserProfilePayload>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let updated_user = state.user_service.update_user_profile(&auth_user.id, payload).await?;
    Ok(Json(updated_user))
}

