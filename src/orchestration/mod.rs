/// Orchestration Engine Module
///
/// Coordinates all protocol adapters and optimizes earnings across networks.
/// Provides multi-protocol monitoring, earnings optimization, and resource reallocation.
pub mod coordinator;
pub mod monitor;
pub mod optimizer;
pub mod reallocation;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Orchestration engine error types
#[derive(Error, Debug)]
pub enum OrchestrationError {
    #[error("Coordination error: {0}")]
    CoordinationError(String),

    #[error("Optimization error: {0}")]
    OptimizationError(String),

    #[error("Reallocation error: {0}")]
    ReallocationError(String),

    #[error("Monitoring error: {0}")]
    MonitoringError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Data error: {0}")]
    DataError(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),
}

pub type OrchestrationResult<T> = Result<T, OrchestrationError>;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Aggregated metrics from all protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    /// Timestamp of metrics
    pub timestamp: DateTime<Utc>,
    /// Total earnings across all protocols (USD/hour)
    pub total_earnings_per_hour: f64,
    /// Earnings by protocol
    pub earnings_by_protocol: HashMap<String, f64>,
    /// Current allocation by protocol (percentage)
    pub allocation_by_protocol: HashMap<String, f64>,
    /// Total resource utilization
    pub resource_utilization: ResourceUtilization,
    /// Connection status by protocol
    pub connection_status: HashMap<String, bool>,
}

/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    /// CPU usage percentage (0-100)
    pub cpu_percent: f64,
    /// Memory usage percentage (0-100)
    pub memory_percent: f64,
    /// Bandwidth usage percentage (0-100)
    pub bandwidth_percent: f64,
    /// Storage usage percentage (0-100)
    pub storage_percent: f64,
}

/// Optimization opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    /// Protocol to reallocate from
    pub from_protocol: String,
    /// Protocol to reallocate to
    pub to_protocol: String,
    /// Current earning rate from protocol
    pub current_rate: f64,
    /// Earning rate after reallocation
    pub projected_rate: f64,
    /// Potential earnings improvement (USD/hour)
    pub earnings_improvement: f64,
    /// Confidence level (0-1)
    pub confidence: f64,
    /// Implementation complexity (0-1, lower is easier)
    pub complexity: f64,
}

/// Allocation plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationPlan {
    /// Proposed allocation by protocol
    pub allocation: HashMap<String, f64>,
    /// Estimated total earnings improvement (USD/hour)
    pub estimated_improvement: f64,
    /// Estimated cost of reallocation
    pub estimated_cost: f64,
    /// Net benefit after cost
    pub net_benefit: f64,
    /// ROI percentage
    pub roi_percent: f64,
    /// Confidence in this plan (0-1)
    pub confidence: f64,
    /// Timestamp when plan was created
    pub created_at: DateTime<Utc>,
}

/// Allocation change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationChange {
    /// Timestamp of change
    pub timestamp: DateTime<Utc>,
    /// Protocol affected
    pub protocol: String,
    /// Old allocation percent
    pub old_allocation: f64,
    /// New allocation percent
    pub new_allocation: f64,
    /// Reason for change
    pub reason: String,
    /// Expected earnings impact
    pub earnings_impact: f64,
}

/// Dashboard snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSnapshot {
    /// Current timestamp
    pub timestamp: DateTime<Utc>,
    /// Total earnings per hour (USD)
    pub total_earnings_per_hour: f64,
    /// Earnings by protocol
    pub earnings_by_protocol: HashMap<String, f64>,
    /// Current allocation
    pub current_allocation: HashMap<String, f64>,
    /// Optimal allocation
    pub optimal_allocation: HashMap<String, f64>,
    /// Optimization opportunity
    pub optimization_opportunity: Option<OptimizationOpportunity>,
    /// Next recommended reallocation time
    pub next_reallocation_in: Option<std::time::Duration>,
    /// Connection status
    pub connection_status: HashMap<String, bool>,
    /// Recent changes
    pub recent_changes: Vec<AllocationChange>,
}

/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    /// Low earnings detected
    LowEarnings { current_rate: f64, threshold: f64 },
    /// Protocol disconnected
    ProtocolDisconnected { protocol: String },
    /// Reallocation opportunity
    ReallocationOpportunity {
        opportunity: OptimizationOpportunity,
    },
    /// Resource contention
    ResourceContention { resource: String },
    /// Optimization potential
    OptimizationPotential { potential_improvement: f64 },
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Timestamp of alert
    pub timestamp: DateTime<Utc>,
    /// Alert type
    pub alert_type: AlertType,
    /// Severity (0-1, higher is more severe)
    pub severity: f64,
    /// Alert message
    pub message: String,
    /// Whether alert has been acknowledged
    pub acknowledged: bool,
}

/// Performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// Period covered
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    /// Total earnings during period
    pub total_earnings: f64,
    /// Average earnings per hour
    pub average_hourly_earnings: f64,
    /// Earnings by protocol
    pub earnings_by_protocol: HashMap<String, f64>,
    /// Allocation changes made
    pub allocation_changes: Vec<AllocationChange>,
    /// Total improvement from reallocations
    pub total_improvement: f64,
    /// Number of successful optimizations
    pub successful_optimizations: u32,
    /// Uptime percentage
    pub uptime_percent: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregated_metrics_creation() {
        let metrics = AggregatedMetrics {
            timestamp: Utc::now(),
            total_earnings_per_hour: 10.50,
            earnings_by_protocol: HashMap::new(),
            allocation_by_protocol: HashMap::new(),
            resource_utilization: ResourceUtilization {
                cpu_percent: 50.0,
                memory_percent: 60.0,
                bandwidth_percent: 40.0,
                storage_percent: 30.0,
            },
            connection_status: HashMap::new(),
        };

        assert_eq!(metrics.total_earnings_per_hour, 10.50);
    }

    #[test]
    fn test_allocation_plan_creation() {
        let plan = AllocationPlan {
            allocation: HashMap::new(),
            estimated_improvement: 1.50,
            estimated_cost: 0.20,
            net_benefit: 1.30,
            roi_percent: 650.0,
            confidence: 0.95,
            created_at: Utc::now(),
        };

        assert_eq!(plan.net_benefit, 1.30);
        assert!(plan.confidence > 0.9);
    }

    #[test]
    fn test_alert_creation() {
        let alert = Alert {
            timestamp: Utc::now(),
            alert_type: AlertType::LowEarnings {
                current_rate: 5.0,
                threshold: 8.0,
            },
            severity: 0.6,
            message: "Earnings below threshold".to_string(),
            acknowledged: false,
        };

        assert!(!alert.acknowledged);
        assert_eq!(alert.severity, 0.6);
    }
}
