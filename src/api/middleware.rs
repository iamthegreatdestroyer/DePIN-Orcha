/// Request Middleware
///
/// Middleware for request tracking, authentication, rate limiting, and validation.
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ResponseError,
    http::StatusCode,
    Error, HttpMessage, HttpResponse,
};
use bcrypt;
use chrono::{DateTime, Utc};
use futures_util::future::LocalBoxFuture;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::fmt;
use std::{
    rc::Rc,
    sync::Arc,
    time::{Duration, SystemTime},
};
use uuid::Uuid;

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug)]
pub enum AuthError {
    MissingApiKey,
    InvalidApiKey,
    ExpiredApiKey,
    InactiveApiKey,
    DatabaseError(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::MissingApiKey => write!(f, "API key is required"),
            AuthError::InvalidApiKey => write!(f, "Invalid API key"),
            AuthError::ExpiredApiKey => write!(f, "API key has expired"),
            AuthError::InactiveApiKey => write!(f, "API key is inactive"),
            AuthError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingApiKey => StatusCode::UNAUTHORIZED,
            AuthError::InvalidApiKey => StatusCode::UNAUTHORIZED,
            AuthError::ExpiredApiKey => StatusCode::UNAUTHORIZED,
            AuthError::InactiveApiKey => StatusCode::FORBIDDEN,
            AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": "AUTHENTICATION_ERROR",
            "message": self.to_string(),
            "timestamp": Utc::now().to_rfc3339(),
        }))
    }
}

#[derive(Debug)]
pub enum RateLimitError {
    Exceeded { retry_after: u64 },
    DatabaseError(String),
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitError::Exceeded { retry_after } => {
                write!(
                    f,
                    "Rate limit exceeded. Retry after {} seconds",
                    retry_after
                )
            }
            RateLimitError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl ResponseError for RateLimitError {
    fn status_code(&self) -> StatusCode {
        StatusCode::TOO_MANY_REQUESTS
    }

    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponse::build(self.status_code());

        if let RateLimitError::Exceeded { retry_after } = self {
            builder.insert_header(("Retry-After", retry_after.to_string()));
            builder.insert_header((
                "X-RateLimit-Reset",
                (SystemTime::now() + Duration::from_secs(*retry_after))
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string(),
            ));
        }

        builder.json(serde_json::json!({
            "error": "RATE_LIMIT_EXCEEDED",
            "message": self.to_string(),
            "timestamp": Utc::now().to_rfc3339(),
        }))
    }
}

// ============================================================================
// API KEY MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub rate_limit_per_minute: i32,
    pub permissions: Vec<String>,
}

// ============================================================================
// REQUEST ID MIDDLEWARE
// ============================================================================

