/// API Module - HTTP REST & WebSocket Server
///
/// Provides RESTful endpoints for orchestration operations and real-time
/// WebSocket connectivity for dashboard updates.

pub mod handlers;
pub mod models;
pub mod routes;
pub mod websocket;
pub mod middleware;

use actix_web::{web, App, HttpServer, middleware::Logger};
use std::sync::Arc;
use crate::orchestration::{
    ProtocolCoordinator, EarningsOptimizer, ReallocationEngine, RealtimeMonitor,
};
use tracing::info;

// ============================================================================
// API STATE
// ============================================================================

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub coordinator: Arc<ProtocolCoordinator>,
    pub optimizer: Arc<tokio::sync::Mutex<EarningsOptimizer>>,
    pub reallocation: Arc<ReallocationEngine>,
    pub monitor: Arc<RealtimeMonitor>,
}

impl AppState {
    /// Create new application state
    pub fn new(
        coordinator: Arc<ProtocolCoordinator>,
        optimizer: Arc<tokio::sync::Mutex<EarningsOptimizer>>,
        reallocation: Arc<ReallocationEngine>,
        monitor: Arc<RealtimeMonitor>,
    ) -> Self {
        Self {
            coordinator,
            optimizer,
            reallocation,
            monitor,
        }
    }
}

// ============================================================================
// SERVER SETUP
// ============================================================================

/// API Server Configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub request_timeout: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: 4,
            request_timeout: 30,
        }
    }
}

/// Start the API server
pub async fn start_server(
    config: ApiConfig,
    app_state: AppState,
) -> std::io::Result<()> {
    let addr = format!("{}:{}", config.host, config.port);
    info!("🚀 Starting API server on {}", addr);

    let app_state_clone = app_state.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state_clone.clone()))
            .wrap(Logger::default())
            .wrap(middleware::RequestIdMiddleware)
            .configure(routes::configure_routes)
    })
    .workers(config.workers)
    .bind(&addr)?
    .run()
    .await
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_config_default() {
        let config = ApiConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_api_config_custom() {
        let config = ApiConfig {
            host: "0.0.0.0".to_string(),
            port: 3000,
            workers: 8,
            request_timeout: 60,
        };
        assert_eq!(config.port, 3000);
    }
}
