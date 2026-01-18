/// Database Module
///
/// Database models, queries, and connection management for metrics persistence.

pub mod models;
pub mod queries;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;
use tracing::info;

// ============================================================================
// DATABASE CONFIGURATION
// ============================================================================

/// Database configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            database_url: "sqlite:depin-orcha.db".to_string(),
            max_connections: 10,
            min_connections: 2,
            connect_timeout: 30,
        }
    }
}

// ============================================================================
// DATABASE INITIALIZATION
// ============================================================================

/// Initialize database connection pool
pub async fn init_pool(config: DbConfig) -> Result<SqlitePool, sqlx::Error> {
    info!("Initializing database: {}", config.database_url);

    // Create database file if it doesn't exist by opening with create flag
    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .connect_with(
            config.database_url.parse::<sqlx::sqlite::SqliteConnectOptions>()?
                .create_if_missing(true)
        )
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    info!("✅ Database initialized successfully");
    Ok(pool)
}

/// Create schema
pub async fn create_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Creating database schema");

    // Metrics table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME NOT NULL,
            total_earnings_per_hour REAL NOT NULL,
            cpu_percent REAL,
            memory_percent REAL,
            bandwidth_percent REAL,
            storage_percent REAL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Protocol metrics table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS protocol_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            metrics_id INTEGER NOT NULL,
            protocol_name TEXT NOT NULL,
            earnings_per_hour REAL NOT NULL,
            allocation_percent REAL NOT NULL,
            connected BOOLEAN DEFAULT true,
            FOREIGN KEY(metrics_id) REFERENCES metrics(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Reallocations table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS reallocations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME NOT NULL,
            protocol_name TEXT NOT NULL,
            old_allocation REAL NOT NULL,
            new_allocation REAL NOT NULL,
            earnings_impact REAL,
            reason TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Alerts table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS alerts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME NOT NULL,
            alert_type TEXT NOT NULL,
            severity REAL NOT NULL,
            message TEXT NOT NULL,
            acknowledged BOOLEAN DEFAULT false,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    info!("✅ Schema created successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_config_default() {
        let config = DbConfig::default();
        assert!(!config.database_url.is_empty());
        assert!(config.max_connections > 0);
    }

    #[test]
    fn test_db_config_custom() {
        let config = DbConfig {
            database_url: "sqlite::memory:".to_string(),
            max_connections: 5,
            min_connections: 1,
            connect_timeout: 60,
        };
        assert_eq!(config.max_connections, 5);
    }
}
