/// Golem Decentralized Compute Network Adapter
///
/// Golem is a decentralized compute network where users can earn rewards
/// by providing computational resources (CPU, GPU) to the network.
///
/// This adapter manages:
/// - Connection to Golem requestor network
/// - Provider node registration and management
/// - Computational task execution and earnings
/// - Resource allocation and optimization

use super::{
    AllocationStrategy, ConnectionStatus, EarningsData, HealthStatus, ProtocolAdapter,
    ProtocolError, ProtocolResult, ResourceMetrics,
};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Golem protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GolemConfig {
    /// Provider node URL
    pub provider_node_url: String,
    /// ETH wallet address
    pub eth_wallet: String,
    /// CPU cores available
    pub cpu_cores: u32,
    /// Memory available in GB
    pub memory_gb: f64,
    /// GPU enabled
    pub gpu_enabled: bool,
    /// Minimum allocation percent
    pub min_allocation_percent: f64,
    /// Maximum allocation percent
    pub max_allocation_percent: f64,
}

impl Default for GolemConfig {
    fn default() -> Self {
        Self {
            provider_node_url: "http://localhost:5001".to_string(),
            eth_wallet: String::new(),
            cpu_cores: 8,
            memory_gb: 16.0,
            gpu_enabled: false,
            min_allocation_percent: 10.0,
            max_allocation_percent: 40.0,
        }
    }
}

// ============================================================================
// INTERNAL STATE
// ============================================================================

/// Golem metrics tracked internally
#[derive(Debug, Clone)]
struct GolemMetrics {
    tasks_completed: u32,
    tasks_failed: u32,
    total_compute_hours: f64,
    connected_at: Option<DateTime<Utc>>,
    cpu_utilization_percent: f64,
    gpu_utilization_percent: f64,
}

impl Default for GolemMetrics {
    fn default() -> Self {
        Self {
            tasks_completed: 0,
            tasks_failed: 0,
            total_compute_hours: 0.0,
            connected_at: None,
            cpu_utilization_percent: 0.0,
            gpu_utilization_percent: 0.0,
        }
    }
}

// ============================================================================
// ADAPTER IMPLEMENTATION
// ============================================================================

/// Golem Decentralized Compute Network Adapter
pub struct GolemAdapter {
    config: GolemConfig,
    status: Arc<RwLock<ConnectionStatus>>,
    allocation: Arc<RwLock<AllocationStrategy>>,
    metrics: Arc<RwLock<GolemMetrics>>,
}

impl GolemAdapter {
    /// Create a new Golem adapter
    pub fn new(config: GolemConfig) -> Self {
        let allocation = AllocationStrategy {
            cpu_cores: (config.cpu_cores as f64 * 0.5) as u32,
            memory_gb: config.memory_gb * 0.5,
            storage_gb: 20.0,
            bandwidth_mbps: 100.0,
            allocation_percent: 30.0,
        };

        Self {
            config,
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            allocation: Arc::new(RwLock::new(allocation)),
            metrics: Arc::new(RwLock::new(GolemMetrics::default())),
        }
    }

    /// Update metrics
    async fn update_metrics(&self) {
        let mut metrics = self.metrics.write().await;

        if let Some(connected_at) = metrics.connected_at {
            let duration = Utc::now().signed_duration_since(connected_at);
            metrics.total_compute_hours = duration.num_seconds() as f64 / 3600.0;
        }

        // Simulate CPU and GPU utilization
        metrics.cpu_utilization_percent = 45.0 + (metrics.tasks_completed % 30) as f64;
        if self.config.gpu_enabled {
            metrics.gpu_utilization_percent = 30.0 + (metrics.tasks_completed % 20) as f64;
        }
    }

    /// Simulate earning for demonstration
    async fn calculate_current_earnings(&self) -> f64 {
        let metrics = self.metrics.read().await;
        let allocation = self.allocation.read().await;

        // Simulate earnings: based on compute hours and resource allocation
        let compute_rate = 1.20; // $1.20 per compute hour
        let gpu_multiplier = if self.config.gpu_enabled { 2.5 } else { 1.0 };

        let base_earnings = compute_rate * metrics.total_compute_hours;
        let allocated_earnings = base_earnings * (allocation.allocation_percent / 100.0);

        allocated_earnings * gpu_multiplier
    }
}

