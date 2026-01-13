// DePIN Orcha - Core Orchestrator Main Entry Point
//
// This is the main orchestration engine that coordinates all protocol adapters
// and the ML optimization engine to maximize earnings across DePIN protocols.

#![warn(rust_2018_idioms)]

use anyhow::Result;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info};

mod config;
mod db;
mod error;
mod metrics;
mod protocols;

use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging
    init_tracing()?;

    info!("🚀 DePIN Orcha Orchestrator Starting...");

    // Load configuration
    let config = Config::load()?;
    info!("✅ Configuration loaded");

    // Initialize database
    db::init(&config).await?;
    info!("✅ Database initialized");

    // Initialize metrics
    metrics::init();
    info!("✅ Metrics initialized");

    // TODO: Initialize protocol adapters
    // TODO: Start optimization loop
    // TODO: Setup HTTP server
    // TODO: Setup health checks

    info!("✅ DePIN Orcha Orchestrator Ready");

    // Wait for shutdown signal
    let shutdown = signal::ctrl_c();
    tokio::select! {
        _ = shutdown => {
            info!("🛑 Shutdown signal received");
        }
    }

    info!("✅ DePIN Orcha Orchestrator Shutdown Complete");
    Ok(())
}

fn init_tracing() -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,depin_orcha=debug"));

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(env_filter)
        .init();

    Ok(())
}