/// Request ID Middleware
/// Attaches a unique request ID to each request
pub struct RequestIdMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RequestIdMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddlewareService<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(RequestIdMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestIdMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        let service = self.service.clone();

        req.extensions_mut().insert(RequestId(request_id));

        Box::pin(async move {
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

/// Request ID wrapper
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

// ============================================================================
// AUTHENTICATION MIDDLEWARE
// ============================================================================

/// Authentication middleware for API key validation
pub struct AuthMiddleware {
    pub db: Arc<SqlitePool>,
    pub skip_auth_paths: Vec<String>,
}

impl AuthMiddleware {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self {
            db,
            skip_auth_paths: vec!["/api/v1/health".to_string(), "/ws".to_string()],
        }
    }

    async fn validate_api_key(db: &SqlitePool, api_key: &str) -> Result<ApiKeyInfo, AuthError> {
        // Fetch all active keys and verify with bcrypt
        let records = sqlx::query!(
            r#"
            SELECT
                id, key_hash, name, description, created_at, expires_at, last_used_at,
                is_active, rate_limit_per_minute, permissions
            FROM api_keys
            WHERE is_active = 1
            "#
        )
        .fetch_all(db)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        // Find matching key by verifying with bcrypt
        let mut matching_record = None;
        for record in records {
            if bcrypt::verify(api_key, &record.key_hash).unwrap_or(false) {
                matching_record = Some(record);
                break;
            }
        }

        let record = matching_record.ok_or(AuthError::InvalidApiKey)?;

        // Check if key is active
        if !record.is_active {
            return Err(AuthError::InactiveApiKey);
        }

        // Check if key has expired
        if let Some(expires_at) = record.expires_at {
            let expires: DateTime<Utc> = DateTime::from_naive_utc_and_offset(expires_at, Utc);

            if Utc::now() > expires {
                return Err(AuthError::ExpiredApiKey);
            }
        }

        // Update last_used_at
        let now = Utc::now().naive_utc();
        let _ = sqlx::query!(
            "UPDATE api_keys SET last_used_at = ? WHERE id = ?",
            now,
            record.id
        )
        .execute(db)
        .await;

        // Parse permissions
        let permissions: Vec<String> =
            serde_json::from_str(&record.permissions).unwrap_or_default();

        Ok(ApiKeyInfo {
            id: record.id.unwrap_or(0),
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
            permissions,
        })
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            db: self.db.clone(),
            skip_auth_paths: self.skip_auth_paths.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    db: Arc<SqlitePool>,
    skip_auth_paths: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let skip_auth = self.skip_auth_paths.iter().any(|p| path.starts_with(p));

        if skip_auth {
            let service = self.service.clone();
            return Box::pin(async move {
                let res = service.call(req).await?;
                Ok(res)
            });
        }

        let db = self.db.clone();
        let service = self.service.clone();

        Box::pin(async move {
            // Extract API key from header
            let api_key = req
                .headers()
                .get("X-API-Key")
                .and_then(|h| h.to_str().ok())
                .ok_or(AuthError::MissingApiKey)?;

            // Validate API key
            let key_info = AuthMiddleware::validate_api_key(&db, api_key).await?;

            // Store key info in request extensions
            req.extensions_mut().insert(key_info);

            // Continue with request
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

// ============================================================================
// RATE LIMITING MIDDLEWARE
// ============================================================================

/// Rate limiting middleware
pub struct RateLimitMiddleware {
    pub db: Arc<SqlitePool>,
    pub window_duration: Duration,
}

impl RateLimitMiddleware {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self {
            db,
            window_duration: Duration::from_secs(60), // 1 minute window
        }
    }

    async fn check_rate_limit(
        db: &SqlitePool,
        api_key_id: i64,
        endpoint: &str,
        limit: i32,
        window_duration: Duration,
    ) -> Result<(), RateLimitError> {
        let window_start = Utc::now() - chrono::Duration::from_std(window_duration).unwrap();
        let window_start_naive = window_start.naive_utc();

        // Count requests in current window
        let count = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(request_count), 0) as count
            FROM rate_limit_log
            WHERE api_key_id = ? AND window_start >= ?
            "#,
            api_key_id,
            window_start_naive
        )
        .fetch_one(db)
        .await
        .map_err(|e| RateLimitError::DatabaseError(e.to_string()))?;

        let current_count = count.count;

        if current_count >= limit {
            return Err(RateLimitError::Exceeded {
                retry_after: window_duration.as_secs(),
            });
        }

        // Record this request
        let now = Utc::now().naive_utc();
        let _ = sqlx::query!(
            r#"
            INSERT INTO rate_limit_log (api_key_id, endpoint, request_count, window_start)
            VALUES (?, ?, 1, ?)
            ON CONFLICT(api_key_id, endpoint, window_start) DO UPDATE SET
                request_count = request_count + 1
            "#,
            api_key_id,
            endpoint,
            now
        )
        .execute(db)
        .await;

        Ok(())
    }

    async fn cleanup_old_logs(db: &SqlitePool, retention: Duration) {
        let cutoff = Utc::now() - chrono::Duration::from_std(retention).unwrap();
        let cutoff_naive = cutoff.naive_utc();

        let _ = sqlx::query!(
            "DELETE FROM rate_limit_log WHERE window_start < ?",
            cutoff_naive
        )
        .execute(db)
        .await;
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // Spawn cleanup task
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Cleanup hourly
            loop {
                interval.tick().await;
                RateLimitMiddleware::cleanup_old_logs(&db, Duration::from_secs(86400)).await;
                // Keep 24 hours
            }
        });

        std::future::ready(Ok(RateLimitMiddlewareService {
            service: Rc::new(service),
            db: self.db.clone(),
            window_duration: self.window_duration,
        }))
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: Rc<S>,
    db: Arc<SqlitePool>,
    window_duration: Duration,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let db = self.db.clone();
        let window_duration = self.window_duration;
        let service = self.service.clone();
        let endpoint = req.path().to_string();

        Box::pin(async move {
            // Get API key info from extensions (set by AuthMiddleware)
            let key_info = req.extensions().get::<ApiKeyInfo>().cloned();

            if let Some(key_info) = key_info {
                // Check rate limit
                RateLimitMiddleware::check_rate_limit(
                    &db,
                    key_info.id,
                    &endpoint,
                    key_info.rate_limit_per_minute,
                    window_duration,
                )
                .await?;
            }

            // Continue with request
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id_creation() {
        let id = RequestId(Uuid::new_v4().to_string());
        assert!(!id.0.is_empty());
    }

    #[test]
    fn test_request_id_uuid_format() {
        let uuid = Uuid::new_v4().to_string();
        assert_eq!(uuid.len(), 36); // UUID v4 string length
    }
}
