/// Grass Network Protocol Adapter
///
/// Grass is a network that monetizes consumer bandwidth and data.
/// Users can earn rewards by sharing their internet connection.
///
/// This adapter manages:
/// - Connection to Grass network
/// - Bandwidth sharing and monitoring
/// - Earnings tracking from data provision
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

/// Grass protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrassConfig {
    /// API endpoint for Grass
    pub api_endpoint: String,
    /// User authentication token
    pub auth_token: String,
    /// Account email
    pub email: String,
    /// Minimum allocation percent
    pub min_allocation_percent: f64,
    /// Maximum allocation percent
    pub max_allocation_percent: f64,
}

impl Default for GrassConfig {
    fn default() -> Self {
        Self {
            api_endpoint: "https://api.grassnet.io".to_string(),
            auth_token: String::new(),
            email: String::new(),
            min_allocation_percent: 20.0,
            max_allocation_percent: 100.0,
        }
    }
}

// ============================================================================
// INTERNAL STATE
// ============================================================================

/// Grass metrics tracked internally
#[derive(Debug, Clone)]
struct GrassMetrics {
    bandwidth_shared_gb: f64,
    data_points_shared: u64,
    connected_at: Option<DateTime<Utc>>,
    connection_uptime_hours: u64,
    user_rank: u32,
}

impl Default for GrassMetrics {
    fn default() -> Self {
        Self {
            bandwidth_shared_gb: 0.0,
            data_points_shared: 0,
            connected_at: None,
            connection_uptime_hours: 0,
            user_rank: 100000,
        }
    }
}

// ============================================================================
// ADAPTER IMPLEMENTATION
// ============================================================================

/// Grass Network Protocol Adapter
pub struct GrassAdapter {
    config: GrassConfig,
    status: Arc<RwLock<ConnectionStatus>>,
    allocation: Arc<RwLock<AllocationStrategy>>,
    metrics: Arc<RwLock<GrassMetrics>>,
}

impl GrassAdapter {
    /// Create a new Grass adapter
    pub fn new(config: GrassConfig) -> Self {
        let allocation = AllocationStrategy {
            cpu_cores: 1,
            memory_gb: 1.0,
            storage_gb: 10.0,
            bandwidth_mbps: 500.0, // Grass primarily uses bandwidth
            allocation_percent: 75.0,
        };

        Self {
            config,
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            allocation: Arc::new(RwLock::new(allocation)),
            metrics: Arc::new(RwLock::new(GrassMetrics::default())),
        }
    }

    /// Update metrics with uptime
    async fn update_uptime(&self) {
        let mut metrics = self.metrics.write().await;
        if let Some(connected_at) = metrics.connected_at {
            let duration = Utc::now().signed_duration_since(connected_at);
            metrics.connection_uptime_hours = duration.num_hours() as u64;

            // Simulate bandwidth sharing: ~1GB per hour
            metrics.bandwidth_shared_gb = metrics.connection_uptime_hours as f64 * 1.5;

            // Simulate data points: ~10 per hour
            metrics.data_points_shared = metrics.connection_uptime_hours * 10;
        }
    }

    /// Simulate earning for demonstration
    async fn calculate_current_earnings(&self) -> f64 {
        let metrics = self.metrics.read().await;
        let allocation = self.allocation.read().await;

        // Simulate earnings: based on bandwidth shared
        let base_rate = 0.02; // $0.02 per GB of bandwidth shared
        let bandwidth_earnings = metrics.bandwidth_shared_gb * base_rate;

        // Rank multiplier: better rank = higher earnings
        let rank_multiplier = (100000.0 / metrics.user_rank.max(100) as f64).min(3.0);

        let allocated_earnings =
            bandwidth_earnings * rank_multiplier * (allocation.allocation_percent / 100.0);

        allocated_earnings
    }
}

#[async_trait]
impl ProtocolAdapter for GrassAdapter {
    fn protocol_name(&self) -> &str {
        "Grass Network"
    }

    async fn connect(&mut self) -> ProtocolResult<()> {
        if self.config.auth_token.is_empty() {
            return Err(ProtocolError::AuthenticationError(
                "Authentication token not configured".to_string(),
            ));
        }

        if self.config.email.is_empty() {
            return Err(ProtocolError::ConfigurationError(
                "Email not configured".to_string(),
            ));
        }

        // Simulate connection
        *self.status.write().await = ConnectionStatus::Connecting;

        // Simulate connection delay
        tokio::time::sleep(tokio::time::Duration::from_millis(120)).await;

        *self.status.write().await = ConnectionStatus::Connected;

        let mut metrics = self.metrics.write().await;
        metrics.connected_at = Some(Utc::now());
        metrics.connection_uptime_hours = 0;
        metrics.user_rank = 50000; // Start with decent rank

        tracing::info!("Connected to Grass Network");
        Ok(())
    }

    async fn disconnect(&mut self) -> ProtocolResult<()> {
        if *self.status.read().await == ConnectionStatus::Disconnected {
            return Ok(());
        }

        *self.status.write().await = ConnectionStatus::Disconnected;

        let mut metrics = self.metrics.write().await;
        metrics.connected_at = None;

        tracing::info!("Disconnected from Grass Network");
        Ok(())
    }

    fn connection_status(&self) -> ConnectionStatus {
        ConnectionStatus::Connected // Placeholder
    }