#[async_trait]
impl ProtocolAdapter for GolemAdapter {
    fn protocol_name(&self) -> &str {
        "Golem Network"
    }

    async fn connect(&mut self) -> ProtocolResult<()> {
        if self.config.eth_wallet.is_empty() {
            return Err(ProtocolError::AuthenticationError(
                "ETH wallet not configured".to_string(),
            ));
        }

        // Simulate connection
        *self.status.write().await = ConnectionStatus::Connecting;

        // Simulate connection delay
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        *self.status.write().await = ConnectionStatus::Connected;

        let mut metrics = self.metrics.write().await;
        metrics.connected_at = Some(Utc::now());
        metrics.tasks_completed = 0;
        metrics.tasks_failed = 0;

        tracing::info!("Connected to Golem Network");
        Ok(())
    }

    async fn disconnect(&mut self) -> ProtocolResult<()> {
        if *self.status.read().await == ConnectionStatus::Disconnected {
            return Ok(());
        }

        *self.status.write().await = ConnectionStatus::Disconnected;

        let mut metrics = self.metrics.write().await;
        metrics.connected_at = None;

        tracing::info!("Disconnected from Golem Network");
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        ConnectionStatus::Connected // Placeholder
    }

    async fn get_current_earnings(&self) -> ProtocolResult<EarningsData> {
        self.update_metrics().await;

        let earnings_usd = self.calculate_current_earnings().await;
        let metrics = self.metrics.read().await;
        let allocation = self.allocation.read().await;

        let mut metric_map = HashMap::new();
        metric_map.insert("tasks_completed".to_string(), metrics.tasks_completed as f64);
        metric_map.insert("tasks_failed".to_string(), metrics.tasks_failed as f64);
        metric_map.insert("cpu_cores_allocated".to_string(), allocation.cpu_cores as f64);
        metric_map.insert("memory_gb_allocated".to_string(), allocation.memory_gb);

        Ok(EarningsData {
            timestamp: Utc::now(),
            amount_usd: earnings_usd,
            protocol_id: "golem".to_string(),
            metrics: metric_map,
        })
    }

    async fn get_historical_earnings(&self, hours: u32) -> ProtocolResult<Vec<EarningsData>> {
        let mut earnings = Vec::new();
        let current_earnings = self.calculate_current_earnings().await;

        // Simulate historical data
        for i in 0..hours {
            let hours_ago = Duration::hours(i as i64);
            let timestamp = Utc::now() - hours_ago;

            // Simulate earnings growth over time
            let growth_factor = 1.0 + (i as f64 / hours as f64) * 0.5;
            let amount = current_earnings / growth_factor;

            earnings.push(EarningsData {
                timestamp,
                amount_usd: amount,
                protocol_id: "golem".to_string(),
                metrics: HashMap::new(),
            });
        }

        Ok(earnings)
    }

    async fn get_resource_usage(&self) -> ProtocolResult<ResourceMetrics> {
        self.update_metrics().await;
        let metrics = self.metrics.read().await;

        Ok(ResourceMetrics {
            cpu_percent: metrics.cpu_utilization_percent,
            memory_mb: (self.config.memory_gb * 1024.0) * 0.4,
            bandwidth_mbps: 75.0,
            storage_gb: 15.0,
            uptime_seconds: (metrics.total_compute_hours * 3600.0) as u64,
        })
    }

    async fn apply_allocation(&mut self, strategy: AllocationStrategy) -> ProtocolResult<()> {
        // Validate allocation
        if !(self.config.min_allocation_percent..=self.config.max_allocation_percent)
            .contains(&strategy.allocation_percent)
        {
            return Err(ProtocolError::AllocationError(format!(
                "Allocation must be between {} and {}%",
                self.config.min_allocation_percent, self.config.max_allocation_percent
            )));
        }

        // Validate CPU cores don't exceed available
        if strategy.cpu_cores > self.config.cpu_cores {
            return Err(ProtocolError::AllocationError(
                "CPU cores allocation exceeds available".to_string(),
            ));
        }

        // Validate memory doesn't exceed available
        if strategy.memory_gb > self.config.memory_gb {
            return Err(ProtocolError::AllocationError(
                "Memory allocation exceeds available".to_string(),
            ));
        }

        *self.allocation.write().await = strategy;
        tracing::info!("Applied allocation strategy to Golem");
        Ok(())
    }

