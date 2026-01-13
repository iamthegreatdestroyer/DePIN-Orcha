/// Earnings Optimizer
///
/// Analyzes earnings patterns and identifies optimization opportunities.
/// Calculates optimal resource allocation to maximize total earnings.
use super::{
    AggregatedMetrics, AllocationPlan, OptimizationOpportunity, OrchestrationError,
    OrchestrationResult,
};
use std::collections::HashMap;

// ============================================================================
// OPTIMIZER CONFIGURATION
// ============================================================================

/// Optimizer configuration
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Minimum improvement (USD/hour) to trigger reallocation
    pub min_improvement_threshold: f64,
    /// Minimum improvement percentage
    pub min_improvement_percent: f64,
    /// Maximum allocation change per reallocation
    pub max_allocation_change: f64,
    /// Consider past X hours for analysis
    pub analysis_window_hours: u32,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            min_improvement_threshold: 0.25, // $0.25/hour
            min_improvement_percent: 5.0,    // 5%
            max_allocation_change: 20.0,     // 20% per change
            analysis_window_hours: 24,
        }
    }
}

// ============================================================================
// OPTIMIZER IMPLEMENTATION
// ============================================================================

/// Earnings Optimizer
///
/// Analyzes protocol earnings and identifies optimization opportunities.
pub struct EarningsOptimizer {
    config: OptimizerConfig,
    metrics_history: Vec<AggregatedMetrics>,
}

impl EarningsOptimizer {
    /// Create a new optimizer
    pub fn new(config: OptimizerConfig) -> Self {
        Self {
            config,
            metrics_history: Vec::new(),
        }
    }

    /// Update with new metrics
    pub fn update_metrics(&mut self, metrics: AggregatedMetrics) {
        self.metrics_history.push(metrics);

        // Keep only recent history
        if self.metrics_history.len() > 1000 {
            self.metrics_history.remove(0);
        }
    }

    /// Analyze optimization opportunities
    pub fn analyze_opportunities(
        &self,
        current_metrics: &AggregatedMetrics,
    ) -> OrchestrationResult<Vec<OptimizationOpportunity>> {
        let mut opportunities = Vec::new();

        let earnings = &current_metrics.earnings_by_protocol;
        let allocation = &current_metrics.allocation_by_protocol;

        // Get list of connected protocols
        let connected_protocols: Vec<_> = current_metrics
            .connection_status
            .iter()
            .filter(|(_, &connected)| connected)
            .map(|(name, _)| name.clone())
            .collect();

        if connected_protocols.len() < 2 {
            return Ok(opportunities);
        }

        // Analyze pairwise reallocations
        for i in 0..connected_protocols.len() {
            for j in 0..connected_protocols.len() {
                if i == j {
                    continue;
                }

                let from_protocol = &connected_protocols[i];
                let to_protocol = &connected_protocols[j];

                let from_rate = earnings.get(from_protocol).copied().unwrap_or(0.0);
                let to_rate = earnings.get(to_protocol).copied().unwrap_or(0.0);
                let from_allocation = allocation.get(from_protocol).copied().unwrap_or(0.0);
                let to_allocation = allocation.get(to_protocol).copied().unwrap_or(0.0);

                // Skip if rates are similar or insufficient allocation
                if (to_rate - from_rate).abs() < 0.01 || from_allocation < 1.0 {
                    continue;
                }

                // Calculate potential improvement
                if to_rate > from_rate {
                    let reallocation_amount =
                        (from_allocation * 0.1).min(self.config.max_allocation_change);
                    let rate_difference = to_rate - from_rate;
                    let improvement = rate_difference * (reallocation_amount / 100.0);

                    // Check if improvement meets threshold
                    if improvement > self.config.min_improvement_threshold
                        && (improvement / from_rate * 100.0) > self.config.min_improvement_percent
                    {
                        let confidence =
                            self.calculate_opportunity_confidence(from_protocol, to_protocol);

                        opportunities.push(OptimizationOpportunity {
                            from_protocol: from_protocol.clone(),
                            to_protocol: to_protocol.clone(),
                            current_rate: from_rate,
                            projected_rate: to_rate,
                            earnings_improvement: improvement,
                            confidence,
                            complexity: 0.3, // Simple reallocation
                        });
                    }
                }
            }
        }

        // Sort by earnings improvement
        opportunities.sort_by(|a, b| {
            b.earnings_improvement
                .partial_cmp(&a.earnings_improvement)
                .unwrap()
        });

        Ok(opportunities)
    }

