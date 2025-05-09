use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, DateTime}; // Ensure Utc and DateTime are imported for timestamps

use crate::{
    errors::AppError,
    models::{NewReceipt, Receipt, UpdateReceiptPayload},
};

#[derive(Clone)]
pub struct ReceiptService {
    pool: SqlitePool,
}

impl ReceiptService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_receipt(&self, user_id: String, payload: NewReceipt) -> Result<Receipt, AppError> {
        let receipt_id = Uuid::new_v4().to_string();
        let current_time = Utc::now();

        // Using sqlx::query() for runtime checking as a diagnostic step
        sqlx::query(
            "INSERT INTO receipts (id, user_id, title, amount, date, description, category, file_url, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&receipt_id)
        .bind(&user_id)
        .bind(&payload.title)
        .bind(payload.amount)
        .bind(payload.date) // DateTime<Utc> should be handled by sqlx with chrono feature
        .bind(payload.description.as_deref())
        .bind(payload.category.as_deref())
        .bind(payload.file_url.as_deref())
        .bind(current_time) // DateTime<Utc>
        .bind(current_time) // DateTime<Utc>
        .execute(&self.pool)
        .await
        .map_err(AppError::SqlxError)?;

        // Fetch the newly created receipt
        self.get_receipt_by_id(&receipt_id)
            .await?
            .ok_or_else(|| AppError::InternalServerError("Failed to retrieve created receipt".to_string()))
    }

    pub async fn get_receipts_by_user_id(&self, user_id: &str, page: u32, per_page: u32) -> Result<Vec<Receipt>, AppError> {
        let offset = (page.saturating_sub(1)) * per_page; // Ensure page is at least 1
        // Simplified SELECT query, removing explicit type casts for DateTime<Utc>
        sqlx::query_as::<_, Receipt>(r#"SELECT id, user_id, title, amount, date, description, category, file_url, created_at, updated_at FROM receipts WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"#)
            .bind(user_id)
            .bind(per_page as i64) // Bind as i64 for LIMIT/OFFSET
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::SqlxError)
    }

    pub async fn get_receipt_by_id(&self, receipt_id: &str) -> Result<Option<Receipt>, AppError> {
        // Simplified SELECT query, removing explicit type casts for DateTime<Utc>
        sqlx::query_as::<_, Receipt>(r#"SELECT id, user_id, title, amount, date, description, category, file_url, created_at, updated_at FROM receipts WHERE id = ?"#)
            .bind(receipt_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::SqlxError)
    }

    pub async fn update_receipt(&self, receipt_id: &str, user_id: &str, payload: UpdateReceiptPayload) -> Result<Receipt, AppError> {
        let current_time = Utc::now();
        let mut current_receipt = self.get_receipt_by_id(receipt_id).await?
            .ok_or_else(|| AppError::NotFound("Receipt not found".to_string()))?;

        if current_receipt.user_id != user_id {
            return Err(AppError::AuthError("User not authorized to update this receipt".to_string()));
        }
        
        let mut changed = false;

        if let Some(title) = payload.title {
            if current_receipt.title != title {
                current_receipt.title = title;
                changed = true;
            }
        }
        if let Some(amount) = payload.amount {
            if current_receipt.amount != amount {
                current_receipt.amount = amount;
                changed = true;
            }
        }
        if let Some(date) = payload.date {
            if current_receipt.date != date {
                current_receipt.date = date;
                changed = true;
            }
        }
        if payload.description.is_some() && current_receipt.description != payload.description {
            current_receipt.description = payload.description;
            changed = true;
        }
        if payload.category.is_some() && current_receipt.category != payload.category {
            current_receipt.category = payload.category;
            changed = true;
        }
        if payload.file_url.is_some() && current_receipt.file_url != payload.file_url {
            current_receipt.file_url = payload.file_url;
            changed = true;
        }

        if changed {
            current_receipt.updated_at = current_time; 
            // Using sqlx::query() for runtime checking as a diagnostic step
            sqlx::query(
                "UPDATE receipts SET title = ?, amount = ?, date = ?, description = ?, category = ?, file_url = ?, updated_at = ? WHERE id = ? AND user_id = ?"
            )
            .bind(&current_receipt.title)
            .bind(current_receipt.amount)
            .bind(current_receipt.date) // DateTime<Utc>
            .bind(current_receipt.description.as_deref())
            .bind(current_receipt.category.as_deref())
            .bind(current_receipt.file_url.as_deref())
            .bind(current_time) // DateTime<Utc>
            .bind(receipt_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::SqlxError)?;
        }

        self.get_receipt_by_id(receipt_id).await?.ok_or_else(|| AppError::InternalServerError("Failed to retrieve updated receipt".to_string()))
    }

    pub async fn delete_receipt(&self, receipt_id: &str, user_id: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM receipts WHERE id = ? AND user_id = ?")
            .bind(receipt_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::SqlxError)?;

        if result.rows_affected() == 0 {
            Err(AppError::NotFound("Receipt not found or user not authorized".to_string()))
        } else {
            Ok(())
        }
    }

    pub async fn admin_get_all_receipts(&self, page: u32, per_page: u32) -> Result<Vec<Receipt>, AppError> {
        let offset = (page.saturating_sub(1)) * per_page;
        // Simplified SELECT query, removing explicit type casts for DateTime<Utc>
        sqlx::query_as::<_, Receipt>(r#"SELECT id, user_id, title, amount, date, description, category, file_url, created_at, updated_at FROM receipts ORDER BY created_at DESC LIMIT ? OFFSET ?"#)
            .bind(per_page as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::SqlxError)
    }
}

