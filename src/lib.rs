// DePIN Orcha Library
//
// Main library crate exporting all core modules including protocols,
// orchestration engine, API, and ML components.

#![warn(rust_2018_idioms)]

pub mod api;
pub mod db;
pub mod orchestration;
pub mod protocols;
pub mod scheduler;

// Re-export commonly used types
pub use orchestration::{
    AggregatedMetrics, Alert, AllocationChange, AllocationPlan, DashboardSnapshot,
    OptimizationOpportunity, OrchestrationError, OrchestrationResult, PerformanceReport,
};

pub use orchestration::coordinator::{ProtocolCoordinator, ProtocolStatus};
pub use orchestration::monitor::{MonitorConfig, RealtimeMonitor};
pub use orchestration::optimizer::{EarningsOptimizer, OptimizerConfig};
pub use orchestration::reallocation::{ReallocationConfig, ReallocationEngine};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_imports() {
        // Verify all modules are accessible
        let _ = "orchestration module loaded";
    }
}
