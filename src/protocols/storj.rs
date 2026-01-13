/// Storj Decentralized Storage Protocol Adapter
///
/// Storj is a decentralized cloud storage network where users can earn rewards
/// by providing storage space and bandwidth.
///
/// This adapter manages:
/// - Connection to Storj satellite network
/// - Storage node participation
/// - Earnings tracking from storage services
/// - Resource allocation and utilization

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

/// Storj protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorjConfig {
    /// API endpoint for Storj
    pub api_endpoint: String,
    /// Node ID/authorization token
    pub node_id: String,
    /// Wallet address for earnings
    pub wallet_address: String,
    /// Allocated storage in GB
    pub allocated_storage_gb: f64,
    /// Minimum allocation percent
    pub min_allocation_percent: f64,
    /// Maximum allocation percent
    pub max_allocation_percent: f64,
}

impl Default for StorjConfig {
    fn default() -> Self {
        Self {
            api_endpoint: "https://satellite.storj.io/api".to_string(),
            node_id: String::new(),
            wallet_address: String::new(),
            allocated_storage_gb: 1000.0,
            min_allocation_percent: 10.0,
            max_allocation_percent: 50.0,
        }
    }
}

// ============================================================================
// INTERNAL STATE
// ============================================================================

/// Storj metrics tracked internally
#[derive(Debug, Clone)]
struct StorjMetrics {
    storage_used_gb: f64,
    bytes_downloaded: u64,
    bytes_uploaded: u64,
    uptime_hours: u64,
    connected_at: Option<DateTime<Utc>>,
    repair_count: u32,
}

impl Default for StorjMetrics {
    fn default() -> Self {
        Self {
            storage_used_gb: 0.0,
            bytes_downloaded: 0,
            bytes_uploaded: 0,
            uptime_hours: 0,
            connected_at: None,
            repair_count: 0,
        }
    }
}

// ============================================================================
// ADAPTER IMPLEMENTATION
// ============================================================================

/// Storj Decentralized Storage Protocol Adapter
pub struct StorjAdapter {
    config: StorjConfig,
    status: Arc<RwLock<ConnectionStatus>>,
    allocation: Arc<RwLock<AllocationStrategy>>,
    metrics: Arc<RwLock<StorjMetrics>>,
}

impl StorjAdapter {
    /// Create a new Storj adapter
    pub fn new(config: StorjConfig) -> Self {
        let allocation = AllocationStrategy {
            cpu_cores: 1,
            memory_gb: 2.0,
            storage_gb: config.allocated_storage_gb,
            bandwidth_mbps: 50.0,
            allocation_percent: 30.0,
        };

        Self {
            config,
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            allocation: Arc::new(RwLock::new(allocation)),
            metrics: Arc::new(RwLock::new(StorjMetrics::default())),
        }
    }

    /// Update metrics with uptime
    async fn update_uptime(&self) {
        let mut metrics = self.metrics.write().await;
        if let Some(connected_at) = metrics.connected_at {
            let duration = Utc::now().signed_duration_since(connected_at);
            metrics.uptime_hours = duration.num_hours() as u64;
        }
    }

    /// Simulate earning for demonstration
    async fn calculate_current_earnings(&self) -> f64 {
        let metrics = self.metrics.read().await;
        let allocation = self.allocation.read().await;

        // Simulate earnings: based on storage and bandwidth
        let storage_factor = metrics.storage_used_gb / self.config.allocated_storage_gb;
        let base_rate = 0.30; // $0.30 per hour for full storage

        let uptime_hours = metrics.uptime_hours as f64;
        (base_rate * storage_factor * allocation.allocation_percent / 100.0) * uptime_hours
    }
}

#[async_trait]
impl ProtocolAdapter for StorjAdapter {
    fn protocol_name(&self) -> &str {
        "Storj Network"
    }

    async fn connect(&mut self) -> ProtocolResult<()> {
        if self.config.node_id.is_empty() {
            return Err(ProtocolError::AuthenticationError(
                "Node ID not configured".to_string(),
            ));
        }

        if self.config.wallet_address.is_empty() {
            return Err(ProtocolError::ConfigurationError(
                "Wallet address not configured".to_string(),
            ));
        }

        // Simulate connection
        *self.status.write().await = ConnectionStatus::Connecting;

        // Simulate connection delay
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        *self.status.write().await = ConnectionStatus::Connected;

        let mut metrics = self.metrics.write().await;
        metrics.connected_at = Some(Utc::now());
        metrics.uptime_hours = 0;
        metrics.storage_used_gb = 50.0; // Start with 50GB used

        tracing::info!("Connected to Storj Network");
        Ok(())
    }

    async fn disconnect(&mut self) -> ProtocolResult<()> {
        if *self.status.read().await == ConnectionStatus::Disconnected {
            return Ok(());
        }

        *self.status.write().await = ConnectionStatus::Disconnected;

        let mut metrics = self.metrics.write().await;
        metrics.connected_at = None;

        tracing::info!("Disconnected from Storj Network");
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        ConnectionStatus::Connected // Placeholder
    }

