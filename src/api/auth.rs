/// API Key Management Handlers
///
/// Endpoints for creating, listing, and managing API keys
use actix_web::{web, HttpResponse, Result};
use bcrypt;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::api::middleware::ApiKeyInfo;

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub description: Option<String>,
    pub expires_in_days: Option<i64>,
    pub rate_limit_per_minute: Option<i32>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
    pub info: ApiKeyInfo,
}

#[derive(Debug, Serialize)]
pub struct ListApiKeysResponse {
    pub keys: Vec<ApiKeyInfo>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApiKeyRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub rate_limit_per_minute: Option<i32>,
    pub permissions: Option<Vec<String>>,
}

// ============================================================================
// HANDLERS
// ============================================================================

/// Create a new API key
pub async fn create_api_key(
    db: web::Data<SqlitePool>,
    req: web::Json<CreateApiKeyRequest>,
) -> Result<HttpResponse> {
    // Generate new API key
    let api_key = format!("dpn_{}", Uuid::new_v4().to_string().replace("-", ""));

    // Hash the API key for storage
    let key_hash = bcrypt::hash(&api_key, bcrypt::DEFAULT_COST)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Calculate expiration
    let expires_at = req
        .expires_in_days
        .map(|days| (Utc::now() + Duration::days(days)).to_rfc3339());

    // Serialize permissions
    let permissions = serde_json::to_string(&req.permissions.clone().unwrap_or_default())
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let rate_limit = req.rate_limit_per_minute.unwrap_or(60);
    let now = Utc::now().to_rfc3339();

    // Insert into database
    let result = sqlx::query!(
        r#"
        INSERT INTO api_keys
            (key_hash, name, description, created_at, expires_at, is_active, rate_limit_per_minute, permissions)
        VALUES (?, ?, ?, ?, ?, 1, ?, ?)
        "#,
        key_hash,
        req.name,
        req.description,
        now,
        expires_at,
        rate_limit,
        permissions
    )
    .execute(db.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let key_id = result.last_insert_rowid();

    // Fetch the created key info
    let info = get_api_key_info(db.get_ref(), key_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(CreateApiKeyResponse { api_key, info }))
}

/// List all API keys (excluding key hashes)
pub async fn list_api_keys(db: web::Data<SqlitePool>) -> Result<HttpResponse> {
    let records = sqlx::query!(
        r#"
        SELECT
            id, name, description, created_at, expires_at, last_used_at,
            is_active, rate_limit_per_minute, permissions
        FROM api_keys
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(db.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let keys: Vec<ApiKeyInfo> = records
        .into_iter()
        .map(|r| ApiKeyInfo {
            id: r.id,
            name: r.name,
            description: r.description,
            created_at: DateTime::from_naive_utc_and_offset(r.created_at, Utc),
            expires_at: r
                .expires_at
                .map(|e| DateTime::from_naive_utc_and_offset(e, Utc)),
            last_used_at: r
                .last_used_at
                .map(|l| DateTime::from_naive_utc_and_offset(l, Utc)),
            is_active: r.is_active,
            rate_limit_per_minute: r.rate_limit_per_minute as i32,
            permissions: serde_json::from_str(&r.permissions).unwrap_or_default(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(ListApiKeysResponse { keys }))
}

/// Get API key by ID
pub async fn get_api_key(
    db: web::Data<SqlitePool>,
    key_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let info = get_api_key_info(db.get_ref(), *key_id)
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json(info))
}

/// Update API key
pub async fn update_api_key(
    db: web::Data<SqlitePool>,
    key_id: web::Path<i64>,
    req: web::Json<UpdateApiKeyRequest>,
) -> Result<HttpResponse> {
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(name) = &req.name {
        updates.push("name = ?");
        params.push(name.clone());
    }

    if let Some(description) = &req.description {
        updates.push("description = ?");
        params.push(description.clone());
    }

    if let Some(is_active) = req.is_active {
        updates.push("is_active = ?");
        params.push(if is_active { "1" } else { "0" }.to_string());
    }

    if let Some(rate_limit) = req.rate_limit_per_minute {
        updates.push("rate_limit_per_minute = ?");
        params.push(rate_limit.to_string());
    }

    if let Some(permissions) = &req.permissions {
        updates.push("permissions = ?");
        let perms_json = serde_json::to_string(permissions)
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        params.push(perms_json);
    }

    if updates.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "NO_UPDATES",
            "message": "No fields to update"
        })));
    }

    // Execute update (simplified version - in production use proper parameterized query)
    let key_id_value = *key_id;
    sqlx::query(&format!(
        "UPDATE api_keys SET {} WHERE id = ?",
        updates.join(", ")
    ))
    .bind(key_id_value)
    .execute(db.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let info = get_api_key_info(db.get_ref(), key_id_value)
        .await
        .map_err(|e| actix_web::error::ErrorNotFound(e))?;

    Ok(HttpResponse::Ok().json(info))
}

/// Delete (revoke) API key
pub async fn delete_api_key(
    db: web::Data<SqlitePool>,
    key_id: web::Path<i64>,
) -> Result<HttpResponse> {
    sqlx::query!("DELETE FROM api_keys WHERE id = ?", *key_id)
        .execute(db.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "API key revoked successfully"
    })))
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

async fn get_api_key_info(db: &SqlitePool, key_id: i64) -> Result<ApiKeyInfo, String> {
    let record = sqlx::query!(
        r#"
        SELECT
            id, name, description, created_at, expires_at, last_used_at,
            is_active, rate_limit_per_minute, permissions
        FROM api_keys
        WHERE id = ?
        "#,
        key_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| "API key not found".to_string())?;

    Ok(ApiKeyInfo {
        id: record.id,
        name: record.name,
        description: record.description,
        created_at: DateTime::from_naive_utc_and_offset(record.created_at, Utc),
        expires_at: record
            .expires_at
            .map(|e| DateTime::from_naive_utc_and_offset(e, Utc)),
        last_used_at: record
            .last_used_at
            .map(|l| DateTime::from_naive_utc_and_offset(l, Utc)),
        is_active: record.is_active,
        rate_limit_per_minute: record.rate_limit_per_minute as i32,
        permissions: serde_json::from_str(&record.permissions).unwrap_or_default(),
    })
}