    async fn get_current_earnings(&self) -> ProtocolResult<EarningsData> {
        self.update_uptime().await;

        let earnings_usd = self.calculate_current_earnings().await;
        let metrics = self.metrics.read().await;
        let allocation = self.allocation.read().await;

        let mut metric_map = HashMap::new();
        metric_map.insert("bandwidth_shared_gb".to_string(), metrics.bandwidth_shared_gb);
        metric_map.insert("data_points_shared".to_string(), metrics.data_points_shared as f64);
        metric_map.insert("user_rank".to_string(), metrics.user_rank as f64);
        metric_map.insert(
            "bandwidth_mbps_allocated".to_string(),
            allocation.bandwidth_mbps,
        );

        Ok(EarningsData {
            timestamp: Utc::now(),
            amount_usd: earnings_usd,
            protocol_id: "grass".to_string(),
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

            // Simulate consistent earnings
            let variance = 0.8 + ((i % 20) as f64 / 20.0) * 0.4;
            let amount = current_earnings * variance;

            earnings.push(EarningsData {
                timestamp,
                amount_usd: amount,
                protocol_id: "grass".to_string(),
                metrics: HashMap::new(),
            });
        }

        Ok(earnings)
    }

    async fn get_resource_usage(&self) -> ProtocolResult<ResourceMetrics> {
        self.update_uptime().await;
        let metrics = self.metrics.read().await;

        Ok(ResourceMetrics {
            cpu_percent: 5.0,
            memory_mb: 128.0,
            bandwidth_mbps: metrics.bandwidth_shared_gb * 1000.0 / 3600.0, // Convert GB/hour to Mbps
            storage_gb: 1.0,
            uptime_seconds: metrics.connection_uptime_hours * 3600,
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

        // Validate bandwidth is reasonable
        if strategy.bandwidth_mbps > 1000.0 {
            return Err(ProtocolError::AllocationError(
                "Bandwidth allocation exceeds maximum".to_string(),
            ));
        }

        *self.allocation.write().await = strategy;
        tracing::info!("Applied allocation strategy to Grass");
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
            "bandwidth_shared_gb".into(),
            serde_json::json!(metrics.bandwidth_shared_gb),
        );
        health_metrics.insert(
            "uptime_hours".into(),
            serde_json::json!(metrics.connection_uptime_hours),
        );
        health_metrics.insert("user_rank".into(), serde_json::json!(metrics.user_rank));

        Ok(HealthStatus {
            is_healthy,
            connection_status: status,
            last_operation: Some(Utc::now()),
            error_message: if is_healthy {
                None
            } else {
                Some("Not connected to Grass".to_string())
            },
            metrics: health_metrics,
        })
    }

    fn get_config(&self) -> serde_json::Value {
        serde_json::json!({
            "protocol": "grass",
            "api_endpoint": self.config.api_endpoint,
            "email": self.config.email,
            "min_allocation_percent": self.config.min_allocation_percent,
            "max_allocation_percent": self.config.max_allocation_percent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grass_creation() {
        let config = GrassConfig {
            auth_token: "test_token".to_string(),
            email: "test@example.com".to_string(),
            ..Default::default()
        };
        let adapter = GrassAdapter::new(config);
        assert_eq!(adapter.protocol_name(), "Grass Network");
    }

    #[tokio::test]
    async fn test_grass_connect_requires_token() {
        let config = GrassConfig::default();
        let mut adapter = GrassAdapter::new(config);
        assert!(adapter.connect().await.is_err());
    }

    #[tokio::test]
    async fn test_grass_connect_disconnect() {
        let config = GrassConfig {
            auth_token: "test_token".to_string(),
            email: "test@example.com".to_string(),
            ..Default::default()
        };
        let mut adapter = GrassAdapter::new(config);

        assert!(adapter.connect().await.is_ok());
        assert!(adapter.disconnect().await.is_ok());
    }

    #[tokio::test]
    async fn test_grass_earnings() {
        let config = GrassConfig {
            auth_token: "test_token".to_string(),
            email: "test@example.com".to_string(),
            ..Default::default()
        };
        let mut adapter = GrassAdapter::new(config);
        adapter.connect().await.unwrap();

        let earnings = adapter.get_current_earnings().await.unwrap();
        assert_eq!(earnings.protocol_id, "grass");
        assert!(earnings.amount_usd >= 0.0);
    }

    #[tokio::test]
    async fn test_grass_allocation_validation() {
        let config = GrassConfig {
            auth_token: "test_token".to_string(),
            email: "test@example.com".to_string(),
            ..Default::default()
        };
        let mut adapter = GrassAdapter::new(config);

        // Valid allocation
        let strategy = AllocationStrategy {
            cpu_cores: 1,
            memory_gb: 1.0,
            storage_gb: 10.0,
            bandwidth_mbps: 500.0,
            allocation_percent: 75.0,
        };
        assert!(adapter.apply_allocation(strategy).await.is_ok());

        // Exceeds min allocation
        let invalid_strategy = AllocationStrategy {
            cpu_cores: 1,
            memory_gb: 1.0,
            storage_gb: 10.0,
            bandwidth_mbps: 500.0,
            allocation_percent: 10.0,
        };
        assert!(adapter.apply_allocation(invalid_strategy).await.is_err());
    }

    #[tokio::test]
    async fn test_grass_historical_earnings() {
        let config = GrassConfig {
            auth_token: "test_token".to_string(),
            email: "test@example.com".to_string(),
            ..Default::default()
        };
        let mut adapter = GrassAdapter::new(config);
        adapter.connect().await.unwrap();

        let history = adapter.get_historical_earnings(24).await.unwrap();
        assert_eq!(history.len(), 24);
        assert!(history.iter().all(|e| e.protocol_id == "grass"));
    }
}
