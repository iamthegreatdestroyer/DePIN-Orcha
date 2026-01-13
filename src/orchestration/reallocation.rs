/// Reallocation Engine
///
/// Executes allocation changes across protocols.
/// Manages reallocation history and validates changes.

use super::{AllocationChange, AllocationPlan, OrchestrationError, OrchestrationResult};
use crate::protocols::{AllocationStrategy, ProtocolAdapter};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// REALLOCATION CONFIGURATION
// ============================================================================

/// Reallocation configuration
#[derive(Debug, Clone)]
pub struct ReallocationConfig {
    /// Minimum time between reallocations
    pub min_hold_duration: Duration,
    /// Maximum reallocations per hour
    pub max_per_hour: u32,
    /// Enable automatic rollback on failure
    pub auto_rollback: bool,
    /// Require confirmation before execution
    pub require_confirmation: bool,
}

impl Default for ReallocationConfig {
    fn default() -> Self {
        Self {
            min_hold_duration: Duration::hours(1),
            max_per_hour: 4,
            auto_rollback: true,
            require_confirmation: false,
        }
    }
}

// ============================================================================
// REALLOCATION ENGINE
// ============================================================================

/// Reallocation Engine
///
/// Manages allocation changes across all protocols.
pub struct ReallocationEngine {
    config: ReallocationConfig,
    history: Arc<RwLock<Vec<AllocationChange>>>,
    last_reallocation: Arc<RwLock<Option<DateTime<Utc>>>>,
    previous_allocation: Arc<RwLock<HashMap<String, f64>>>,
}

