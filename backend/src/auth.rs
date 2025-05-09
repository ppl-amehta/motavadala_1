use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
    http::header::{HeaderMap, AUTHORIZATION, COOKIE}, // Import HeaderMap, AUTHORIZATION, and COOKIE from http::header
    middleware::Next,
    response::Response,
    body::Body,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::Duration;

use crate::{
    errors::AppError,
    models::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
    pub role: String,
}

const JWT_SECRET: &[u8] = b"your-very-secret-key-that-is-long-and-secure";

pub fn create_jwt(user_id: &str, role: &str) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(ChronoDuration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_owned(),
        exp: expiration as usize,
    };
    let header = Header::default();
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|e| AppError::JwtError(format!("Failed to create token: {}", e)))
}

fn extract_token_from_headers(headers: &HeaderMap) -> Result<String, AppError> {
    // Check Authorization header first
    if let Some(auth_header) = headers.get(AUTHORIZATION) { // Use http::header::AUTHORIZATION
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str[7..].trim();
                if !token.is_empty() {
                    return Ok(token.to_string());
                }
            }
        }
    }
    // Then check Cookie header
    if let Some(cookie_header_val) = headers.get(COOKIE) { // Use http::header::COOKIE
        if let Ok(cookie_str) = cookie_header_val.to_str() {
            for cookie_pair in cookie_str.split(";") {
                let mut parts = cookie_pair.trim().splitn(2, "=");
                if let (Some(name), Some(value)) = (parts.next(), parts.next()) {
                    if name == "token" {
                        let token = value.trim();
                        if !token.is_empty() {
                            return Ok(token.to_string());
                        }
                    }
                }
            }
        }
    }
    Err(AppError::AuthError("Missing or invalid authentication token".to_string()))
}

pub async fn require_auth(
    State(_state): State<Arc<AppState>>,
    mut req: axum::http::Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let headers = req.headers();
    let token_str = extract_token_from_headers(headers)?;
    let decoding_key = DecodingKey::from_secret(JWT_SECRET);
    let validation = Validation::default();
    match decode::<Claims>(&token_str, &decoding_key, &validation) {
        Ok(token_data) => {
            let claims = token_data.claims;
            let authenticated_user = AuthenticatedUser {
                id: claims.sub.clone(),
                role: claims.role.clone(),
            };
            req.extensions_mut().insert(authenticated_user);
            Ok(next.run(req).await)
        }
        Err(e) => Err(AppError::JwtError(format!("Invalid token: {}. Token: {}", e, token_str))),
    }
}

pub async fn require_admin_auth(
    State(_state): State<Arc<AppState>>,
    mut req: axum::http::Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let headers = req.headers();
    let token_str = extract_token_from_headers(headers)?;
    let decoding_key = DecodingKey::from_secret(JWT_SECRET);
    let validation = Validation::default();
    match decode::<Claims>(&token_str, &decoding_key, &validation) {
        Ok(token_data) => {
            let claims = token_data.claims;
            if claims.role != "admin" {
                return Err(AppError::AuthError("Admin privileges required".to_string()));
            }
            let authenticated_user = AuthenticatedUser {
                id: claims.sub.clone(),
                role: claims.role.clone(),
            };
            req.extensions_mut().insert(authenticated_user);
            Ok(next.run(req).await)
        }
        Err(e) => Err(AppError::JwtError(format!("Invalid token: {}", e))),
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or_else(|| AppError::AuthError("User not authenticated. Ensure require_auth middleware is applied.".to_string()))
    }
}

pub fn create_auth_cookie(token: &str) -> axum_extra::extract::cookie::Cookie<
'static
> {
    axum_extra::extract::cookie::Cookie::build(("token", token.to_owned()))
        .path("/")
        .http_only(true)
        .secure(true) 
        .same_site(SameSite::Lax)
        .max_age(Duration::hours(24))
        .build()
}

pub fn remove_auth_cookie() -> axum_extra::extract::cookie::Cookie<
'static
> {
    axum_extra::extract::cookie::Cookie::build(("token", ""))
        .path("/")
        .http_only(true)
        .secure(true) 
        .max_age(Duration::ZERO)
        .same_site(SameSite::Lax)
        .build()
}

