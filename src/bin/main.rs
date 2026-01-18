// Copyright (c) 2026 DePIN-Orcha Project. All Rights Reserved.
// Phase 5: Application Entry Point - Production-Ready Binary

//! # DePIN-Orcha Main Application
//!
//! This is the primary executable entry point for the DePIN-Orcha orchestration system.
//! It initializes all components, starts the HTTP/WebSocket server, and manages graceful shutdown.
//!
//! ## Architecture
//! ```text
//! main.rs
//!   ├─> Load Configuration (env vars + .env file)
//!   ├─> Initialize Database (SQLite pool + schema)
//!   ├─> Create Orchestration Engine (ProtocolCoordinator)
//!   ├─> Start HTTP Server (Actix-web with routes)
//!   ├─> Start WebSocket Server (Real-time streaming)
//!   ├─> Start Background Schedulers (Optimization, Cleanup)
//!   └─> Handle Graceful Shutdown (SIGTERM, SIGINT)
//! ```
//!
//! ## Environment Variables
//! - `API_HOST`: API server host (default: "127.0.0.1")
//! - `API_PORT`: API server port (default: "8080")
//! - `API_WORKERS`: Number of worker threads (default: 4)
//! - `API_REQUEST_TIMEOUT`: Request timeout in seconds (default: 30)
//! - `DATABASE_URL`: SQLite database path (default: "depin_orcha.db")
//! - `DB_MAX_CONNECTIONS`: Max pool connections (default: 10)
//! - `DB_MIN_CONNECTIONS`: Min pool connections (default: 2)
//! - `LOG_LEVEL`: Logging level (default: "info")
//! - `RUST_LOG`: Rust logging configuration (overrides LOG_LEVEL)

use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::Mutex;

// Import our modules
use depin_orcha::api::{routes::configure_routes, middleware::RequestIdMiddleware, websocket, ApiConfig, AppState};
use depin_orcha::db::{create_schema, init_pool, DbConfig};
use depin_orcha::{
    ProtocolCoordinator, EarningsOptimizer, OptimizerConfig,
    ReallocationEngine, ReallocationConfig,
    RealtimeMonitor, MonitorConfig,
};

/// Main application entry point
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file (if present)
    dotenv::dotenv().ok();

    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("🚀 Starting DePIN-Orcha Orchestration System v1.0.0");
    log::info!("📅 Date: {}", chrono::Utc::now().to_rfc3339());

    // Step 1: Load API Configuration
    let api_config = load_api_config();
    log::info!("✅ API Configuration loaded:");
    log::info!("   Host: {}", api_config.host);
    log::info!("   Port: {}", api_config.port);
    log::info!("   Workers: {}", api_config.workers);
    log::info!("   Request Timeout: {}s", api_config.request_timeout);

    // Step 2: Load Database Configuration
    let db_config = load_db_config();
    log::info!("✅ Database Configuration loaded:");
    log::info!("   URL: {}", db_config.database_url);
    log::info!("   Max Connections: {}", db_config.max_connections);
    log::info!("   Min Connections: {}", db_config.min_connections);

    // Step 3: Initialize Database Connection Pool
    log::info!("🔧 Initializing database connection pool...");
    let db_pool = init_pool(db_config.clone())
        .await
        .expect("Failed to initialize database pool");
    log::info!("✅ Database pool initialized with {} connections", db_config.max_connections);

    // Step 4: Create Database Schema
    log::info!("🔧 Creating database schema...");
    create_schema(&db_pool)
        .await
        .expect("Failed to create database schema");
    log::info!("✅ Database schema created successfully");

    // Step 5: Initialize Protocol Coordinator (Orchestration Engine)
    log::info!("🔧 Initializing Protocol Coordinator...");
    // Step 6: Create orchestration components
    let coordinator = Arc::new(ProtocolCoordinator::new(1000)); // Keep 1000 history entries
    log::info!("✅ Protocol Coordinator initialized");

    let optimizer_config = OptimizerConfig::default();
    let optimizer = Arc::new(Mutex::new(EarningsOptimizer::new(optimizer_config)));
    log::info!("✅ Earnings Optimizer initialized");

    let reallocation_config = ReallocationConfig::default();
    let reallocation = Arc::new(ReallocationEngine::new(reallocation_config));
    log::info!("✅ Reallocation Engine initialized");

    let monitor_config = MonitorConfig::default();
    let monitor = Arc::new(RealtimeMonitor::new(monitor_config));
    log::info!("✅ Realtime Monitor initialized");

    // Step 7: Create Application State (thread-safe, Arc-wrapped)
    let app_state = web::Data::new(AppState {
        coordinator: coordinator.clone(),
        optimizer: optimizer.clone(),
        reallocation: reallocation.clone(),
        monitor: monitor.clone(),
    });
    log::info!("✅ Application state created");

    // Step 8: Start Background Schedulers
    log::info!("🔧 Starting background schedulers...");
    let scheduler_config = depin_orcha::scheduler::SchedulerConfig::from_env();
    depin_orcha::scheduler::start_schedulers(
        coordinator.clone(),
        db_pool.clone(),
        scheduler_config,
    );
    log::info!("✅ Background schedulers started successfully");

    // Step 8: Build and Start HTTP Server
    let bind_address = format!("{}:{}", api_config.host, api_config.port);
    log::info!("🌐 Starting HTTP server at http://{}", bind_address);

    let db_pool_arc = Arc::new(db_pool.clone());

    let server = HttpServer::new(move || {
        App::new()
            // Add application state
            .app_data(app_state.clone())
            .app_data(web::Data::new(db_pool.clone()))
            // Add middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Configure API routes with database pool
            .configure(|cfg| configure_routes(cfg, db_pool_arc.clone()))
            // Add WebSocket endpoint
            .route("/ws", web::get().to(websocket::ws_handler))
            // Add health check at root
            .route("/", web::get().to(|| async { "DePIN-Orcha Orchestration System v1.0.0" }))
    })
    .workers(api_config.workers)
    .bind(&bind_address)?
    .run();

    log::info!("✅ HTTP server started successfully");
    log::info!("📡 WebSocket endpoint: ws://{}/ws", bind_address);
    log::info!("🏥 Health check: http://{}/api/v1/health", bind_address);
    log::info!("📊 Dashboard: http://{}/api/v1/dashboard", bind_address);

    // Step 9: Handle Graceful Shutdown
    let server_handle = server.handle();

    tokio::spawn(async move {
        shutdown_signal().await;
        log::warn!("⚠️  Shutdown signal received, initiating graceful shutdown...");
        server_handle.stop(true).await;
    });

    // Run server until stopped
    log::info!("🎉 DePIN-Orcha is now running!");
    log::info!("   Press Ctrl+C to shutdown gracefully");
    log::info!("");

    server.await?;

    log::info!("✅ Server shutdown complete");
    log::info!("👋 Goodbye!");

    Ok(())
}

