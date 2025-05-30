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
        // The 'email' field in NewUser now holds username_or_email
        sqlx::query!(
            "INSERT INTO users (id, email, name, password_hash, role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            user_id,
            new_user.email, // This is username_or_email
            new_user.name,
            hashed_password,
            new_user.role,
            now, 
            now  
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    // This error message might need to be more generic if 'email' can be a username
                    return AppError::AuthError("Username or email already exists".to_string());
                }
            }
            AppError::SqlxError(e)
        })?;

        self.get_user_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to retrieve created user".to_string()))
    }

    // Updated to accept username_or_email
    pub async fn authenticate_user(&self, username_or_email: &str, password: &str) -> Result<User, AppError> {
        // The 'email' column in the database now stores username_or_email
        let user_record = sqlx::query!(
            "SELECT id, email, name, password_hash, role, created_at, updated_at FROM users WHERE email = ?",
            username_or_email
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(AppError::SqlxError)?
        .ok_or(AppError::AuthError("Invalid credentials (user not found)".to_string()))?;

        let valid_password = verify(password, &user_record.password_hash)
            .map_err(|e| AppError::PasswordHashingError(format!("Password verification failed: {}", e)))?;

        if valid_password {
            Ok(User {
                id: user_record.id,
                email: user_record.email, // This field in User struct now holds username_or_email
                name: user_record.name,
                role: user_record.role,
                created_at: DateTime::<Utc>::from_naive_utc_and_offset(user_record.created_at, Utc),
                updated_at: DateTime::<Utc>::from_naive_utc_and_offset(user_record.updated_at, Utc),
            })
        } else {
            Err(AppError::AuthError("Invalid credentials (password mismatch)".to_string()))
        }
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as!(
            User,
            // The 'email' column in the database now stores username_or_email
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
        // The 'email' field in UpdateUserProfilePayload now holds username_or_email
        if let Some(username_or_email_payload) = payload.email {
            if username_or_email_payload != current_user.email {
                // Add validation here if username_or_email_payload needs to be unique across users
                current_user.email = username_or_email_payload;
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
            // The 'email' column in the database now stores username_or_email
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

