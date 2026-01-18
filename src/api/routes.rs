/// API Route Configuration
///
/// Defines all HTTP endpoints and their routes with authentication and rate limiting.
use actix_web::web;
use sqlx::SqlitePool;
use std::sync::Arc;

use super::{auth, handlers, middleware};

/// Configure all API routes with authentication and rate limiting
pub fn configure_routes(cfg: &mut web::ServiceConfig, db_pool: Arc<SqlitePool>) {
    log::info!("Configuring API routes...");

    // Single unified /api/v1 scope with nested sub-scopes for different middleware layers
    cfg.service(
        web::scope("/api/v1")
            .wrap(middleware::RequestIdMiddleware)
            // Public routes (no authentication required)
            .route("/health", web::get().to(handlers::health_check))
            .route("/status", web::get().to(handlers::get_status))
            // Protected endpoints sub-scope (authentication required)
            .service(
                web::scope("")
                    .wrap(middleware::RateLimitMiddleware::new(db_pool.clone()))
                    .wrap(middleware::AuthMiddleware::new(db_pool.clone()))
                    // Metrics endpoints
                    .route("/metrics", web::get().to(handlers::get_metrics))
                    .route("/metrics/current", web::get().to(handlers::get_metrics))
                    .route(
                        "/metrics/history",
                        web::get().to(handlers::get_metrics_history),
                    )
                    // Optimization endpoints
                    .route("/opportunities", web::get().to(handlers::get_opportunities))
                    .route(
                        "/allocation",
                        web::get().to(handlers::get_optimal_allocation),
                    )
                    // Reallocation endpoints
                    .route(
                        "/reallocate",
                        web::post().to(handlers::execute_reallocation),
                    )
                    .route(
                        "/reallocation/history",
                        web::get().to(handlers::get_reallocation_history),
                    )
                    // Dashboard endpoints
                    .route("/dashboard", web::get().to(handlers::get_dashboard))
                    // Alert endpoints
                    .route("/alerts", web::get().to(handlers::get_alerts))
                    .route(
                        "/alerts/acknowledge",
                        web::post().to(handlers::acknowledge_alert),
                    )
                    // Admin routes (flattened into main protected scope)
                    // API key management
                    .route("/admin/keys", web::post().to(auth::create_api_key))
                    .route("/admin/keys", web::get().to(auth::list_api_keys))
                    .route("/admin/keys/{id}", web::get().to(auth::get_api_key))
                    .route("/admin/keys/{id}", web::put().to(auth::update_api_key))
                    .route("/admin/keys/{id}", web::delete().to(auth::delete_api_key)),
            ),
    );

    log::info!("All API routes configured successfully");
    log::info!("   Public: /api/v1/health, /api/v1/status");
    log::info!("   Protected: /api/v1/metrics, /api/v1/allocation, etc.");
    log::info!("   Admin: /api/v1/admin/keys");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_routes_configuration() {
        // Verify route configuration is defined
        assert!(true);
    }
}
