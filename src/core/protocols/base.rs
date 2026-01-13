// Base protocol trait
// All protocol adapters implement this trait

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Protocol-specific errors
#[derive(thiserror::Error, Debug)]
pub enum ProtocolError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Resource allocation failed: {0}")]
    AllocationError(String),

    #[error("Data parsing error: {0}")]
    ParseError(String),

    #[error("Protocol not supported: {0}")]
    UnsupportedError(String),
}

/// Earnings data from a protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsData {
    pub timestamp: DateTime<Utc>,
    pub earnings_usd: f64,
    pub earnings_native: f64,
    pub native_token_symbol: String,
    pub hourly_rate_usd: f64,
    pub protocol_specific: HashMap<String, serde_json::Value>,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub bandwidth_mbps: f64,
    pub storage_gb: f64,
    pub disk_io_mbps: Option<f64>,
    pub network_latency_ms: Option<f64>,
}

/// Resource allocation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationStrategy {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub bandwidth_percent: f64,
    pub storage_percent: f64,
    pub priority_level: u8,
    pub optimization_params: HashMap<String, f64>,
}

/// Protocol connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error(String),
}

/// Protocol health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub uptime_percent: f64,
    pub last_error: Option<String>,
    pub last_check: DateTime<Utc>,
}

/// Base trait that all protocol adapters must implement
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    /// Protocol identifier
    fn protocol_name(&self) -> &str;

    /// Connect to the protocol network
    async fn connect(&mut self) -> Result<(), ProtocolError>;

    /// Disconnect from the protocol network
    async fn disconnect(&mut self) -> Result<(), ProtocolError>;

    /// Get current connection status
    async fn connection_status(&self) -> ConnectionStatus;

    /// Get current earnings data
    async fn get_current_earnings(&self) -> Result<EarningsData, ProtocolError>;

    /// Get historical earnings
    async fn get_historical_earnings(&self, hours: u32) -> Result<Vec<EarningsData>, ProtocolError>;

    /// Get current resource usage
    async fn get_resource_usage(&self) -> Result<ResourceMetrics, ProtocolError>;

    /// Apply new resource allocation
    async fn apply_allocation(&mut self, allocation: AllocationStrategy) -> Result<(), ProtocolError>;

    /// Get current allocation
    async fn get_current_allocation(&self) -> Result<AllocationStrategy, ProtocolError>;

    /// Perform health check
    async fn health_check(&self) -> Result<HealthStatus, ProtocolError>;

    /// Get protocol-specific configuration
    fn get_config(&self) -> serde_json::Value;
}
