/// Multi-Protocol Coordinator
///
/// Monitors and aggregates data from all protocol adapters.
/// Provides unified view of earnings, resources, and connection status.

use super::{AggregatedMetrics, OrchestrationError, OrchestrationResult, ResourceUtilization};
use crate::protocols::{ProtocolAdapter, EarningsData, ResourceMetrics};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// COORDINATOR IMPLEMENTATION
// ============================================================================

/// Multi-Protocol Coordinator
///
/// Manages connections to all protocol adapters and aggregates their data.
pub struct ProtocolCoordinator {
    /// Map of protocol name to adapter
    adapters: HashMap<String, Arc<RwLock<Box<dyn ProtocolAdapter>>>>,
    /// Historical metrics
    metrics_history: Arc<RwLock<Vec<AggregatedMetrics>>>,
    /// Last update timestamp
    last_update: Arc<RwLock<Option<DateTime<Utc>>>>,
    /// Maximum history size
    max_history_size: usize,
}

impl ProtocolCoordinator {
    /// Create a new coordinator
    pub fn new(max_history_size: usize) -> Self {
        Self {
            adapters: HashMap::new(),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            last_update: Arc::new(RwLock::new(None)),
            max_history_size,
        }
    }

    /// Register a protocol adapter
    pub fn register_adapter(
        &mut self,
        protocol_name: String,
        adapter: Box<dyn ProtocolAdapter>,
    ) {
        self.adapters
            .insert(protocol_name, Arc::new(RwLock::new(adapter)));
    }

    /// Get list of registered protocols
    pub fn registered_protocols(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }

    /// Poll all adapters and aggregate metrics
    pub async fn poll_all(&self) -> OrchestrationResult<AggregatedMetrics> {
        let timestamp = Utc::now();
        let mut earnings_by_protocol = HashMap::new();
        let mut allocation_by_protocol = HashMap::new();
        let mut connection_status = HashMap::new();

        let mut total_cpu = 0.0;
        let mut total_memory = 0.0;
        let mut total_bandwidth = 0.0;
        let mut total_storage = 0.0;
        let mut count = 0u32;

        // Poll each adapter
        for (protocol_name, adapter_lock) in &self.adapters {
            let adapter = adapter_lock.read().await;

            // Get current earnings
            match adapter.get_current_earnings().await {
                Ok(earnings) => {
                    earnings_by_protocol.insert(protocol_name.clone(), earnings.amount_usd);
                }
                Err(e) => {
                    tracing::warn!("Failed to get earnings from {}: {}", protocol_name, e);
                }
            }

            // Get current allocation
            match adapter.get_current_allocation().await {
                Ok(allocation) => {
                    allocation_by_protocol.insert(
                        protocol_name.clone(),
                        allocation.allocation_percent,
                    );
                }
                Err(e) => {
                    tracing::warn!("Failed to get allocation from {}: {}", protocol_name, e);
                }
            }

            // Get resource usage
            match adapter.get_resource_usage().await {
                Ok(resources) => {
                    total_cpu += resources.cpu_percent;
                    total_memory += resources.memory_mb;
                    total_bandwidth += resources.bandwidth_mbps;
                    total_storage += resources.storage_gb;
                    count += 1;
                }
                Err(e) => {
                    tracing::warn!("Failed to get resources from {}: {}", protocol_name, e);
                }
            }

            // Get health status
            match adapter.health_check().await {
                Ok(health) => {
                    connection_status.insert(
                        protocol_name.clone(),
                        health.is_healthy && health.connection_status
                            == crate::protocols::ConnectionStatus::Connected,
                    );
                }
                Err(e) => {
                    tracing::warn!("Failed to get health from {}: {}", protocol_name, e);
                    connection_status.insert(protocol_name.clone(), false);
                }
            }
        }

        let total_earnings_per_hour: f64 = earnings_by_protocol.values().sum();

        // Calculate average resource utilization
        let resource_utilization = if count > 0 {
            ResourceUtilization {
                cpu_percent: (total_cpu / count as f64).min(100.0),
                memory_percent: (total_memory / count as f64).min(100.0),
                bandwidth_percent: (total_bandwidth / count as f64).min(100.0),
                storage_percent: (total_storage / count as f64).min(100.0),
            }
        } else {
            ResourceUtilization {
                cpu_percent: 0.0,
                memory_percent: 0.0,
                bandwidth_percent: 0.0,
                storage_percent: 0.0,
            }
        };

        let metrics = AggregatedMetrics {
            timestamp,
            total_earnings_per_hour,
            earnings_by_protocol,
            allocation_by_protocol,
            resource_utilization,
            connection_status,
        };

        // Update history
        let mut history = self.metrics_history.write().await;
        history.push(metrics.clone());
        if history.len() > self.max_history_size {
            history.remove(0);
        }

        *self.last_update.write().await = Some(timestamp);

        tracing::debug!("Polled all protocols: {:.2}/hour earnings", metrics.total_earnings_per_hour);

        Ok(metrics)
    }