/// Load API configuration from environment variables
fn load_api_config() -> ApiConfig {
    ApiConfig {
        host: std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: std::env::var("API_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("API_PORT must be a valid number"),
        workers: std::env::var("API_WORKERS")
            .unwrap_or_else(|_| "4".to_string())
            .parse()
            .expect("API_WORKERS must be a valid number"),
        request_timeout: std::env::var("API_REQUEST_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .expect("API_REQUEST_TIMEOUT must be a valid number"),
    }
}

/// Load database configuration from environment variables
fn load_db_config() -> DbConfig {
    DbConfig {
        database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "depin_orcha.db".to_string()),
        max_connections: std::env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("DB_MAX_CONNECTIONS must be a valid number"),
        min_connections: std::env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "2".to_string())
            .parse()
            .expect("DB_MIN_CONNECTIONS must be a valid number"),
        connect_timeout: std::env::var("DB_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .expect("DB_CONNECT_TIMEOUT must be a valid number"),
    }
}

/// Wait for shutdown signal (SIGTERM or Ctrl+C)
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            log::info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            log::info!("Received SIGTERM signal");
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_config_defaults() {
        // Clear env vars for test
        std::env::remove_var("API_HOST");
        std::env::remove_var("API_PORT");
        std::env::remove_var("API_WORKERS");
        std::env::remove_var("API_REQUEST_TIMEOUT");

        let config = load_api_config();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.workers, 4);
        assert_eq!(config.request_timeout, 30);
    }

    #[test]
    fn test_db_config_defaults() {
        // Clear env vars for test
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("DB_MAX_CONNECTIONS");
        std::env::remove_var("DB_MIN_CONNECTIONS");
        std::env::remove_var("DB_CONNECT_TIMEOUT");

        let config = load_db_config();
        assert_eq!(config.database_url, "depin_orcha.db");
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 2);
        assert_eq!(config.connect_timeout, 5);
    }

    #[test]
    fn test_api_config_from_env() {
        std::env::set_var("API_HOST", "0.0.0.0");
        std::env::set_var("API_PORT", "3000");
        std::env::set_var("API_WORKERS", "8");
        std::env::set_var("API_REQUEST_TIMEOUT", "60");

        let config = load_api_config();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.workers, 8);
        assert_eq!(config.request_timeout, 60);

        // Cleanup
        std::env::remove_var("API_HOST");
        std::env::remove_var("API_PORT");
        std::env::remove_var("API_WORKERS");
        std::env::remove_var("API_REQUEST_TIMEOUT");
    }
}
