// Copyright (c) 2026 DePIN-Orcha Project. All Rights Reserved.
// Phase 5: Background Task Scheduler

//! # Background Task Scheduler
//!
//! This module provides scheduled background tasks for:
//! - Periodic optimization runs
//! - Automatic reallocation execution
//! - Metrics cleanup and archival
//! - Alert processing pipeline
//! - Report generation
//!
//! ## Architecture
//! ```text
//! Scheduler
//!   ├─> OptimizationTask (every N seconds)
//!   │     └─> Analyze opportunities → Trigger reallocations
//!   ├─> CleanupTask (daily)
//!   │     └─> Remove old metrics → Archive alerts
//!   ├─> AlertProcessor (every minute)
//!   │     └─> Check thresholds → Generate alerts
//!   └─> ReportGenerator (hourly)
//!         └─> Generate performance reports → Store to DB
//! ```

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

use crate::ProtocolCoordinator;

/// Configuration for scheduler tasks
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Optimization interval in seconds (default: 300 = 5 minutes)
    pub optimization_interval: u64,
    /// Metrics retention period in days (default: 30)
    pub metrics_retention_days: i64,
    /// Alert processing interval in seconds (default: 60 = 1 minute)
    pub alert_processing_interval: u64,
    /// Minimum reallocation threshold as percentage improvement (default: 5.0%)
    pub min_reallocation_threshold: f64,
    /// CPU usage alert threshold (default: 90.0%)
    pub cpu_alert_threshold: f64,
    /// Memory usage alert threshold (default: 85.0%)
    pub memory_alert_threshold: f64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            optimization_interval: 300,
            metrics_retention_days: 30,
            alert_processing_interval: 60,
            min_reallocation_threshold: 5.0,
            cpu_alert_threshold: 90.0,
            memory_alert_threshold: 85.0,
        }
    }
}

