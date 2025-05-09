use tower_http::cors::{CorsLayer, Any};
use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware::{self, from_fn_with_state}, // Ensure from_fn_with_state is correctly imported
    response::IntoResponse,
    Json,
    http::StatusCode,
    extract::DefaultBodyLimit,
};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::net::TcpListener; // Added TcpListener import
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::fs;
use std::str::FromStr;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;

mod errors;
mod auth;
mod models;
mod handlers;
mod services;

use crate::{
    auth::{require_auth, require_admin_auth},
    handlers::{
        auth_handler::{register_user_handler, login_handler, logout_handler},
        user_handler::{get_user_profile_handler, update_user_profile_handler},
        receipt_handler::*,
        admin::{
            user_handler::get_all_users_handler as admin_get_all_users,
            receipt_handler::get_all_receipts_admin_handler as admin_get_all_receipts,
            report_handler::get_admin_reports_handler as admin_get_reports,
        }
    },
    models::AppState,
    services::{
        user_service::UserService,
        receipt_service::ReceiptService,
        pdf_service::PdfService,
        email_service::EmailService,
    }
};

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "Healthy" })))
}

async fn seed_initial_users(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Seeding initial users...");

    // Admin User
    let admin_email = "Dipadmin";
    let admin_exists: Option<i32> = sqlx::query_scalar("SELECT 1 FROM users WHERE email = ?")
        .bind(admin_email)
        .fetch_optional(pool)
        .await?;

    if admin_exists.is_none() {
        let admin_id = Uuid::new_v4().to_string();
        let admin_password = "mv1962";
        let hashed_admin_password = hash(admin_password, DEFAULT_COST)
            .map_err(|e| sqlx::Error::Configuration(e.into()))?;
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            admin_id,
            admin_email,
            "Admin User", // Default name for admin
            hashed_admin_password,
            "admin",
            now,
            now
        )
        .execute(pool)
        .await?;
        info!("Admin user seeded: {}", admin_email);
    } else {
        info!("Admin user {} already exists.", admin_email);
    }

    // Regular User
    let user_email = "GVora";
    let user_exists: Option<i32> = sqlx::query_scalar("SELECT 1 FROM users WHERE email = ?")
        .bind(user_email)
        .fetch_optional(pool)
        .await?;

    if user_exists.is_none() {
        let user_id = Uuid::new_v4().to_string();
        let user_password = "12345678";
        let hashed_user_password = hash(user_password, DEFAULT_COST)
            .map_err(|e| sqlx::Error::Configuration(e.into()))?;
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            user_id,
            user_email,
            "User GVora", // Default name for user
            hashed_user_password,
            "user",
            now,
            now
        )
        .execute(pool)
        .await?;
        info!("Regular user seeded: {}", user_email);
    } else {
        info!("Regular user {} already exists.", user_email);
    }

    info!("Initial user seeding completed.");
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=debug,receipt_management_serverless=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:/home/ubuntu/receipt_management_serverless/receipt_app.db".to_string());
    info!("Connecting to database: {}", database_url);

    let db_path = database_url.strip_prefix("sqlite:").unwrap_or(&database_url);
    if let Some(parent_dir) = std::path::Path::new(db_path).parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let connect_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true);

    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;
    info!("Database connection established.");

    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    info!("Database migrations completed.");

    // Seed initial users after migrations
    seed_initial_users(&pool).await.expect("Failed to seed initial users");

    let user_service = Arc::new(UserService::new(pool.clone()));
    let receipt_service = Arc::new(ReceiptService::new(pool.clone()));
    let pdf_service = Arc::new(PdfService::new());
    let email_service = Arc::new(EmailService::new());

    let app_state = Arc::new(AppState {
        db_pool: pool.clone(),
        user_service,
        receipt_service,
        pdf_service,
        email_service,
    });

    let auth_router = Router::new()
        .route("/register", post(register_user_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler));

    let user_router = Router::new()
        .route("/profile", get(get_user_profile_handler).put(update_user_profile_handler))
        .layer(middleware::from_fn_with_state(app_state.clone(), require_auth));

    let receipt_router = Router::new()
        .route("/", post(create_receipt_handler).get(get_user_receipts_handler))
        .route("/:receipt_id", get(get_receipt_handler).put(update_receipt_handler).delete(delete_receipt_handler))
        .route("/:receipt_id/pdf", get(generate_receipt_pdf_handler))
        .route("/:receipt_id/email", post(email_receipt_handler))
        .layer(middleware::from_fn_with_state(app_state.clone(), require_auth));

    let admin_router = Router::new()
        .route("/users", get(admin_get_all_users))
        .route("/receipts", get(admin_get_all_receipts))
        .route("/reports", get(admin_get_reports))
        .layer(middleware::from_fn_with_state(app_state.clone(), require_admin_auth));

    let api_router = Router::new()
        .nest("/auth", auth_router)
        .nest("/users", user_router)
        .nest("/receipts", receipt_router)
        .nest("/admin", admin_router)
        .route("/health", get(health_check))
        .with_state(app_state.clone())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024));

    fs::create_dir_all("static").unwrap_or_else(|e| info!("Failed to create static dir or it exists: {}", e));

    let app = Router::new()
    .layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    )
        .nest("/api", api_router)
        .fallback_service(ServeDir::new("static").not_found_service(get(health_check)))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

