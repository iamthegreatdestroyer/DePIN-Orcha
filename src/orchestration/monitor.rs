/// Real-Time Monitor
///
/// Provides dashboard metrics, alerting, and performance reporting.
/// Generates real-time insights and historical analysis.

use super::{
    AggregatedMetrics, Alert, AlertType, DashboardSnapshot, OptimizationOpportunity,
    PerformanceReport, OrchestrationError, OrchestrationResult,
};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// MONITOR CONFIGURATION
// ============================================================================

/// Monitor configuration
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    /// Earnings threshold for alerts
    pub low_earnings_threshold: f64,
    /// Optimization potential threshold
    pub optimization_threshold: f64,
    /// Connection timeout duration
    pub connection_timeout: Duration,
    /// Maximum alerts to keep
    pub max_alerts: usize,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            low_earnings_threshold: 5.0,
            optimization_threshold: 0.25,
            connection_timeout: Duration::minutes(5),
            max_alerts: 1000,
        }
    }
}

// ============================================================================
// MONITOR IMPLEMENTATION
// ============================================================================

/// Real-Time Monitor
///
/// Monitors protocol performance and generates alerts.
pub struct RealtimeMonitor {
    config: MonitorConfig,
    alerts: Arc<RwLock<Vec<Alert>>>,
    metrics_snapshots: Arc<RwLock<Vec<AggregatedMetrics>>>,
    last_dashboard_update: Arc<RwLock<Option<DateTime<Utc>>>>,
}

impl RealtimeMonitor {
    /// Create a new monitor
    pub fn new(config: MonitorConfig) -> Self {
        Self {
            config,
            alerts: Arc::new(RwLock::new(Vec::new())),
            metrics_snapshots: Arc::new(RwLock::new(Vec::new())),
            last_dashboard_update: Arc::new(RwLock::new(None)),
        }
    }

    /// Get dashboard metrics
    pub async fn get_dashboard_metrics(
        &self,
        current_metrics: &AggregatedMetrics,
        optimal_allocation: &HashMap<String, f64>,
        opportunities: &[OptimizationOpportunity],
    ) -> OrchestrationResult<DashboardSnapshot> {
        // Calculate next reallocation time (suggest in 2 hours if opportunity exists)
        let next_reallocation = if !opportunities.is_empty() {
            Some(std::time::Duration::from_secs(2 * 3600))
        } else {
            None
        };

        // Get recent changes from metrics history
        let recent_changes = self
            .get_recent_allocation_changes(24)
            .await;

        let snapshot = DashboardSnapshot {
            timestamp: Utc::now(),
            total_earnings_per_hour: current_metrics.total_earnings_per_hour,
            earnings_by_protocol: current_metrics.earnings_by_protocol.clone(),
            current_allocation: current_metrics.allocation_by_protocol.clone(),
            optimal_allocation: optimal_allocation.clone(),
            optimization_opportunity: opportunities.first().cloned(),
            next_reallocation_in: next_reallocation,
            connection_status: current_metrics.connection_status.clone(),
            recent_changes,
        };

        *self.last_dashboard_update.write().await = Some(Utc::now());

        Ok(snapshot)
    }

