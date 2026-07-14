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

use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

use crate::{EarningsOptimizer, ProtocolCoordinator};

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
///
/// The `optimizer` is shared (same `Arc`) with the API layer's `AppState` so
/// the advisory recommendations surfaced by the background loop and the
/// `/opportunities` / `/allocation` HTTP endpoints are computed from one
/// consistent metrics history.
pub fn start_schedulers(
    coordinator: Arc<ProtocolCoordinator>,
    optimizer: Arc<Mutex<EarningsOptimizer>>,
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
        optimizer.clone(),
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

/// Periodic optimization task (ADVISORY-ONLY)
///
/// Runs every N seconds to:
/// 1. Feed the coordinator by polling all registered protocol adapters
///    (`poll_all`), refreshing the aggregated metrics and history.
/// 2. Persist the freshly-collected metrics to the database.
/// 3. Invoke the `EarningsOptimizer` to compute reallocation opportunities
///    and an optimal allocation plan, and LOG them as recommendations.
///
/// ADVISORY-ONLY: this task never applies a reallocation and never moves any
/// workload — it only observes and recommends. Automatically executing the
/// optimizer's recommendations (e.g. via `ReallocationEngine::execute_reallocation`
/// or an adapter's `apply_allocation`) is intentionally DEFERRED and left as
/// future work, to be gated behind an explicit operator opt-in.
async fn optimization_task(
    coordinator: Arc<ProtocolCoordinator>,
    optimizer: Arc<Mutex<EarningsOptimizer>>,
    db_pool: SqlitePool,
    config: SchedulerConfig,
) {
    let mut interval = interval(Duration::from_secs(config.optimization_interval));
    let mut run_count = 0u64;

    log::info!("🔄 Optimization task started (advisory-only mode)");

    loop {
        interval.tick().await;
        run_count += 1;

        log::debug!("🔄 Running optimization task (run #{})", run_count);

        // ----------------------------------------------------------------
        // Coordinator feed: actively poll every registered protocol adapter.
        // This refreshes the coordinator's aggregated metrics + history so
        // downstream consumers (this optimizer loop, the alert task via
        // get_current_metrics, and the /opportunities & /allocation API
        // endpoints) have real data. Without this poll the coordinator is
        // constructed but never fed, so get_current_metrics() always returns
        // None and optimization is a no-op.
        // ----------------------------------------------------------------
        let metrics = match coordinator.poll_all().await {
            Ok(m) => m,
            Err(e) => {
                log::error!("❌ Failed to poll protocol adapters: {}", e);
                continue;
            }
        };

        // Store metrics to database
        if let Err(e) = store_metrics_to_db(&db_pool, &metrics).await {
            log::error!("❌ Failed to store metrics: {}", e);
            continue;
        }

        log::debug!("✅ Metrics collected and stored successfully");

        // ----------------------------------------------------------------
        // ADVISORY-ONLY optimization.
        //
        // Invoke the EarningsOptimizer to (a) update its rolling history,
        // (b) analyze reallocation opportunities, and (c) compute the optimal
        // allocation plan. Results are ONLY LOGGED as recommendations.
        //
        // We deliberately do NOT call ReallocationEngine::execute_reallocation
        // (nor apply_allocation on any adapter) here — no workloads are moved.
        // Auto-applying these recommendations is DEFERRED future work.
        // ----------------------------------------------------------------
        let mut opt = optimizer.lock().await;

        // Feed the optimizer's rolling history so confidence scoring improves
        // as more samples accumulate.
        opt.update_metrics(metrics.clone());

        let opportunities = match opt.analyze_opportunities(&metrics) {
            Ok(opps) => opps,
            Err(e) => {
                log::error!("❌ Optimizer failed to analyze opportunities: {}", e);
                continue;
            }
        };

        let plan = opt.calculate_optimal_allocation(&metrics).ok();

        if opportunities.is_empty() {
            log::debug!(
                "🔎 [ADVISORY] No reallocation opportunities identified (run #{})",
                run_count
            );
        } else {
            log::info!(
                "💡 [ADVISORY] {} reallocation opportunit{} identified (run #{}) — advisory-only, nothing applied",
                opportunities.len(),
                if opportunities.len() == 1 { "y" } else { "ies" },
                run_count
            );
            for (idx, opp) in opportunities.iter().enumerate() {
                log::info!(
                    "   #{} [ADVISORY] recommend moving allocation {} -> {}: +${:.4}/hr (current ${:.4}/hr -> projected ${:.4}/hr, confidence {:.0}%) — NOT applied",
                    idx + 1,
                    opp.from_protocol,
                    opp.to_protocol,
                    opp.earnings_improvement,
                    opp.current_rate,
                    opp.projected_rate,
                    opp.confidence * 100.0,
                );
            }

            // Surface the optimizer's own verdict on whether the best
            // opportunity clears its internal thresholds. Informational only
            // in advisory-only mode — no action is taken regardless.
            if opt.should_reallocate(&opportunities, plan.as_ref()) {
                log::info!(
                    "✅ [ADVISORY] Optimizer recommends reallocation (best opportunity clears thresholds) — auto-apply deferred, no action taken"
                );
            } else {
                log::debug!("[ADVISORY] Optimizer does not recommend reallocation at this time");
            }
        }

        if let Some(plan) = plan {
            log::info!(
                "📋 [ADVISORY] Optimal allocation plan: est. +${:.4}/hr, net benefit ${:.4}, ROI {:.1}%, confidence {:.0}% — advisory-only, not applied (auto-apply deferred)",
                plan.estimated_improvement,
                plan.net_benefit,
                plan.roi_percent,
                plan.confidence * 100.0,
            );
        }
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
    severity_base.clamp(0.0, 100.0)
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
