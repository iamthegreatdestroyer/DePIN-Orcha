/// Streamr Network Protocol Adapter
///
/// Streamr is a real-time data streaming network where users can earn rewards
/// for providing network bandwidth and node participation.
///
/// This adapter manages:
/// - Connection to Streamr broker network
/// - Data stream publishing and subscription
/// - Earnings tracking from network participation
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

/// Streamr protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamrConfig {
    /// API endpoint for Streamr
    pub api_endpoint: String,
    /// Private key for authentication
    pub private_key: String,
    /// Stream IDs to subscribe to
    pub streams: Vec<String>,
    /// Publishing interval in seconds
    pub publish_interval_seconds: u64,
    /// Minimum allocation percent
    pub min_allocation_percent: f64,
    /// Maximum allocation percent
    pub max_allocation_percent: f64,
}

impl Default for StreamrConfig {
    fn default() -> Self {
        Self {
            api_endpoint: "https://core.streamr.network/api/v1".to_string(),
            private_key: String::new(),
            streams: vec![],
            publish_interval_seconds: 60,
            min_allocation_percent: 5.0,
            max_allocation_percent: 30.0,
        }
    }
}

// ============================================================================
// INTERNAL STATE
// ============================================================================

/// Streamr metrics tracked internally
#[derive(Debug, Clone)]
struct StreamrMetrics {
    messages_published: u64,
    bytes_published: u64,
    last_publish_time: Option<DateTime<Utc>>,
    connection_uptime_seconds: u64,
    connected_at: Option<DateTime<Utc>>,
}

impl Default for StreamrMetrics {
    fn default() -> Self {
        Self {
            messages_published: 0,
            bytes_published: 0,
            last_publish_time: None,
            connection_uptime_seconds: 0,
            connected_at: None,
        }
    }
}

// ============================================================================
// ADAPTER IMPLEMENTATION
// ============================================================================

/// Streamr Network Protocol Adapter
pub struct StreamrAdapter {
    config: StreamrConfig,
    status: Arc<RwLock<ConnectionStatus>>,
    allocation: Arc<RwLock<AllocationStrategy>>,
    metrics: Arc<RwLock<StreamrMetrics>>,
}

impl StreamrAdapter {
    /// Create a new Streamr adapter
    pub fn new(config: StreamrConfig) -> Self {
        let allocation = AllocationStrategy {
            cpu_cores: 2,
            memory_gb: 4.0,
            storage_gb: 50.0,
            bandwidth_mbps: 100.0,
            allocation_percent: 20.0,
        };

        Self {
            config,
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            allocation: Arc::new(RwLock::new(allocation)),
            metrics: Arc::new(RwLock::new(StreamrMetrics::default())),
        }
    }

    /// Update metrics with uptime
    async fn update_uptime(&self) {
        let mut metrics = self.metrics.write().await;
        if let Some(connected_at) = metrics.connected_at {
            let duration = Utc::now().signed_duration_since(connected_at);
            metrics.connection_uptime_seconds = duration.num_seconds() as u64;
        }
    }

    /// Simulate earning for demonstration
    async fn calculate_current_earnings(&self) -> f64 {
        let metrics = self.metrics.read().await;
        let allocation = self.allocation.read().await;

        // Simulate earnings: base rate * allocation percent * uptime hours
        let uptime_hours = metrics.connection_uptime_seconds as f64 / 3600.0;
        let base_rate = 0.50; // $0.50 per hour at 100% allocation

        (base_rate * allocation.allocation_percent / 100.0) * uptime_hours
    }
}

#[async_trait]
impl ProtocolAdapter for StreamrAdapter {
    fn protocol_name(&self) -> &str {
        "Streamr Network"
    }

    async fn connect(&mut self) -> ProtocolResult<()> {
        if self.config.private_key.is_empty() {
            return Err(ProtocolError::AuthenticationError(
                "Private key not configured".to_string(),
            ));
        }

        // Simulate connection
        *self.status.write().await = ConnectionStatus::Connecting;

        // Simulate connection delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        *self.status.write().await = ConnectionStatus::Connected;

        let mut metrics = self.metrics.write().await;
        metrics.connected_at = Some(Utc::now());
        metrics.connection_uptime_seconds = 0;

        tracing::info!("Connected to Streamr Network");
        Ok(())
    }

    async fn disconnect(&mut self) -> ProtocolResult<()> {
        if *self.status.read().await == ConnectionStatus::Disconnected {
            return Ok(());
        }

        *self.status.write().await = ConnectionStatus::Disconnected;

        let mut metrics = self.metrics.write().await;
        metrics.connected_at = None;

        tracing::info!("Disconnected from Streamr Network");
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        // This would be async, but we need to return synchronously
        // In a real implementation, this might use a cached value
        ConnectionStatus::Connected // Placeholder
    }

