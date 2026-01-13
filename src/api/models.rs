use chrono::{DateTime, Utc};
/// API Request/Response Models
///
/// Data structures for HTTP requests and responses.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// METRICS ENDPOINTS
// ============================================================================

/// Get current metrics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub timestamp: DateTime<Utc>,
    pub total_earnings_per_hour: f64,
    pub earnings_by_protocol: HashMap<String, f64>,
    pub allocation_by_protocol: HashMap<String, f64>,
    pub connection_status: HashMap<String, bool>,
    pub resource_utilization: ResourceUtilizationDto,
}

/// Resource utilization DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilizationDto {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub bandwidth_percent: f64,
    pub storage_percent: f64,
}

// ============================================================================
// OPTIMIZATION ENDPOINTS
// ============================================================================

/// Get opportunities request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunitiesRequest {
    #[serde(default)]
    pub limit: Option<usize>,
}

/// Get opportunities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunitiesResponse {
    pub opportunities: Vec<OpportunityDto>,
    pub best_improvement: Option<f64>,
    pub confidence: f64,
}

/// Optimization opportunity DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityDto {
    pub from_protocol: String,
    pub to_protocol: String,
    pub current_rate: f64,
    pub projected_rate: f64,
    pub earnings_improvement: f64,
    pub confidence: f64,
}

// ============================================================================
// ALLOCATION ENDPOINTS
// ============================================================================

/// Get optimal allocation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationResponse {
    pub current_allocation: HashMap<String, f64>,
    pub optimal_allocation: HashMap<String, f64>,
    pub estimated_improvement: f64,
    pub net_benefit: f64,
    pub roi_percent: f64,
}

/// Execute reallocation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReallocateRequest {
    pub allocation: HashMap<String, f64>,
    pub reason: String,
}

/// Execute reallocation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReallocateResponse {
    pub success: bool,
    pub message: String,
    pub changes: Vec<AllocationChangeDto>,
}

/// Allocation change DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationChangeDto {
    pub timestamp: DateTime<Utc>,
    pub protocol: String,
    pub old_allocation: f64,
    pub new_allocation: f64,
    pub earnings_impact: f64,
}

// ============================================================================
// DASHBOARD ENDPOINTS
// ============================================================================

/// Dashboard snapshot response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardResponse {
    pub timestamp: DateTime<Utc>,
    pub total_earnings_per_hour: f64,
    pub earnings_by_protocol: HashMap<String, f64>,
    pub current_allocation: HashMap<String, f64>,
    pub optimal_allocation: HashMap<String, f64>,
    pub next_reallocation_in: Option<u64>,
    pub connection_status: HashMap<String, bool>,
    pub alerts_count: u32,
}

// ============================================================================
// ALERT ENDPOINTS
// ============================================================================

/// Get alerts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertsResponse {
    pub alerts: Vec<AlertDto>,
    pub total_count: usize,
    pub critical_count: u32,
}

/// Alert DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertDto {
    pub timestamp: DateTime<Utc>,
    pub alert_type: String,
    pub severity: f64,
    pub message: String,
    pub acknowledged: bool,
}

/// Acknowledge alert request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcknowledgeAlertRequest {
    pub timestamp: DateTime<Utc>,
}

// ============================================================================
// HISTORY & REPORTING
// ============================================================================

/// Get metrics history request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsHistoryRequest {
    pub hours: Option<i64>,
    pub limit: Option<usize>,
}

/// Get metrics history response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsHistoryResponse {
    pub metrics: Vec<MetricsSnapshot>,
    pub total_count: usize,
}

/// Metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub timestamp: DateTime<Utc>,
    pub total_earnings: f64,
    pub earnings_by_protocol: HashMap<String, f64>,
}

/// Get performance report request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
}

/// Performance report response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResponse {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_earnings: f64,
    pub average_hourly_earnings: f64,
    pub earnings_by_protocol: HashMap<String, f64>,
    pub total_improvement: f64,
    pub successful_optimizations: u32,
    pub uptime_percent: f64,
}

// ============================================================================
// CONFIGURATION ENDPOINTS
// ============================================================================

/// Get configuration response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub optimizer_config: OptimizerConfigDto,
    pub reallocation_config: ReallocationConfigDto,
    pub monitor_config: MonitorConfigDto,
}

/// Optimizer configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfigDto {
    pub min_improvement_threshold: f64,
    pub min_improvement_percent: f64,
    pub max_allocation_change: f64,
    pub analysis_window_hours: u32,
}

/// Reallocation configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReallocationConfigDto {
    pub min_hold_duration_seconds: u64,
    pub max_per_hour: u32,
    pub auto_rollback: bool,
}

/// Monitor configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfigDto {
    pub low_earnings_threshold: f64,
    pub optimization_threshold: f64,
    pub connection_timeout_seconds: u64,
    pub max_alerts: usize,
}

// ============================================================================
// ERROR RESPONSES
// ============================================================================

/// Error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl ErrorResponse {
    /// Create new error response
    pub fn new(error: String, message: String) -> Self {
        Self {
            error,
            message,
            timestamp: Utc::now(),
        }
    }
}

// ============================================================================
// SUCCESS RESPONSES
// ============================================================================

/// Success response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub timestamp: DateTime<Utc>,
}

impl<T: Serialize> SuccessResponse<T> {
    /// Create new success response
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
            timestamp: Utc::now(),
        }
    }
}

// ============================================================================
// WEBSOCKET MESSAGES
// ============================================================================

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// Subscribe to metrics updates
    Subscribe {
        protocol: Option<String>,
    },
    /// Unsubscribe from updates
    Unsubscribe {
        protocol: Option<String>,
    },
    /// Metrics update
    MetricsUpdate {
        metrics: MetricsSnapshot,
    },
    /// Alert notification
    AlertNotification {
        alert: AlertDto,
    },
    /// Reallocation notification
    ReallocationNotification {
        changes: Vec<AllocationChangeDto>,
    },
    /// Ping/Pong for keep-alive
    Ping,
    Pong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_response_creation() {
        let response = MetricsResponse {
            timestamp: Utc::now(),
            total_earnings_per_hour: 10.5,
            earnings_by_protocol: HashMap::new(),
            allocation_by_protocol: HashMap::new(),
            connection_status: HashMap::new(),
            resource_utilization: ResourceUtilizationDto {
                cpu_percent: 50.0,
                memory_percent: 60.0,
                bandwidth_percent: 40.0,
                storage_percent: 30.0,
            },
        };
        assert_eq!(response.total_earnings_per_hour, 10.5);
    }

    #[test]
    fn test_error_response_creation() {
        let error = ErrorResponse::new("NOT_FOUND".to_string(), "Protocol not found".to_string());
        assert_eq!(error.error, "NOT_FOUND");
    }

    #[test]
    fn test_success_response_serialization() {
        let data = vec!["a", "b", "c"];
        let response = SuccessResponse::new(data);
        assert!(response.success);
    }
}