    /// Check for alerts
    pub async fn check_alerts(
        &self,
        current_metrics: &AggregatedMetrics,
        opportunities: &[OptimizationOpportunity],
    ) -> OrchestrationResult<Vec<Alert>> {
        let mut new_alerts = Vec::new();

        // Check low earnings
        if current_metrics.total_earnings_per_hour < self.config.low_earnings_threshold {
            new_alerts.push(Alert {
                timestamp: Utc::now(),
                alert_type: AlertType::LowEarnings {
                    current_rate: current_metrics.total_earnings_per_hour,
                    threshold: self.config.low_earnings_threshold,
                },
                severity: 0.6,
                message: format!(
                    "Earnings {:.2}/hr below threshold {:.2}/hr",
                    current_metrics.total_earnings_per_hour, self.config.low_earnings_threshold
                ),
                acknowledged: false,
            });
        }

        // Check disconnected protocols
        for (protocol, connected) in &current_metrics.connection_status {
            if !connected {
                new_alerts.push(Alert {
                    timestamp: Utc::now(),
                    alert_type: AlertType::ProtocolDisconnected {
                        protocol: protocol.clone(),
                    },
                    severity: 0.9,
                    message: format!("Protocol {} disconnected", protocol),
                    acknowledged: false,
                });
            }
        }

        // Check optimization opportunities
        if let Some(best_opp) = opportunities.first() {
            if best_opp.earnings_improvement > self.config.optimization_threshold {
                new_alerts.push(Alert {
                    timestamp: Utc::now(),
                    alert_type: AlertType::ReallocationOpportunity {
                        opportunity: best_opp.clone(),
                    },
                    severity: 0.4,
                    message: format!(
                        "Optimization opportunity: {:.2}/hr improvement",
                        best_opp.earnings_improvement
                    ),
                    acknowledged: false,
                });
            }
        }

        // Check high resource usage
        if current_metrics.resource_utilization.cpu_percent > 95.0 {
            new_alerts.push(Alert {
                timestamp: Utc::now(),
                alert_type: AlertType::ResourceContention {
                    resource: "CPU".to_string(),
                },
                severity: 0.8,
                message: "CPU utilization critically high".to_string(),
                acknowledged: false,
            });
        }

        // Add new alerts to history
        let mut alerts = self.alerts.write().await;
        for alert in &new_alerts {
            alerts.push(alert.clone());
        }

        // Trim old alerts
        if alerts.len() > self.config.max_alerts {
            alerts.drain(0..alerts.len() - self.config.max_alerts);
        }

        Ok(new_alerts)
    }