    /// Calculate optimal allocation
    pub fn calculate_optimal_allocation(
        &self,
        current_metrics: &AggregatedMetrics,
    ) -> OrchestrationResult<AllocationPlan> {
        let earnings = &current_metrics.earnings_by_protocol;
        let current_allocation = &current_metrics.allocation_by_protocol;

        // Greedy allocation: allocate more to higher earning protocols
        let mut optimal = current_allocation.clone();
        let mut total_allocation = current_allocation.values().sum::<f64>();

        // Normalize to 100%
        if (total_allocation - 100.0).abs() > 0.1 {
            for value in optimal.values_mut() {
                *value = (*value / total_allocation) * 100.0;
            }
        }

        // Calculate earnings rate per allocated unit for each protocol
        let mut protocol_efficiency: Vec<_> = earnings
            .iter()
            .map(|(name, rate)| {
                let allocation = optimal.get(name).copied().unwrap_or(1.0).max(0.1);
                (name.clone(), rate / allocation)
            })
            .collect();

        // Sort by efficiency
        protocol_efficiency.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Reallocate to most efficient protocols
        let mut new_allocation = current_allocation.clone();

        if protocol_efficiency.len() >= 2 {
            // Move allocation from lowest to highest efficiency
            let bottom_protocol = &protocol_efficiency[protocol_efficiency.len() - 1].0;
            let top_protocol = &protocol_efficiency[0].0;

            let move_amount = (new_allocation.get(bottom_protocol).copied().unwrap_or(0.0) * 0.1)
                .min(self.config.max_allocation_change);

            if move_amount > 0.1 {
                *new_allocation.entry(bottom_protocol.clone()).or_insert(0.0) -= move_amount;
                *new_allocation.entry(top_protocol.clone()).or_insert(0.0) += move_amount;
            }
        }

        // Calculate improvement
        let current_earnings = earnings.values().sum::<f64>();
        let estimated_improvement = self.estimate_earnings_improvement(&new_allocation, earnings);
        let net_benefit = estimated_improvement;
        let cost = estimated_improvement * 0.05; // Assume 5% cost
        let roi_percent = if cost > 0.001 {
            (net_benefit / cost) * 100.0
        } else {
            100.0
        };

        Ok(AllocationPlan {
            allocation: new_allocation,
            estimated_improvement,
            estimated_cost: cost,
            net_benefit: net_benefit - cost,
            roi_percent,
            confidence: 0.85,
            created_at: chrono::Utc::now(),
        })
    }

    /// Estimate earnings improvement for a given allocation
    pub fn estimate_earnings_improvement(
        &self,
        new_allocation: &HashMap<String, f64>,
        earnings_rates: &HashMap<String, f64>,
    ) -> f64 {
        let mut total = 0.0;

        for (protocol, rate) in earnings_rates {
            let allocation = new_allocation.get(protocol).copied().unwrap_or(0.0);
            total += rate * (allocation / 100.0);
        }

        total
    }

    /// Determine if reallocation should be executed
    pub fn should_reallocate(
        &self,
        opportunities: &[OptimizationOpportunity],
        current_plan: Option<&AllocationPlan>,
    ) -> bool {
        if opportunities.is_empty() {
            return false;
        }

        let best_opportunity = &opportunities[0];

        // Check if improvement meets threshold
        if best_opportunity.earnings_improvement < self.config.min_improvement_threshold {
            return false;
        }

        // Check confidence
        if best_opportunity.confidence < 0.7 {
            return false;
        }

        // Check ROI if we have a plan
        if let Some(plan) = current_plan {
            if plan.net_benefit < self.config.min_improvement_threshold {
                return false;
            }
        }

        true
    }