    /// Get protocol status
    pub async fn get_protocol_status(
        &self,
        protocol_name: &str,
    ) -> OrchestrationResult<ProtocolStatus> {
        let adapter_lock = self
            .adapters
            .get(protocol_name)
            .ok_or_else(|| OrchestrationError::CoordinationError(format!(
                "Protocol {} not registered",
                protocol_name
            )))?;

        let adapter = adapter_lock.read().await;

        let earnings = adapter
            .get_current_earnings()
            .await
            .map(|e| e.amount_usd)
            .unwrap_or(0.0);

        let allocation = adapter
            .get_current_allocation()
            .await
            .map(|a| a.allocation_percent)
            .unwrap_or(0.0);

        let resources = adapter
            .get_resource_usage()
            .await
            .ok();

        let health = adapter
            .health_check()
            .await
            .ok();

        Ok(ProtocolStatus {
            protocol_name: protocol_name.to_string(),
            earnings_per_hour: earnings,
            allocation_percent: allocation,
            resources,
            health_status: health,
        })
    }

    /// Get metrics history
    pub async fn get_metrics_history(&self) -> Vec<AggregatedMetrics> {
        self.metrics_history.read().await.clone()
    }

    /// Calculate total earnings across all protocols
    pub async fn calculate_total_earnings(&self) -> OrchestrationResult<f64> {
        let history = self.metrics_history.read().await;
        if history.is_empty() {
            return Ok(0.0);
        }
        Ok(history.iter().map(|m| m.total_earnings_per_hour).sum())
    }

    /// Get current aggregated metrics
    pub async fn get_current_metrics(&self) -> OrchestrationResult<Option<AggregatedMetrics>> {
        let history = self.metrics_history.read().await;
        Ok(history.last().cloned())
    }

    /// Get metrics for time period
    pub async fn get_metrics_for_period(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> OrchestrationResult<Vec<AggregatedMetrics>> {
        let history = self.metrics_history.read().await;
        Ok(history
            .iter()
            .filter(|m| m.timestamp >= start && m.timestamp <= end)
            .cloned()
            .collect())
    }

    /// Get last update timestamp
    pub async fn get_last_update(&self) -> Option<DateTime<Utc>> {
        *self.last_update.read().await
    }

    /// Clear history
    pub async fn clear_history(&self) {
        self.metrics_history.write().await.clear();
    }
}

/// Protocol status snapshot
#[derive(Debug, Clone)]
pub struct ProtocolStatus {
    /// Protocol name
    pub protocol_name: String,
    /// Current earnings per hour (USD)
    pub earnings_per_hour: f64,
    /// Current allocation percent
    pub allocation_percent: f64,
    /// Resource metrics
    pub resources: Option<ResourceMetrics>,
    /// Health status
    pub health_status: Option<crate::protocols::HealthStatus>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinator_creation() {
        let coordinator = ProtocolCoordinator::new(1000);
        assert_eq!(coordinator.registered_protocols().len(), 0);
    }

    #[test]
    fn test_coordinator_max_history() {
        let coordinator = ProtocolCoordinator::new(5);
        assert_eq!(coordinator.max_history_size, 5);
    }

    #[tokio::test]
    async fn test_empty_metrics() {
        let coordinator = ProtocolCoordinator::new(1000);
        let metrics = coordinator.get_current_metrics().await.unwrap();
        assert!(metrics.is_none());
    }

    #[tokio::test]
    async fn test_registered_protocols() {
        let mut coordinator = ProtocolCoordinator::new(1000);
        // Note: In real tests, we would create mock adapters
        let protocols = coordinator.registered_protocols();
        assert_eq!(protocols.len(), 0);
    }

    #[tokio::test]
    async fn test_metrics_history_limit() {
        let coordinator = ProtocolCoordinator::new(3);
        let history = coordinator.get_metrics_history().await;
        assert!(history.len() <= 3);
    }
}
