use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::{User, NewUser, UpdateUserProfilePayload};
use crate::errors::AppError;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, DateTime, NaiveDateTime};

pub struct UserService {
    db_pool: SqlitePool,
}

impl UserService {
    pub fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }

    pub async fn create_user(&self, new_user: NewUser) -> Result<User, AppError> {
        let user_id = Uuid::new_v4().to_string();
        let hashed_password = hash(&new_user.password, DEFAULT_COST)
            .map_err(|e| AppError::PasswordHashingError(format!("Failed to hash password: {}", e)))?;
        
        let now = Utc::now();
        sqlx::query!(
            "INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            user_id,
            new_user.email,
            new_user.name,
            hashed_password,
            new_user.role,
            now, // Add created_at
            now  // Add updated_at
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    return AppError::AuthError("Email already exists".to_string());
                }
            }
            AppError::SqlxError(e)
        })?;

        self.get_user_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to retrieve created user".to_string()))
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<User, AppError> {
        // sqlx::query! returns a record with NaiveDateTime, which needs conversion.
        let user_record = sqlx::query!(
            "SELECT id, email, name, password_hash, role, created_at, updated_at FROM users WHERE email = ?",
            email
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(AppError::SqlxError)?
        .ok_or(AppError::AuthError("Invalid credentials".to_string()))?;

        if verify(password, &user_record.password_hash).unwrap_or(false) { 
            Ok(User {
                id: user_record.id,
                email: user_record.email,
                name: user_record.name,
                role: user_record.role,
                created_at: DateTime::<Utc>::from_naive_utc_and_offset(user_record.created_at, Utc), // Explicit conversion
                updated_at: DateTime::<Utc>::from_naive_utc_and_offset(user_record.updated_at, Utc), // Explicit conversion
            })
        } else {
            Err(AppError::AuthError("Invalid credentials".to_string()))
        }
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as!(
            User,
            r#"SELECT id, email, name, role, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM users WHERE id = ?"#,
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(AppError::SqlxError)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        sqlx::query_as!(User, r#"SELECT id, email, name, role, created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM users"#)
            .fetch_all(&self.db_pool)
            .await
            .map_err(AppError::SqlxError)
    }

    pub async fn update_user_profile(&self, user_id: &str, payload: UpdateUserProfilePayload) -> Result<User, AppError> {
        let mut current_user = self.get_user_by_id(user_id).await?.ok_or(AppError::NotFound("User not found".to_string()))?;

        let mut changed = false;
        if let Some(email) = payload.email {
            if email != current_user.email {
                current_user.email = email;
                changed = true;
            }
        }
        if let Some(name) = payload.name {
             if name != current_user.name {
                current_user.name = name;
                changed = true;
            }
        }

        if changed {
            let now_utc = Utc::now();
            current_user.updated_at = now_utc;
            sqlx::query!(
                "UPDATE users SET email = ?, name = ?, updated_at = ? WHERE id = ?",
                current_user.email,
                current_user.name,
                now_utc, 
                user_id
            )
            .execute(&self.db_pool)
            .await
            .map_err(AppError::SqlxError)?;
        }
        
        Ok(current_user)
    }
}

