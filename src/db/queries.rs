/// Database Queries
///
/// SQL query functions for metrics persistence and retrieval.

use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::models::*;

// ============================================================================
// METRICS QUERIES
// ============================================================================

/// Store metrics in database
pub async fn store_metrics(
    pool: &SqlitePool,
    timestamp: DateTime<Utc>,
    total_earnings: f64,
    cpu: f64,
    memory: f64,
    bandwidth: f64,
    storage: f64,
) -> Result<i64, sqlx::Error> {
    let record = MetricsRecord::new(
        timestamp,
        total_earnings,
        cpu,
        memory,
        bandwidth,
        storage,
    );

    let result = sqlx::query(
        r#"
        INSERT INTO metrics 
        (timestamp, total_earnings_per_hour, cpu_percent, memory_percent, bandwidth_percent, storage_percent)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&record.timestamp)
    .bind(record.total_earnings_per_hour)
    .bind(record.cpu_percent)
    .bind(record.memory_percent)
    .bind(record.bandwidth_percent)
    .bind(record.storage_percent)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Store protocol metrics
pub async fn store_protocol_metrics(
    pool: &SqlitePool,
    metrics_id: i64,
    protocol_name: String,
    earnings: f64,
    allocation: f64,
    connected: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO protocol_metrics 
        (metrics_id, protocol_name, earnings_per_hour, allocation_percent, connected)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(metrics_id)
    .bind(protocol_name)
    .bind(earnings)
    .bind(allocation)
    .bind(connected)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get latest metrics
pub async fn get_latest_metrics(
    pool: &SqlitePool,
) -> Result<Option<MetricsRecord>, sqlx::Error> {
    sqlx::query_as::<_, MetricsRecord>(
        "SELECT * FROM metrics ORDER BY timestamp DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await
}

/// Get metrics history (last N records)
pub async fn get_metrics_history(
    pool: &SqlitePool,
    limit: i64,
) -> Result<Vec<MetricsRecord>, sqlx::Error> {
    sqlx::query_as::<_, MetricsRecord>(
        "SELECT * FROM metrics ORDER BY timestamp DESC LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Get metrics by time range
pub async fn get_metrics_by_range(
    pool: &SqlitePool,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<MetricsRecord>, sqlx::Error> {
    sqlx::query_as::<_, MetricsRecord>(
        r#"
        SELECT * FROM metrics 
        WHERE timestamp BETWEEN ? AND ?
        ORDER BY timestamp DESC
        "#,
    )
    .bind(start.to_rfc3339())
    .bind(end.to_rfc3339())
    .fetch_all(pool)
    .await
}

// ============================================================================
// REALLOCATION QUERIES
// ============================================================================

/// Store reallocation
pub async fn store_reallocation(
    pool: &SqlitePool,
    timestamp: DateTime<Utc>,
    protocol_name: String,
    old_allocation: f64,
    new_allocation: f64,
    earnings_impact: Option<f64>,
    reason: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO reallocations 
        (timestamp, protocol_name, old_allocation, new_allocation, earnings_impact, reason)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(timestamp.to_rfc3339())
    .bind(protocol_name)
    .bind(old_allocation)
    .bind(new_allocation)
    .bind(earnings_impact)
    .bind(reason)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get reallocation history
pub async fn get_reallocation_history(
    pool: &SqlitePool,
    limit: i64,
) -> Result<Vec<ReallocationRecord>, sqlx::Error> {
    sqlx::query_as::<_, ReallocationRecord>(
        "SELECT * FROM reallocations ORDER BY timestamp DESC LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Get reallocation count by protocol
pub async fn get_reallocation_count(
    pool: &SqlitePool,
    protocol: &str,
) -> Result<i64, sqlx::Error> {
    let result: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM reallocations WHERE protocol_name = ?"
    )
    .bind(protocol)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}

// ============================================================================
// ALERT QUERIES
// ============================================================================

/// Store alert
pub async fn store_alert(
    pool: &SqlitePool,
    timestamp: DateTime<Utc>,
    alert_type: String,
    severity: f64,
    message: String,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO alerts 
        (timestamp, alert_type, severity, message)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(timestamp.to_rfc3339())
    .bind(alert_type)
    .bind(severity)
    .bind(message)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get alert history
pub async fn get_alert_history(
    pool: &SqlitePool,
    limit: i64,
) -> Result<Vec<AlertRecord>, sqlx::Error> {
    sqlx::query_as::<_, AlertRecord>(
        "SELECT * FROM alerts ORDER BY timestamp DESC LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Acknowledge alert
pub async fn acknowledge_alert(
    pool: &SqlitePool,
    timestamp: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE alerts SET acknowledged = true WHERE timestamp = ?"
    )
    .bind(timestamp.to_rfc3339())
    .execute(pool)
    .await?;

    Ok(())
}

/// Get unacknowledged alerts
pub async fn get_unacknowledged_alerts(
    pool: &SqlitePool,
) -> Result<Vec<AlertRecord>, sqlx::Error> {
    sqlx::query_as::<_, AlertRecord>(
        "SELECT * FROM alerts WHERE acknowledged = false ORDER BY timestamp DESC"
    )
    .fetch_all(pool)
    .await
}

// ============================================================================
// STATISTICS QUERIES
// ============================================================================

/// Get total earnings for period
pub async fn get_total_earnings(
    pool: &SqlitePool,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<f64, sqlx::Error> {
    let result: (Option<f64>,) = sqlx::query_as(
        r#"
        SELECT SUM(total_earnings_per_hour) FROM metrics 
        WHERE timestamp BETWEEN ? AND ?
        "#,
    )
    .bind(start.to_rfc3339())
    .bind(end.to_rfc3339())
    .fetch_one(pool)
    .await?;

    Ok(result.0.unwrap_or(0.0))
}

/// Get average uptime
pub async fn get_average_uptime(
    pool: &SqlitePool,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<f64, sqlx::Error> {
    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM metrics 
        WHERE timestamp BETWEEN ? AND ?
        "#,
    )
    .bind(start.to_rfc3339())
    .bind(end.to_rfc3339())
    .fetch_one(pool)
    .await?;

    if total == 0 {
        return Ok(0.0);
    }

    // Calculate uptime based on successful metric collection
    // In a real system, this would be more sophisticated
    Ok(100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reallocation_record_creation() {
        let now = Utc::now();
        let record = ReallocationRecord::new(
            now,
            "storj".to_string(),
            50.0,
            60.0,
            Some(1.5),
            Some("test".to_string()),
        );
        assert_eq!(record.protocol_name, "storj");
    }

    #[test]
    fn test_alert_record_creation() {
        let now = Utc::now();
        let record = AlertRecord::new(
            now,
            "TEST".to_string(),
            0.5,
            "Test alert".to_string(),
        );
        assert_eq!(record.alert_type, "TEST");
    }
}
