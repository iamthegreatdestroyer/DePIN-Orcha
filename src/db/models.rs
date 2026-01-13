/// Database Models
///
/// SQL models for database operations.

use chrono::{DateTime, Utc};
use sqlx::FromRow;

// ============================================================================
// METRICS MODELS
// ============================================================================

/// Metrics record in database
#[derive(Debug, Clone, FromRow)]
pub struct MetricsRecord {
    pub id: Option<i64>,
    pub timestamp: String,
    pub total_earnings_per_hour: f64,
    pub cpu_percent: Option<f64>,
    pub memory_percent: Option<f64>,
    pub bandwidth_percent: Option<f64>,
    pub storage_percent: Option<f64>,
    #[sqlx(skip)]
    pub created_at: Option<DateTime<Utc>>,
}

impl MetricsRecord {
    /// Create new metrics record
    pub fn new(
        timestamp: DateTime<Utc>,
        total_earnings: f64,
        cpu: f64,
        memory: f64,
        bandwidth: f64,
        storage: f64,
    ) -> Self {
        Self {
            id: None,
            timestamp: timestamp.to_rfc3339(),
            total_earnings_per_hour: total_earnings,
            cpu_percent: Some(cpu),
            memory_percent: Some(memory),
            bandwidth_percent: Some(bandwidth),
            storage_percent: Some(storage),
            created_at: Some(Utc::now()),
        }
    }
}

// ============================================================================
// PROTOCOL METRICS MODELS
// ============================================================================

/// Protocol metrics record
#[derive(Debug, Clone, FromRow)]
pub struct ProtocolMetricsRecord {
    pub id: Option<i64>,
    pub metrics_id: i64,
    pub protocol_name: String,
    pub earnings_per_hour: f64,
    pub allocation_percent: f64,
    pub connected: Option<bool>,
}

impl ProtocolMetricsRecord {
    /// Create new protocol metrics record
    pub fn new(
        metrics_id: i64,
        protocol_name: String,
        earnings: f64,
        allocation: f64,
        connected: bool,
    ) -> Self {
        Self {
            id: None,
            metrics_id,
            protocol_name,
            earnings_per_hour: earnings,
            allocation_percent: allocation,
            connected: Some(connected),
        }
    }
}

// ============================================================================
// REALLOCATION MODELS
// ============================================================================

/// Reallocation record
#[derive(Debug, Clone, FromRow)]
pub struct ReallocationRecord {
    pub id: Option<i64>,
    pub timestamp: String,
    pub protocol_name: String,
    pub old_allocation: f64,
    pub new_allocation: f64,
    pub earnings_impact: Option<f64>,
    pub reason: Option<String>,
}

impl ReallocationRecord {
    /// Create new reallocation record
    pub fn new(
        timestamp: DateTime<Utc>,
        protocol_name: String,
        old_allocation: f64,
        new_allocation: f64,
        earnings_impact: Option<f64>,
        reason: Option<String>,
    ) -> Self {
        Self {
            id: None,
            timestamp: timestamp.to_rfc3339(),
            protocol_name,
            old_allocation,
            new_allocation,
            earnings_impact,
            reason,
        }
    }
}

// ============================================================================
// ALERT MODELS
// ============================================================================

/// Alert record
#[derive(Debug, Clone, FromRow)]
pub struct AlertRecord {
    pub id: Option<i64>,
    pub timestamp: String,
    pub alert_type: String,
    pub severity: f64,
    pub message: String,
    pub acknowledged: Option<bool>,
}

impl AlertRecord {
    /// Create new alert record
    pub fn new(
        timestamp: DateTime<Utc>,
        alert_type: String,
        severity: f64,
        message: String,
    ) -> Self {
        Self {
            id: None,
            timestamp: timestamp.to_rfc3339(),
            alert_type,
            severity,
            message,
            acknowledged: Some(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_record_creation() {
        let now = Utc::now();
        let record = MetricsRecord::new(now, 10.5, 50.0, 60.0, 40.0, 30.0);
        assert_eq!(record.total_earnings_per_hour, 10.5);
        assert_eq!(record.cpu_percent, Some(50.0));
    }

    #[test]
    fn test_reallocation_record_creation() {
        let now = Utc::now();
        let record = ReallocationRecord::new(
            now,
            "streamr".to_string(),
            50.0,
            75.0,
            Some(2.5),
            Some("Optimization".to_string()),
        );
        assert_eq!(record.protocol_name, "streamr");
        assert_eq!(record.new_allocation, 75.0);
    }

    #[test]
    fn test_alert_record_creation() {
        let now = Utc::now();
        let record = AlertRecord::new(
            now,
            "LOW_EARNINGS".to_string(),
            0.8,
            "Earnings below threshold".to_string(),
        );
        assert_eq!(record.alert_type, "LOW_EARNINGS");
        assert_eq!(record.acknowledged, Some(false));
    }
}