    async fn get_current_earnings(&self) -> ProtocolResult<EarningsData> {
        let earnings_usd = self.calculate_current_earnings().await;
        let metrics = self.metrics.read().await;

        let mut metric_map = HashMap::new();
        metric_map.insert("storage_used_gb".to_string(), metrics.storage_used_gb);
        metric_map.insert("uptime_hours".to_string(), metrics.uptime_hours as f64);
        metric_map.insert("repair_count".to_string(), metrics.repair_count as f64);

        Ok(EarningsData {
            timestamp: Utc::now(),
            amount_usd: earnings_usd,
            protocol_id: "storj".to_string(),
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

            // Simulate varying earnings with some variance
            let variance = 1.0 - (i as f64 / hours as f64) * 0.2;
            let amount = current_earnings * variance;

            earnings.push(EarningsData {
                timestamp,
                amount_usd: amount,
                protocol_id: "storj".to_string(),
                metrics: HashMap::new(),
            });
        }

        Ok(earnings)
    }

    async fn get_resource_usage(&self) -> ProtocolResult<ResourceMetrics> {
        self.update_uptime().await;
        let metrics = self.metrics.read().await;

        Ok(ResourceMetrics {
            cpu_percent: 10.0,
            memory_mb: 256.0,
            bandwidth_mbps: 25.0,
            storage_gb: metrics.storage_used_gb,
            uptime_seconds: metrics.uptime_hours * 3600,
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

        // Validate storage doesn't exceed allocated
        if strategy.storage_gb > self.config.allocated_storage_gb {
            return Err(ProtocolError::AllocationError(
                "Storage allocation exceeds limit".to_string(),
            ));
        }

        *self.allocation.write().await = strategy;
        tracing::info!("Applied allocation strategy to Storj");
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
            "storage_used_gb".into(),
            serde_json::json!(metrics.storage_used_gb),
        );
        health_metrics.insert(
            "uptime_hours".into(),
            serde_json::json!(metrics.uptime_hours),
        );

        Ok(HealthStatus {
            is_healthy,
            connection_status: status,
            last_operation: Some(Utc::now()),
            error_message: if is_healthy {
                None
            } else {
                Some("Not connected to Storj".to_string())
            },
            metrics: health_metrics,
        })
    }

    fn get_config(&self) -> serde_json::Value {
        serde_json::json!({
            "protocol": "storj",
            "api_endpoint": self.config.api_endpoint,
            "allocated_storage_gb": self.config.allocated_storage_gb,
            "min_allocation_percent": self.config.min_allocation_percent,
            "max_allocation_percent": self.config.max_allocation_percent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storj_creation() {
        let config = StorjConfig {
            node_id: "test_node".to_string(),
            wallet_address: "0x123...".to_string(),
            ..Default::default()
        };
        let adapter = StorjAdapter::new(config);
        assert_eq!(adapter.protocol_name(), "Storj Network");
    }

    #[tokio::test]
    async fn test_storj_connect_requires_config() {
        let config = StorjConfig::default();
        let mut adapter = StorjAdapter::new(config);
        assert!(adapter.connect().await.is_err());
    }

    #[tokio::test]
    async fn test_storj_connect_disconnect() {
        let config = StorjConfig {
            node_id: "test_node".to_string(),
            wallet_address: "0x123...".to_string(),
            ..Default::default()
        };
        let mut adapter = StorjAdapter::new(config);

        assert!(adapter.connect().await.is_ok());
        assert!(adapter.disconnect().await.is_ok());
    }

    #[tokio::test]
    async fn test_storj_earnings() {
        let config = StorjConfig {
            node_id: "test_node".to_string(),
            wallet_address: "0x123...".to_string(),
            ..Default::default()
        };
        let mut adapter = StorjAdapter::new(config);
        adapter.connect().await.unwrap();

        let earnings = adapter.get_current_earnings().await.unwrap();
        assert_eq!(earnings.protocol_id, "storj");
        assert!(earnings.amount_usd >= 0.0);
    }

    #[tokio::test]
    async fn test_storj_allocation_validation() {
        let config = StorjConfig {
            node_id: "test_node".to_string(),
            wallet_address: "0x123...".to_string(),
            allocated_storage_gb: 100.0,
            ..Default::default()
        };
        let mut adapter = StorjAdapter::new(config);

        // Valid allocation
        let strategy = AllocationStrategy {
            cpu_cores: 1,
            memory_gb: 2.0,
            storage_gb: 50.0,
            bandwidth_mbps: 50.0,
            allocation_percent: 30.0,
        };
        assert!(adapter.apply_allocation(strategy).await.is_ok());

        // Storage exceeds allocation
        let invalid_strategy = AllocationStrategy {
            cpu_cores: 1,
            memory_gb: 2.0,
            storage_gb: 150.0,
            bandwidth_mbps: 50.0,
            allocation_percent: 30.0,
        };
        assert!(adapter.apply_allocation(invalid_strategy).await.is_err());
    }
}