impl SchedulerConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            optimization_interval: std::env::var("OPTIMIZATION_INTERVAL")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300),
            metrics_retention_days: std::env::var("METRICS_RETENTION_DAYS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            alert_processing_interval: std::env::var("ALERT_PROCESSING_INTERVAL")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(60),
            min_reallocation_threshold: std::env::var("MIN_REALLOCATION_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5.0),
            cpu_alert_threshold: std::env::var("CPU_ALERT_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(90.0),
            memory_alert_threshold: std::env::var("MEMORY_ALERT_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(85.0),
        }
    }
}

/// Start all background schedulers
pub fn start_schedulers(
    coordinator: Arc<ProtocolCoordinator>,
    db_pool: SqlitePool,
    config: SchedulerConfig,
) {
    log::info!("🕐 Starting background schedulers...");
    log::info!("   Optimization interval: {}s", config.optimization_interval);
    log::info!("   Alert processing interval: {}s", config.alert_processing_interval);
    log::info!("   Metrics retention: {} days", config.metrics_retention_days);

    // Spawn optimization task
    tokio::spawn(optimization_task(
        coordinator.clone(),
        db_pool.clone(),
        config.clone(),
    ));

    // Spawn alert processing task
    tokio::spawn(alert_processing_task(
        coordinator.clone(),
        db_pool.clone(),
        config.clone(),
    ));

    // Spawn cleanup task (runs once per day)
    tokio::spawn(cleanup_task(db_pool.clone(), config.clone()));

    log::info!("✅ All schedulers started successfully");
}

/// Periodic optimization task
///
/// Runs every N seconds to:
/// 1. Collect current metrics
/// 2. Analyze optimization opportunities
/// 3. Execute automatic reallocations if threshold met
async fn optimization_task(
    coordinator: Arc<ProtocolCoordinator>,
    db_pool: SqlitePool,
    config: SchedulerConfig,
) {
    let mut interval = interval(Duration::from_secs(config.optimization_interval));
    let mut run_count = 0u64;

    log::info!("🔄 Optimization task started");

    loop {
        interval.tick().await;
        run_count += 1;

        log::debug!("🔄 Running optimization task (run #{})", run_count);

        // Get current metrics (ProtocolCoordinator is already thread-safe)
        let metrics = match coordinator.get_current_metrics().await {
            Ok(Some(m)) => m,
            Ok(None) => {
                log::debug!("No metrics available yet");
                continue;
            }
            Err(e) => {
                log::error!("Failed to get metrics: {}", e);
                continue;
            }
        };

        // Store metrics to database
        if let Err(e) = store_metrics_to_db(&db_pool, &metrics).await {
            log::error!("❌ Failed to store metrics: {}", e);
            continue;
        }

        log::debug!("✅ Metrics collected and stored successfully");

        // TODO: Implement optimization logic with EarningsOptimizer
        // Will be added in a follow-up commit
    }
}

/// Alert processing task
///
/// Runs every minute to:
/// 1. Check resource thresholds (CPU, memory, bandwidth)
/// 2. Generate alerts for anomalies
/// 3. Store alerts to database
async fn alert_processing_task(
    coordinator: Arc<ProtocolCoordinator>,
    db_pool: SqlitePool,
    config: SchedulerConfig,
) {
    let mut interval = interval(Duration::from_secs(config.alert_processing_interval));

    log::info!("🚨 Alert processing task started");

    loop {
        interval.tick().await;

        // Get current metrics (ProtocolCoordinator is already thread-safe)
        let metrics = match coordinator.get_current_metrics().await {
            Ok(Some(m)) => m,
            Ok(None) => {
                continue; // No metrics yet
            }
            Err(e) => {
                log::error!("❌ Failed to get metrics for alerts: {}", e);
                continue;
            }
        };

        // Check CPU threshold
        if metrics.resource_utilization.cpu_percent > config.cpu_alert_threshold {
            let severity = calculate_severity(
                metrics.resource_utilization.cpu_percent,
                config.cpu_alert_threshold,
            );

            if let Err(e) = store_alert_to_db(
                &db_pool,
                "HIGH_CPU_USAGE",
                severity,
                &format!(
                    "CPU usage at {:.1}% (threshold: {:.1}%)",
                    metrics.resource_utilization.cpu_percent, config.cpu_alert_threshold
                ),
            )
            .await
            {
                log::error!("❌ Failed to store CPU alert: {}", e);
            } else {
                log::warn!(
                    "🚨 HIGH CPU USAGE ALERT: {:.1}% (severity: {:.1})",
                    metrics.resource_utilization.cpu_percent,
                    severity
                );
            }
        }

        // Check memory threshold
        if metrics.resource_utilization.memory_percent > config.memory_alert_threshold {
            let severity = calculate_severity(
                metrics.resource_utilization.memory_percent,
                config.memory_alert_threshold,
            );

            if let Err(e) = store_alert_to_db(
                &db_pool,
                "HIGH_MEMORY_USAGE",
                severity,
                &format!(
                    "Memory usage at {:.1}% (threshold: {:.1}%)",
                    metrics.resource_utilization.memory_percent, config.memory_alert_threshold
                ),
            )
            .await
            {
                log::error!("❌ Failed to store memory alert: {}", e);
            } else {
                log::warn!(
                    "🚨 HIGH MEMORY USAGE ALERT: {:.1}% (severity: {:.1})",
                    metrics.resource_utilization.memory_percent,
                    severity
                );
            }
        }
    }
}

/// Cleanup task
///
/// Runs once per day to:
/// 1. Remove old metrics records beyond retention period
/// 2. Archive acknowledged alerts
async fn cleanup_task(db_pool: SqlitePool, config: SchedulerConfig) {
    // Run once per day
    let mut interval = interval(Duration::from_secs(86400)); // 24 hours

    log::info!("🧹 Cleanup task started (runs daily)");

    loop {
        interval.tick().await;

        log::info!("🧹 Running daily cleanup task");

        // Calculate cutoff date
        let cutoff_date = Utc::now() - chrono::Duration::days(config.metrics_retention_days);
        let cutoff_str = cutoff_date.to_rfc3339();

        // Delete old metrics
        let result = sqlx::query(
            "DELETE FROM metrics WHERE created_at < ?1"
        )
        .bind(&cutoff_str)
        .execute(&db_pool)
        .await;

        match result {
            Ok(result) => {
                let rows_deleted = result.rows_affected();
                log::info!("✅ Deleted {} old metrics records", rows_deleted);
            }
            Err(e) => {
                log::error!("❌ Failed to delete old metrics: {}", e);
            }
        }

        // Delete old acknowledged alerts (keep for 7 days)
        let alert_cutoff = Utc::now() - chrono::Duration::days(7);
        let alert_cutoff_str = alert_cutoff.to_rfc3339();

        let result = sqlx::query(
            "DELETE FROM alerts WHERE acknowledged = 1 AND created_at < ?1"
        )
        .bind(&alert_cutoff_str)
        .execute(&db_pool)
        .await;

        match result {
            Ok(result) => {
                let rows_deleted = result.rows_affected();
                log::info!("✅ Deleted {} old acknowledged alerts", rows_deleted);
            }
            Err(e) => {
                log::error!("❌ Failed to delete old alerts: {}", e);
            }
        }

        log::info!("✅ Cleanup task completed");
    }
}

/// Helper: Store metrics to database
async fn store_metrics_to_db(
    db_pool: &SqlitePool,
    metrics: &crate::orchestration::AggregatedMetrics,
) -> Result<(), sqlx::Error> {
    use crate::db::queries::{store_metrics, store_protocol_metrics};
    use chrono::Utc;

    let metrics_id = store_metrics(
        db_pool,
        Utc::now(),
        metrics.total_earnings_per_hour,
        metrics.resource_utilization.cpu_percent,
        metrics.resource_utilization.memory_percent,
        metrics.resource_utilization.bandwidth_percent,
        metrics.resource_utilization.storage_percent,
    )
    .await?;

    // Store per-protocol metrics
    for (protocol, earnings) in &metrics.earnings_by_protocol {
        let allocation = metrics.allocation_by_protocol.get(protocol).copied().unwrap_or(0.0);
        let connected = metrics.connection_status.get(protocol).copied().unwrap_or(false);

        store_protocol_metrics(
            db_pool,
            metrics_id,
            protocol.clone(),
            *earnings,
            allocation,
            connected,
        )
        .await?;
    }

    Ok(())
}

/// Helper: Store reallocation to database
async fn store_reallocation_to_db(
    db_pool: &SqlitePool,
    protocol: &str,
    old_allocation: f64,
    new_allocation: f64,
    earnings_impact: Option<f64>,
    reason: Option<&str>,
) -> Result<(), sqlx::Error> {
    use crate::db::queries::store_reallocation;
    use chrono::Utc;

    store_reallocation(
        db_pool,
        Utc::now(),
        protocol.to_string(),
        old_allocation,
        new_allocation,
        earnings_impact,
        reason.map(|s| s.to_string()),
    )
    .await
}

/// Helper: Store alert to database
async fn store_alert_to_db(
    db_pool: &SqlitePool,
    alert_type: &str,
    severity: f64,
    message: &str,
) -> Result<(), sqlx::Error> {
    use crate::db::queries::store_alert;
    use chrono::Utc;

    store_alert(
        db_pool,
        Utc::now(),
        alert_type.to_string(),
        severity,
        message.to_string(),
    )
    .await
}

/// Calculate alert severity based on threshold exceedance
///
/// Returns a severity score from 0.0 to 100.0
fn calculate_severity(current_value: f64, threshold: f64) -> f64 {
    let exceedance = current_value - threshold;
    let severity_base = (exceedance / threshold) * 100.0;

    // Cap at 100.0
    severity_base.min(100.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_config_defaults() {
        let config = SchedulerConfig::default();
        assert_eq!(config.optimization_interval, 300);
        assert_eq!(config.metrics_retention_days, 30);
        assert_eq!(config.alert_processing_interval, 60);
        assert_eq!(config.min_reallocation_threshold, 5.0);
    }

    #[test]
    fn test_calculate_severity() {
        // Exactly at threshold = 0% severity
        assert_eq!(calculate_severity(90.0, 90.0), 0.0);

        // 5% over threshold = ~5.5% severity
        let severity = calculate_severity(94.5, 90.0);
        assert!(severity > 4.0 && severity < 6.0);

        // 50% over threshold = 50% severity
        let severity = calculate_severity(135.0, 90.0);
        assert!(severity > 49.0 && severity < 51.0);

        // Way over threshold = capped at 100%
        assert_eq!(calculate_severity(300.0, 90.0), 100.0);
    }

    #[test]
    fn test_config_from_env() {
        std::env::set_var("OPTIMIZATION_INTERVAL", "600");
        std::env::set_var("METRICS_RETENTION_DAYS", "60");
        std::env::set_var("MIN_REALLOCATION_THRESHOLD", "10.0");

        let config = SchedulerConfig::from_env();
        assert_eq!(config.optimization_interval, 600);
        assert_eq!(config.metrics_retention_days, 60);
        assert_eq!(config.min_reallocation_threshold, 10.0);

        // Cleanup
        std::env::remove_var("OPTIMIZATION_INTERVAL");
        std::env::remove_var("METRICS_retention_DAYS");
        std::env::remove_var("MIN_REALLOCATION_THRESHOLD");
    }
}