    async fn get_current_allocation(&self) -> ProtocolResult<AllocationStrategy> {
        Ok(self.allocation.read().await.clone())
    }

    async fn health_check(&self) -> ProtocolResult<HealthStatus> {
        let status = *self.status.read().await;
        let is_healthy = status == ConnectionStatus::Connected;
        let metrics = self.metrics.read().await;

        let mut health_metrics = HashMap::new();
        health_metrics.insert(
            "tasks_completed".into(),
            serde_json::json!(metrics.tasks_completed),
        );
        health_metrics.insert(
            "cpu_utilization".into(),
            serde_json::json!(metrics.cpu_utilization_percent),
        );
        health_metrics.insert(
            "gpu_utilization".into(),
            serde_json::json!(metrics.gpu_utilization_percent),
        );

        Ok(HealthStatus {
            is_healthy,
            connection_status: status,
            last_operation: Some(Utc::now()),
            error_message: if is_healthy {
                None
            } else {
                Some("Not connected to Golem".to_string())
            },
            metrics: health_metrics,
        })
    }

    fn get_config(&self) -> serde_json::Value {
        serde_json::json!({
            "protocol": "golem",
            "provider_node_url": self.config.provider_node_url,
            "cpu_cores": self.config.cpu_cores,
            "memory_gb": self.config.memory_gb,
            "gpu_enabled": self.config.gpu_enabled,
            "min_allocation_percent": self.config.min_allocation_percent,
            "max_allocation_percent": self.config.max_allocation_percent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_golem_creation() {
        let config = GolemConfig {
            eth_wallet: "0x123...".to_string(),
            ..Default::default()
        };
        let adapter = GolemAdapter::new(config);
        assert_eq!(adapter.protocol_name(), "Golem Network");
    }

    #[tokio::test]
    async fn test_golem_connect_requires_wallet() {
        let config = GolemConfig::default();
        let mut adapter = GolemAdapter::new(config);
        assert!(adapter.connect().await.is_err());
    }

    #[tokio::test]
    async fn test_golem_connect_disconnect() {
        let config = GolemConfig {
            eth_wallet: "0x123...".to_string(),
            ..Default::default()
        };
        let mut adapter = GolemAdapter::new(config);

        assert!(adapter.connect().await.is_ok());
        assert!(adapter.disconnect().await.is_ok());
    }

    #[tokio::test]
    async fn test_golem_earnings() {
        let config = GolemConfig {
            eth_wallet: "0x123...".to_string(),
            ..Default::default()
        };
        let mut adapter = GolemAdapter::new(config);
        adapter.connect().await.unwrap();

        let earnings = adapter.get_current_earnings().await.unwrap();
        assert_eq!(earnings.protocol_id, "golem");
        assert!(earnings.amount_usd >= 0.0);
    }

    #[tokio::test]
    async fn test_golem_allocation_validation() {
        let config = GolemConfig {
            eth_wallet: "0x123...".to_string(),
            cpu_cores: 8,
            memory_gb: 16.0,
            ..Default::default()
        };
        let mut adapter = GolemAdapter::new(config);

        // Valid allocation
        let strategy = AllocationStrategy {
            cpu_cores: 4,
            memory_gb: 8.0,
            storage_gb: 20.0,
            bandwidth_mbps: 100.0,
            allocation_percent: 25.0,
        };
        assert!(adapter.apply_allocation(strategy).await.is_ok());

        // CPU exceeds allocation
        let invalid_strategy = AllocationStrategy {
            cpu_cores: 16,
            memory_gb: 8.0,
            storage_gb: 20.0,
            bandwidth_mbps: 100.0,
            allocation_percent: 25.0,
        };
        assert!(adapter.apply_allocation(invalid_strategy).await.is_err());
    }

    #[tokio::test]
    async fn test_golem_resource_usage() {
        let config = GolemConfig {
            eth_wallet: "0x123...".to_string(),
            cpu_cores: 8,
            memory_gb: 16.0,
            gpu_enabled: true,
            ..Default::default()
        };
        let mut adapter = GolemAdapter::new(config);
        adapter.connect().await.unwrap();

        let resources = adapter.get_resource_usage().await.unwrap();
        assert!(resources.cpu_percent > 0.0);
        assert!(resources.memory_mb > 0.0);
    }
}
