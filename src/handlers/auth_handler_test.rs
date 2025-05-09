#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AppState, LoginRequest, User};
    use crate::auth;
    use std::sync::Arc;
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
        routing::post,
        Router,
    };
    use tower::ServiceExt; // for `oneshot` and `ready`
    use sqlx::SqlitePool;
    use chrono::Utc;

    // Mock AppState for testing
    async fn setup_app_state() -> Arc<AppState> {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        // Initialize schema if necessary (assuming migrations are handled elsewhere or not needed for this specific test)
        // For simplicity, we'll assume the user table exists or create it manually here if needed.
        // This is often done via a test setup function that runs migrations.
        
        Arc::new(AppState {
            db_pool: pool,
            // Mock other services if needed, or use real ones if they don't have external dependencies
        })
    }

    #[tokio::test]
    async fn test_login_handler_success() {
        let app_state = setup_app_state().await;

        // You would typically seed a user into the database for testing login
        // For this example, we'll assume the `authenticate_user` service function is mocked or works with an in-memory store.
        // Let's refine this to actually insert a user for a more realistic test.
        let user_id = "test_user_id".to_string();
        let user_email = "test@example.com".to_string();
        let user_name = "Test User".to_string();
        let user_role = "user".to_string();
        let password = "password123";
        let hashed_password = auth::hash_password(password).expect("Failed to hash password");

        sqlx::query!(
            "INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            user_id,
            user_email,
            user_name,
            hashed_password,
            user_role,
            Utc::now(),
            Utc::now()
        )
        .execute(&app_state.db_pool)
        .await
        .expect("Failed to insert test user");

        let login_request = LoginRequest {
            username_or_email: user_email.clone(), // Use the seeded user's email
            password: password.to_string(),
        };

        let app = Router::new().route("/login", post(login_handler)).with_state(app_state);

        let response = app
            .oneshot(Request::builder()
                .method(Method::POST)
                .uri("/login")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&login_request).unwrap()))
                .unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(body_json["message"], "Login successful");
        assert_eq!(body_json["user"]["email"], user_email);
        assert!(body_json["token"].is_string());
    }

    #[tokio::test]
    async fn test_login_handler_failure_wrong_password() {
        let app_state = setup_app_state().await;

        // Seed a user
        let user_id = "test_user_wrong_pass_id".to_string();
        let user_email = "wrongpass@example.com".to_string();
        let user_name = "Wrong Pass User".to_string();
        let user_role = "user".to_string();
        let correct_password = "password123";
        let hashed_password = auth::hash_password(correct_password).expect("Failed to hash password");

        sqlx::query!(
            "INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            user_id,
            user_email.clone(),
            user_name,
            hashed_password,
            user_role,
            Utc::now(),
            Utc::now()
        )
        .execute(&app_state.db_pool)
        .await
        .expect("Failed to insert test user for wrong password test");

        let login_request = LoginRequest {
            username_or_email: user_email.clone(),
            password: "wrongpassword".to_string(), // Incorrect password
        };

        let app = Router::new().route("/login", post(login_handler)).with_state(app_state);

        let response = app
            .oneshot(Request::builder()
                .method(Method::POST)
                .uri("/login")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&login_request).unwrap()))
                .unwrap())
            .await
            .unwrap();
        
        // Assuming your AppError::InvalidCredentials maps to a 401 Unauthorized status
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_json["message"], "Invalid credentials"); // Or whatever your error message is
    }

    #[tokio::test]
    async fn test_login_handler_user_not_found() {
        let app_state = setup_app_state().await;
        // No user seeded for this test case

        let login_request = LoginRequest {
            username_or_email: "nonexistent@example.com".to_string(),
            password: "anypassword".to_string(),
        };

        let app = Router::new().route("/login", post(login_handler)).with_state(app_state);

        let response = app
            .oneshot(Request::builder()
                .method(Method::POST)
                .uri("/login")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&login_request).unwrap()))
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED); // Or appropriate error code for user not found

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_json["message"], "User not found"); // Or whatever your error message is
    }

    // Add more tests for validation errors (e.g., empty username/password) if needed.
}

