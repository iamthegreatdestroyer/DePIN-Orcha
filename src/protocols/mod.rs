/// Protocol Adapters Module
///
/// Provides trait definitions and implementations for all supported DePIN protocols.
/// Each protocol adapter manages connection, earnings tracking, and resource allocation
/// for its respective network.

pub mod streamr;
pub mod storj;
pub mod golem;
pub mod grass;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Protocol adapter error types
#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Allocation error: {0}")]
    AllocationError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Resource error: {0}")]
    ResourceError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedError(String),

    #[error("Data error: {0}")]
    DataError(String),
}

pub type ProtocolResult<T> = Result<T, ProtocolError>;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Connection status states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// Disconnected and idle
    Disconnected,
    /// Connecting to network
    Connecting,
    /// Connected and operational
    Connected,
    /// Temporarily disconnected but attempting to reconnect
    Reconnecting,
    /// Connection failed
    Failed,
}

impl std::fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disconnected => write!(f, "Disconnected"),
            Self::Connecting => write!(f, "Connecting"),
            Self::Connected => write!(f, "Connected"),
            Self::Reconnecting => write!(f, "Reconnecting"),
            Self::Failed => write!(f, "Failed"),
        }
    }
}

/// Single earnings record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsData {
    /// Timestamp of earnings
    pub timestamp: DateTime<Utc>,
    /// Amount earned in USD
    pub amount_usd: f64,
    /// Protocol-specific identifier
    pub protocol_id: String,
    /// Optional detailed metrics
    pub metrics: HashMap<String, f64>,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// CPU usage percentage (0-100)
    pub cpu_percent: f64,
    /// Memory usage in MB
    pub memory_mb: f64,
    /// Network bandwidth in Mbps
    pub bandwidth_mbps: f64,
    /// Storage used in GB
    pub storage_gb: f64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

/// Resource allocation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationStrategy {
    /// CPU cores allocated
    pub cpu_cores: u32,
    /// Memory allocated in GB
    pub memory_gb: f64,
    /// Storage allocated in GB
    pub storage_gb: f64,
    /// Bandwidth allocated in Mbps
    pub bandwidth_mbps: f64,
    /// Allocation percentage (0-100)
    pub allocation_percent: f64,
}

/// Health status report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Is healthy
    pub is_healthy: bool,
    /// Connection status
    pub connection_status: ConnectionStatus,
    /// Last successful operation timestamp
    pub last_operation: Option<DateTime<Utc>>,
    /// Error message if unhealthy
    pub error_message: Option<String>,
    /// Protocol-specific health metrics
    pub metrics: HashMap<String, serde_json::Value>,
}

// ============================================================================
// PROTOCOL ADAPTER TRAIT
// ============================================================================

/// Core trait for all protocol adapters
#[async_trait::async_trait]
pub trait ProtocolAdapter: Send + Sync {
    /// Get protocol name
    fn protocol_name(&self) -> &str;

    /// Connect to the protocol network
    async fn connect(&mut self) -> ProtocolResult<()>;

    /// Disconnect from the protocol network
    async fn disconnect(&mut self) -> ProtocolResult<()>;

    /// Get current connection status
    fn connection_status(&self) -> ConnectionStatus;

    /// Get current earnings
    async fn get_current_earnings(&self) -> ProtocolResult<EarningsData>;

    /// Get historical earnings
    async fn get_historical_earnings(&self, hours: u32) -> ProtocolResult<Vec<EarningsData>>;

    /// Get resource usage metrics
    async fn get_resource_usage(&self) -> ProtocolResult<ResourceMetrics>;

    /// Apply allocation strategy
    async fn apply_allocation(&mut self, strategy: AllocationStrategy) -> ProtocolResult<()>;

    /// Get current allocation
    async fn get_current_allocation(&self) -> ProtocolResult<AllocationStrategy>;

    /// Health check
    async fn health_check(&self) -> ProtocolResult<HealthStatus>;

    /// Get configuration as JSON value
    fn get_config(&self) -> serde_json::Value;
}

// ============================================================================
// COMMON UTILITIES
// ============================================================================

/// Helper to create a basic health status
pub fn basic_health_status(
    is_healthy: bool,
    connection_status: ConnectionStatus,
    error: Option<String>,
) -> HealthStatus {
    HealthStatus {
        is_healthy,
        connection_status,
        last_operation: if is_healthy { Some(Utc::now()) } else { None },
        error_message: error,
        metrics: HashMap::new(),
    }
}

/// Helper to parse percentage allocation
pub fn validate_allocation_percent(percent: f64) -> ProtocolResult<()> {
    if !(0.0..=100.0).contains(&percent) {
        return Err(ProtocolError::AllocationError(
            "Allocation percent must be between 0 and 100".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_status_display() {
        assert_eq!(ConnectionStatus::Connected.to_string(), "Connected");
        assert_eq!(ConnectionStatus::Disconnected.to_string(), "Disconnected");
    }

    #[test]
    fn test_validate_allocation_percent() {
        assert!(validate_allocation_percent(50.0).is_ok());
        assert!(validate_allocation_percent(0.0).is_ok());
        assert!(validate_allocation_percent(100.0).is_ok());
        assert!(validate_allocation_percent(-1.0).is_err());
        assert!(validate_allocation_percent(101.0).is_err());
    }

    #[test]
    fn test_basic_health_status() {
        let status = basic_health_status(true, ConnectionStatus::Connected, None);
        assert!(status.is_healthy);
        assert_eq!(status.connection_status, ConnectionStatus::Connected);
        assert!(status.last_operation.is_some());
        assert!(status.error_message.is_none());
    }

    #[test]
    fn test_earnings_data_serialization() {
        let earnings = EarningsData {
            timestamp: Utc::now(),
            amount_usd: 10.50,
            protocol_id: "streamr-1".to_string(),
            metrics: HashMap::new(),
        };

        let json = serde_json::to_string(&earnings).unwrap();
        let deserialized: EarningsData = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.amount_usd, 10.50);
    }
}