    /// Calculate opportunity confidence
    fn calculate_opportunity_confidence(&self, from_protocol: &str, to_protocol: &str) -> f64 {
        // Base confidence on history consistency
        let from_history: Vec<_> = self
            .metrics_history
            .iter()
            .filter_map(|m| m.earnings_by_protocol.get(from_protocol).copied())
            .collect();

        let to_history: Vec<_> = self
            .metrics_history
            .iter()
            .filter_map(|m| m.earnings_by_protocol.get(to_protocol).copied())
            .collect();

        if from_history.is_empty() || to_history.is_empty() {
            return 0.7; // Default confidence
        }

        // Calculate variance as indicator of stability
        let from_avg = from_history.iter().sum::<f64>() / from_history.len() as f64;
        let to_avg = to_history.iter().sum::<f64>() / to_history.len() as f64;

        let from_variance = from_history
            .iter()
            .map(|x| (x - from_avg).powi(2))
            .sum::<f64>()
            / from_history.len() as f64;

        let to_variance =
            to_history.iter().map(|x| (x - to_avg).powi(2)).sum::<f64>() / to_history.len() as f64;

        // More stable = higher confidence
        let stability_factor = 1.0 / (1.0 + (from_variance + to_variance).sqrt());
        (0.7 * stability_factor + 0.3).min(0.99)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_metrics() -> AggregatedMetrics {
        let mut earnings = HashMap::new();
        earnings.insert("streamr".to_string(), 3.0);
        earnings.insert("storj".to_string(), 4.0);
        earnings.insert("golem".to_string(), 2.5);

        let mut allocation = HashMap::new();
        allocation.insert("streamr".to_string(), 30.0);
        allocation.insert("storj".to_string(), 40.0);
        allocation.insert("golem".to_string(), 30.0);

        let mut status = HashMap::new();
        status.insert("streamr".to_string(), true);
        status.insert("storj".to_string(), true);
        status.insert("golem".to_string(), true);

        AggregatedMetrics {
            timestamp: Utc::now(),
            total_earnings_per_hour: 9.5,
            earnings_by_protocol: earnings,
            allocation_by_protocol: allocation,
            resource_utilization: super::super::ResourceUtilization {
                cpu_percent: 50.0,
                memory_percent: 60.0,
                bandwidth_percent: 40.0,
                storage_percent: 30.0,
            },
            connection_status: status,
        }
    }

    #[test]
    fn test_optimizer_creation() {
        let optimizer = EarningsOptimizer::new(OptimizerConfig::default());
        assert_eq!(optimizer.metrics_history.len(), 0);
    }

    #[test]
    fn test_analyze_opportunities() {
        let optimizer = EarningsOptimizer::new(OptimizerConfig::default());
        let metrics = create_test_metrics();

        let opportunities = optimizer.analyze_opportunities(&metrics).unwrap();
        assert!(!opportunities.is_empty());
    }

    #[test]
    fn test_calculate_optimal_allocation() {
        let optimizer = EarningsOptimizer::new(OptimizerConfig::default());
        let metrics = create_test_metrics();

        let plan = optimizer.calculate_optimal_allocation(&metrics).unwrap();
        assert!(plan.confidence > 0.8);
    }

    #[test]
    fn test_estimate_earnings_improvement() {
        let optimizer = EarningsOptimizer::new(OptimizerConfig::default());
        let mut earnings = HashMap::new();
        earnings.insert("streamr".to_string(), 3.0);
        earnings.insert("storj".to_string(), 4.0);

        let mut allocation = HashMap::new();
        allocation.insert("streamr".to_string(), 50.0);
        allocation.insert("storj".to_string(), 50.0);

        let improvement = optimizer.estimate_earnings_improvement(&allocation, &earnings);
        assert!(improvement > 0.0);
    }
}
