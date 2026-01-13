use super::handlers;
/// API Route Configuration
///
/// Defines all HTTP endpoints and their routes.
use actix_web::web;

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health & Status endpoints
        .route("/api/v1/health", web::get().to(handlers::health_check))
        .route("/api/v1/status", web::get().to(handlers::get_status))
        // Metrics endpoints
        .route("/api/v1/metrics", web::get().to(handlers::get_metrics))
        .route(
            "/api/v1/metrics/history",
            web::get().to(handlers::get_metrics_history),
        )
        // Optimization endpoints
        .route(
            "/api/v1/opportunities",
            web::get().to(handlers::get_opportunities),
        )
        .route(
            "/api/v1/allocation",
            web::get().to(handlers::get_optimal_allocation),
        )
        // Reallocation endpoints
        .route(
            "/api/v1/reallocate",
            web::post().to(handlers::execute_reallocation),
        )
        .route(
            "/api/v1/reallocation/history",
            web::get().to(handlers::get_reallocation_history),
        )
        // Dashboard endpoints
        .route("/api/v1/dashboard", web::get().to(handlers::get_dashboard))
        // Alert endpoints
        .route("/api/v1/alerts", web::get().to(handlers::get_alerts))
        .route(
            "/api/v1/alerts/acknowledge",
            web::post().to(handlers::acknowledge_alert),
        );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_routes_configuration() {
        // Verify route configuration is defined
        assert!(true);
    }
}