impl ReallocationEngine {
    /// Create a new reallocation engine
    pub fn new(config: ReallocationConfig) -> Self {
        Self {
            config,
            history: Arc::new(RwLock::new(Vec::new())),
            last_reallocation: Arc::new(RwLock::new(None)),
            previous_allocation: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Execute a reallocation plan
    pub async fn execute_reallocation(
        &self,
        plan: &AllocationPlan,
        adapters: &HashMap<String, Arc<RwLock<Box<dyn ProtocolAdapter>>>>,
    ) -> OrchestrationResult<()> {
        // Validate constraints
        self.can_reallocate().await?;

        // Validate plan
        self.validate_plan(plan, adapters).await?;

        // Store previous allocation for rollback
        let mut previous = self.previous_allocation.write().await;
        previous.clear();

        // Execute reallocation for each protocol
        for (protocol_name, target_allocation) in &plan.allocation {
            let adapter_lock = adapters.get(protocol_name).ok_or_else(|| {
                OrchestrationError::ReallocationError(format!(
                    "Protocol {} not found",
                    protocol_name
                ))
            })?;

            let mut adapter = adapter_lock.write().await;

            // Store previous allocation
            let current = adapter.get_current_allocation().await.ok();
            if let Some(current_alloc) = current {
                previous.insert(protocol_name.clone(), current_alloc.allocation_percent);
            }

            // Create and apply new allocation strategy
            let strategy = AllocationStrategy {
                cpu_cores: 4,
                memory_gb: 8.0,
                storage_gb: 100.0,
                bandwidth_mbps: 200.0,
                allocation_percent: *target_allocation,
            };

            match adapter.apply_allocation(strategy).await {
                Ok(()) => {
                    tracing::info!(
                        "Applied allocation {} to {}",
                        target_allocation,
                        protocol_name
                    );

                    // Record change
                    let change = AllocationChange {
                        timestamp: Utc::now(),
                        protocol: protocol_name.clone(),
                        old_allocation: previous
                            .get(protocol_name)
                            .copied()
                            .unwrap_or(0.0),
                        new_allocation: *target_allocation,
                        reason: "Optimization reallocation".to_string(),
                        earnings_impact: plan.estimated_improvement,
                    };

                    self.history.write().await.push(change);
                }
                Err(e) => {
                    if self.config.auto_rollback {
                        tracing::warn!(
                            "Reallocation failed for {}, rolling back: {}",
                            protocol_name,
                            e
                        );
                        self.rollback_allocation(adapters).await?;
                    }
                    return Err(OrchestrationError::ReallocationError(format!(
                        "Failed to apply allocation to {}: {}",
                        protocol_name, e
                    )));
                }
            }
        }

        *self.last_reallocation.write().await = Some(Utc::now());

        tracing::info!(
            "Reallocation completed with {:.2}/hour improvement",
            plan.estimated_improvement
        );

        Ok(())
    }

    /// Check if reallocation is possible
    pub async fn can_reallocate(&self) -> OrchestrationResult<()> {
        let last = *self.last_reallocation.read().await;

        if let Some(last_time) = last {
            let elapsed = Utc::now() - last_time;

            if elapsed < self.config.min_hold_duration {
                return Err(OrchestrationError::ReallocationError(
                    format!(
                        "Minimum hold duration not met. Wait {:?} more",
                        self.config.min_hold_duration - elapsed
                    ),
                ));
            }

            // Check rate limiting
            let recent_count = self
                .history
                .read()
                .await
                .iter()
                .filter(|c| {
                    let age = Utc::now() - c.timestamp;
                    age < Duration::hours(1)
                })
                .count();

            if recent_count as u32 >= self.config.max_per_hour {
                return Err(OrchestrationError::ReallocationError(
                    "Rate limit exceeded for reallocations".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Rollback to previous allocation
    pub async fn rollback_allocation(
        &self,
        adapters: &HashMap<String, Arc<RwLock<Box<dyn ProtocolAdapter>>>>,
    ) -> OrchestrationResult<()> {
        let previous = self.previous_allocation.read().await.clone();

        for (protocol_name, previous_allocation) in previous {
            let adapter_lock = adapters.get(&protocol_name).ok_or_else(|| {
                OrchestrationError::ReallocationError(format!(
                    "Protocol {} not found",
                    protocol_name
                ))
            })?;

            let mut adapter = adapter_lock.write().await;

            let strategy = AllocationStrategy {
                cpu_cores: 4,
                memory_gb: 8.0,
                storage_gb: 100.0,
                bandwidth_mbps: 200.0,
                allocation_percent: previous_allocation,
            };

            adapter.apply_allocation(strategy).await?;

            tracing::info!(
                "Rolled back {} to {}%",
                protocol_name,
                previous_allocation
            );
        }

        Ok(())
    }

    /// Get reallocation history
    pub async fn get_reallocation_history(&self) -> Vec<AllocationChange> {
        self.history.read().await.clone()
    }

    /// Get recent reallocations
    pub async fn get_recent_reallocations(
        &self,
        hours: i64,
    ) -> Vec<AllocationChange> {
        let cutoff = Utc::now() - Duration::hours(hours);
        self.history
            .read()
            .await
            .iter()
            .filter(|c| c.timestamp > cutoff)
            .cloned()
            .collect()
    }

    /// Estimate reallocation cost
    pub fn estimate_reallocation_cost(
        &self,
        protocol_count: usize,
    ) -> f64 {
        // Estimate based on number of protocols affected
        0.05 * protocol_count as f64 // $0.05 per protocol
    }

    /// Validate a plan before execution
    async fn validate_plan(
        &self,
        plan: &AllocationPlan,
        adapters: &HashMap<String, Arc<RwLock<Box<dyn ProtocolAdapter>>>>,
    ) -> OrchestrationResult<()> {
        // Check all protocols exist
        for protocol_name in plan.allocation.keys() {
            if !adapters.contains_key(protocol_name) {
                return Err(OrchestrationError::ReallocationError(format!(
                    "Protocol {} not registered",
                    protocol_name
                )));
            }
        }

        // Check total allocation
        let total: f64 = plan.allocation.values().sum();
        if (total - 100.0).abs() > 1.0 {
            return Err(OrchestrationError::ReallocationError(
                format!("Total allocation {} != 100%", total),
            ));
        }

        // Check ROI is positive
        if plan.net_benefit < 0.0 {
            return Err(OrchestrationError::ReallocationError(
                "Plan has negative net benefit".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = ReallocationEngine::new(ReallocationConfig::default());
        assert!(engine.history.blocking_read().is_empty());
    }

    #[test]
    fn test_estimate_cost() {
        let engine = ReallocationEngine::new(ReallocationConfig::default());
        let cost = engine.estimate_reallocation_cost(3);
        assert_eq!(cost, 0.15);
    }

    #[tokio::test]
    async fn test_can_reallocate_empty_history() {
        let engine = ReallocationEngine::new(ReallocationConfig::default());
        assert!(engine.can_reallocate().await.is_ok());
    }

    #[test]
    fn test_reallocation_config_defaults() {
        let config = ReallocationConfig::default();
        assert_eq!(config.max_per_hour, 4);
        assert!(config.auto_rollback);
    }
}