    async fn get_current_earnings(&self) -> ProtocolResult<EarningsData> {
        let earnings_usd = self.calculate_current_earnings().await;
        let allocation = self.allocation.read().await;

        let mut metrics = HashMap::new();
        metrics.insert("allocation_percent".to_string(), allocation.allocation_percent);
        metrics.insert("cpu_cores".to_string(), allocation.cpu_cores as f64);

        Ok(EarningsData {
            timestamp: Utc::now(),
            amount_usd: earnings_usd,
            protocol_id: "streamr".to_string(),
            metrics,
        })
    }

    async fn get_historical_earnings(&self, hours: u32) -> ProtocolResult<Vec<EarningsData>> {
        let mut earnings = Vec::new();
        let current_earnings = self.calculate_current_earnings().await;

        // Simulate historical data with decreasing earnings
        for i in 0..hours {
            let hours_ago = Duration::hours(i as i64);
            let timestamp = Utc::now() - hours_ago;

            // Simulate varying earnings
            let amount = current_earnings * (1.0 - (i as f64 / hours as f64) * 0.3);

            earnings.push(EarningsData {
                timestamp,
                amount_usd: amount,
                protocol_id: "streamr".to_string(),
                metrics: HashMap::new(),
            });
        }

        Ok(earnings)
    }

    async fn get_resource_usage(&self) -> ProtocolResult<ResourceMetrics> {
        self.update_uptime().await;
        let metrics = self.metrics.read().await;

        Ok(ResourceMetrics {
            cpu_percent: 25.0,
            memory_mb: 512.0,
            bandwidth_mbps: 45.0,
            storage_gb: 2.5,
            uptime_seconds: metrics.connection_uptime_seconds,
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

        *self.allocation.write().await = strategy;
        tracing::info!("Applied allocation strategy to Streamr");
        Ok(())
    }

    async fn get_current_allocation(&self) -> ProtocolResult<AllocationStrategy> {
        Ok(self.allocation.read().await.clone())
    }

    async fn health_check(&self) -> ProtocolResult<HealthStatus> {
        let status = *self.status.read().await;
        let is_healthy = status == ConnectionStatus::Connected;

        Ok(HealthStatus {
            is_healthy,
            connection_status: status,
            last_operation: Some(Utc::now()),
            error_message: if is_healthy {
                None
            } else {
                Some("Not connected to Streamr".to_string())
            },
            metrics: HashMap::new(),
        })
    }

    fn get_config(&self) -> serde_json::Value {
        serde_json::json!({
            "protocol": "streamr",
            "api_endpoint": self.config.api_endpoint,
            "streams_subscribed": self.config.streams.len(),
            "publish_interval_seconds": self.config.publish_interval_seconds,
            "min_allocation_percent": self.config.min_allocation_percent,
            "max_allocation_percent": self.config.max_allocation_percent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_streamr_creation() {
        let config = StreamrConfig {
            private_key: "test_key".to_string(),
            ..Default::default()
        };
        let adapter = StreamrAdapter::new(config);
        assert_eq!(adapter.protocol_name(), "Streamr Network");
    }

    #[tokio::test]
    async fn test_streamr_connect_disconnect() {
        let config = StreamrConfig {
            private_key: "test_key".to_string(),
            ..Default::default()
        };
        let mut adapter = StreamrAdapter::new(config);

        assert!(adapter.connect().await.is_ok());
        assert!(adapter.disconnect().await.is_ok());
    }

    #[tokio::test]
    async fn test_streamr_earnings() {
        let config = StreamrConfig {
            private_key: "test_key".to_string(),
            ..Default::default()
        };
        let mut adapter = StreamrAdapter::new(config);
        adapter.connect().await.unwrap();

        let earnings = adapter.get_current_earnings().await.unwrap();
        assert_eq!(earnings.protocol_id, "streamr");
        assert!(earnings.amount_usd >= 0.0);
    }

    #[tokio::test]
    async fn test_streamr_allocation() {
        let config = StreamrConfig {
            private_key: "test_key".to_string(),
            ..Default::default()
        };
        let mut adapter = StreamrAdapter::new(config);

        let strategy = AllocationStrategy {
            cpu_cores: 4,
            memory_gb: 8.0,
            storage_gb: 100.0,
            bandwidth_mbps: 200.0,
            allocation_percent: 25.0,
        };

        assert!(adapter.apply_allocation(strategy.clone()).await.is_ok());

        let current = adapter.get_current_allocation().await.unwrap();
        assert_eq!(current.allocation_percent, 25.0);
    }

    #[tokio::test]
    async fn test_streamr_health_check() {
        let config = StreamrConfig {
            private_key: "test_key".to_string(),
            ..Default::default()
        };
        let mut adapter = StreamrAdapter::new(config);
        adapter.connect().await.unwrap();

        let health = adapter.health_check().await.unwrap();
        assert!(health.is_healthy);
    }
}