    /// Generate performance report
    pub async fn generate_report(
        &self,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> OrchestrationResult<PerformanceReport> {
        let snapshots = self.metrics_snapshots.read().await;

        let period_metrics: Vec<_> = snapshots
            .iter()
            .filter(|m| m.timestamp >= period_start && m.timestamp <= period_end)
            .collect();

        if period_metrics.is_empty() {
            return Err(OrchestrationError::MonitoringError(
                "No metrics for period".to_string(),
            ));
        }

        // Calculate totals
        let total_earnings: f64 = period_metrics.iter()
            .map(|m| m.total_earnings_per_hour)
            .sum();

        let average_hourly = total_earnings / period_metrics.len() as f64;

        // Earnings by protocol
        let mut earnings_by_protocol: HashMap<String, f64> = HashMap::new();
        for metric in &period_metrics {
            for (protocol, earning) in &metric.earnings_by_protocol {
                *earnings_by_protocol.entry(protocol.clone()).or_insert(0.0) += earning;
            }
        }

        let allocation_changes = self.get_recent_allocation_changes(
            (period_end - period_start).num_hours()
        ).await;

        let total_improvement: f64 = allocation_changes.iter()
            .map(|c| c.earnings_impact)
            .sum();

        let successful_optimizations = allocation_changes.len() as u32;

        let uptime_percent = self.calculate_uptime(&period_metrics);

        Ok(PerformanceReport {
            period_start,
            period_end,
            total_earnings,
            average_hourly_earnings: average_hourly,
            earnings_by_protocol,
            allocation_changes,
            total_improvement,
            successful_optimizations,
            uptime_percent,
        })
    }

    /// Get earnings trends
    pub async fn get_earnings_trends(
        &self,
        hours: i64,
    ) -> OrchestrationResult<Vec<(DateTime<Utc>, f64)>> {
        let cutoff = Utc::now() - Duration::hours(hours);
        let snapshots = self.metrics_snapshots.read().await;

        Ok(snapshots
            .iter()
            .filter(|m| m.timestamp > cutoff)
            .map(|m| (m.timestamp, m.total_earnings_per_hour))
            .collect())
    }

    /// Export metrics to JSON
    pub async fn export_metrics(
        &self,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> OrchestrationResult<String> {
        let snapshots = self.metrics_snapshots.read().await;

        let filtered: Vec<_> = snapshots
            .iter()
            .filter(|m| m.timestamp >= period_start && m.timestamp <= period_end)
            .collect();

        serde_json::to_string_pretty(&filtered)
            .map_err(|e| OrchestrationError::DataError(e.to_string()))
    }

    /// Update metrics snapshot
    pub async fn update_snapshot(&self, metrics: AggregatedMetrics) {
        let mut snapshots = self.metrics_snapshots.write().await;
        snapshots.push(metrics);

        // Keep last 10,000 snapshots
        if snapshots.len() > 10000 {
            snapshots.drain(0..snapshots.len() - 10000);
        }
    }

    /// Get alert history
    pub async fn get_alert_history(&self) -> Vec<Alert> {
        self.alerts.read().await.clone()
    }

    /// Acknowledge alert
    pub async fn acknowledge_alert(&self, alert_timestamp: DateTime<Utc>) -> OrchestrationResult<()> {
        let mut alerts = self.alerts.write().await;
        for alert in alerts.iter_mut() {
            if alert.timestamp == alert_timestamp {
                alert.acknowledged = true;
                return Ok(());
            }
        }
        Err(OrchestrationError::MonitoringError("Alert not found".to_string()))
    }

    /// Clear old data
    pub async fn cleanup_old_data(&self, retention_days: i64) {
        let cutoff = Utc::now() - Duration::days(retention_days);

        let mut snapshots = self.metrics_snapshots.write().await;
        snapshots.retain(|m| m.timestamp > cutoff);

        let mut alerts = self.alerts.write().await;
        alerts.retain(|a| a.timestamp > cutoff);
    }

    /// Get recent allocation changes
    async fn get_recent_allocation_changes(&self, hours: i64) -> Vec<super::AllocationChange> {
        let cutoff = Utc::now() - Duration::hours(hours);
        let snapshots = self.metrics_snapshots.read().await;

        // Extract allocation changes from metrics history
        let mut changes = Vec::new();
        let mut prev_allocations: HashMap<String, f64> = HashMap::new();

        for snapshot in snapshots.iter() {
            if snapshot.timestamp < cutoff {
                continue;
            }

            for (protocol, allocation) in &snapshot.allocation_by_protocol {
                if let Some(prev_alloc) = prev_allocations.get(protocol) {
                    if (allocation - prev_alloc).abs() > 0.1 {
                        changes.push(super::AllocationChange {
                            timestamp: snapshot.timestamp,
                            protocol: protocol.clone(),
                            old_allocation: *prev_alloc,
                            new_allocation: *allocation,
                            reason: "Automatic reallocation".to_string(),
                            earnings_impact: 0.0,
                        });
                    }
                }
                prev_allocations.insert(protocol.clone(), *allocation);
            }
        }

        changes
    }

    /// Calculate uptime percentage
    fn calculate_uptime(&self, metrics: &[&AggregatedMetrics]) -> f64 {
        if metrics.is_empty() {
            return 0.0;
        }

        let connected_count = metrics
            .iter()
            .filter(|m| m.connection_status.values().all(|&connected| connected))
            .count();

        (connected_count as f64 / metrics.len() as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metrics() -> AggregatedMetrics {
        let mut earnings = HashMap::new();
        earnings.insert("streamr".to_string(), 3.0);
        earnings.insert("storj".to_string(), 4.0);

        let mut allocation = HashMap::new();
        allocation.insert("streamr".to_string(), 50.0);
        allocation.insert("storj".to_string(), 50.0);

        let mut status = HashMap::new();
        status.insert("streamr".to_string(), true);
        status.insert("storj".to_string(), true);

        AggregatedMetrics {
            timestamp: Utc::now(),
            total_earnings_per_hour: 7.0,
            earnings_by_protocol: earnings,
            allocation_by_protocol: allocation,
            resource_utilization: super::super::ResourceUtilization {
                cpu_percent: 50.0,
                memory_percent: 60.0,
                bandwidth_percent: 40.0,
                storage_percent: 30.0,
            },
            connection_status: status,
        }
    }

    #[test]
    fn test_monitor_creation() {
        let monitor = RealtimeMonitor::new(MonitorConfig::default());
        assert!(monitor.config.max_alerts > 0);
    }

    #[tokio::test]
    async fn test_check_low_earnings_alert() {
        let monitor = RealtimeMonitor::new(MonitorConfig {
            low_earnings_threshold: 10.0,
            ..Default::default()
        });

        let metrics = create_test_metrics();
        let alerts = monitor.check_alerts(&metrics, &[]).await.unwrap();

        assert!(!alerts.is_empty());
    }

    #[tokio::test]
    async fn test_dashboard_snapshot() {
        let monitor = RealtimeMonitor::new(MonitorConfig::default());
        let metrics = create_test_metrics();
        let allocation = HashMap::new();

        let snapshot = monitor
            .get_dashboard_metrics(&metrics, &allocation, &[])
            .await
            .unwrap();

        assert_eq!(snapshot.total_earnings_per_hour, 7.0);
    }

    #[test]
    fn test_monitor_config_defaults() {
        let config = MonitorConfig::default();
        assert_eq!(config.max_alerts, 1000);
    }
}
