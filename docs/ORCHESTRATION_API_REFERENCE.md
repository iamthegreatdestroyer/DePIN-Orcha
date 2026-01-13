# 📚 ORCHESTRATION ENGINE API REFERENCE

**Phase 3: Orchestration Module Documentation**  
**Status:** Live - Ready for Integration  
**Module:** `src/orchestration/`

---

## 🏗️ MODULE OVERVIEW

```
orchestration/
├── mod.rs              (Core types & errors)
├── coordinator.rs      (Multi-protocol polling)
├── optimizer.rs        (Allocation optimization)
├── reallocation.rs     (Execution engine)
└── monitor.rs          (Real-time monitoring)
```

---

## 📦 CORE DATA TYPES

### AggregatedMetrics

```rust
pub struct AggregatedMetrics {
    pub timestamp: DateTime<Utc>,
    pub total_earnings_per_hour: f64,
    pub earnings_by_protocol: HashMap<String, f64>,
    pub allocation_by_protocol: HashMap<String, f64>,
    pub resource_utilization: ResourceUtilization,
    pub connection_status: HashMap<String, bool>,
}
```

### AllocationPlan

```rust
pub struct AllocationPlan {
    pub allocation: HashMap<String, f64>,
    pub estimated_improvement: f64,
    pub estimated_cost: f64,
    pub net_benefit: f64,
    pub roi_percent: f64,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
}
```

### OptimizationOpportunity

```rust
pub struct OptimizationOpportunity {
    pub from_protocol: String,
    pub to_protocol: String,
    pub current_rate: f64,
    pub projected_rate: f64,
    pub earnings_improvement: f64,
    pub confidence: f64,
    pub complexity: f64,
}
```

### DashboardSnapshot

```rust
pub struct DashboardSnapshot {
    pub timestamp: DateTime<Utc>,
    pub total_earnings_per_hour: f64,
    pub earnings_by_protocol: HashMap<String, f64>,
    pub current_allocation: HashMap<String, f64>,
    pub optimal_allocation: HashMap<String, f64>,
    pub optimization_opportunity: Option<OptimizationOpportunity>,
    pub next_reallocation_in: Option<Duration>,
    pub connection_status: HashMap<String, bool>,
    pub recent_changes: Vec<AllocationChange>,
}
```

---

## 🔧 MODULE 1: MULTI-PROTOCOL COORDINATOR

### ProtocolCoordinator

```rust
pub struct ProtocolCoordinator {
    adapters: HashMap<String, Arc<RwLock<Box<dyn ProtocolAdapter>>>>,
    metrics_history: Arc<RwLock<Vec<AggregatedMetrics>>>,
    last_update: Arc<RwLock<Option<DateTime<Utc>>>>,
    max_history_size: usize,
}
```

### Key Methods

#### new(max_history_size: usize)

Creates a new coordinator with specified history limit.

```rust
let coordinator = ProtocolCoordinator::new(1000);
```

#### register_adapter(protocol_name: String, adapter: Box<dyn ProtocolAdapter>)

Registers a protocol adapter.

```rust
coordinator.register_adapter("streamr".to_string(), streamr_adapter);
```

#### poll_all() → OrchestrationResult<AggregatedMetrics>

Polls all registered protocols and aggregates metrics.

```rust
let metrics = coordinator.poll_all().await?;
println!("Total earnings: ${:.2}/hr", metrics.total_earnings_per_hour);
```

#### get_protocol_status(protocol_name: &str) → OrchestrationResult<ProtocolStatus>

Gets status of a specific protocol.

```rust
let status = coordinator.get_protocol_status("streamr").await?;
println!("Streamr earnings: ${:.2}/hr", status.earnings_per_hour);
```

#### get_metrics_history() → Vec<AggregatedMetrics>

Retrieves all historical metrics.

```rust
let history = coordinator.get_metrics_history().await;
```

#### get_metrics_for_period(start: DateTime<Utc>, end: DateTime<Utc>) → OrchestrationResult<Vec<AggregatedMetrics>>

Gets metrics for a specific time period.

```rust
let start = Utc::now() - Duration::hours(24);
let end = Utc::now();
let period_metrics = coordinator.get_metrics_for_period(start, end).await?;
```

---

## 🎯 MODULE 2: EARNINGS OPTIMIZER

### EarningsOptimizer

```rust
pub struct EarningsOptimizer {
    config: OptimizerConfig,
    metrics_history: Vec<AggregatedMetrics>,
}

pub struct OptimizerConfig {
    pub min_improvement_threshold: f64,      // $0.25/hour default
    pub min_improvement_percent: f64,        // 5% default
    pub max_allocation_change: f64,          // 20% default
    pub analysis_window_hours: u32,          // 24 hours default
}
```

### Key Methods

#### new(config: OptimizerConfig)

Creates optimizer with configuration.

```rust
let optimizer = EarningsOptimizer::new(OptimizerConfig::default());
```

#### analyze_opportunities(&self, current_metrics: &AggregatedMetrics) → OrchestrationResult<Vec<OptimizationOpportunity>>

Finds optimization opportunities.

```rust
let opportunities = optimizer.analyze_opportunities(&metrics)?;
for opp in opportunities {
    println!("{} → {}: +${:.2}/hr", opp.from_protocol, opp.to_protocol, opp.earnings_improvement);
}
```

#### calculate_optimal_allocation(&self, current_metrics: &AggregatedMetrics) → OrchestrationResult<AllocationPlan>

Calculates optimal allocation.

```rust
let plan = optimizer.calculate_optimal_allocation(&metrics)?;
println!("Estimated improvement: ${:.2}/hr", plan.estimated_improvement);
```

#### should_reallocate(&self, opportunities: &[OptimizationOpportunity], current_plan: Option<&AllocationPlan>) → bool

Determines if reallocation should happen.

```rust
if optimizer.should_reallocate(&opportunities, Some(&plan)) {
    // Execute reallocation
}
```

#### estimate_earnings_improvement(&self, new_allocation: &HashMap<String, f64>, earnings_rates: &HashMap<String, f64>) → f64

Projects earnings improvement.

```rust
let improvement = optimizer.estimate_earnings_improvement(&allocation, &earnings);
```

---

## ⚙️ MODULE 3: REALLOCATION ENGINE

### ReallocationEngine

```rust
pub struct ReallocationEngine {
    config: ReallocationConfig,
    history: Arc<RwLock<Vec<AllocationChange>>>,
    last_reallocation: Arc<RwLock<Option<DateTime<Utc>>>>,
    previous_allocation: Arc<RwLock<HashMap<String, f64>>>,
}

pub struct ReallocationConfig {
    pub min_hold_duration: Duration,        // 1 hour default
    pub max_per_hour: u32,                  // 4 default
    pub auto_rollback: bool,                // true default
    pub require_confirmation: bool,         // false default
}
```

### Key Methods

#### execute_reallocation(&self, plan: &AllocationPlan, adapters: &HashMap<...>) → OrchestrationResult<()>

Executes allocation plan.

```rust
engine.execute_reallocation(&plan, &adapters).await?;
```

#### can_reallocate(&self) → OrchestrationResult<()>

Checks if reallocation is currently possible.

```rust
if engine.can_reallocate().await.is_ok() {
    // Safe to reallocate
}
```

#### rollback_allocation(&self, adapters: &HashMap<...>) → OrchestrationResult<()>

Reverts to previous allocation.

```rust
engine.rollback_allocation(&adapters).await?;
```

#### get_reallocation_history(&self) → Vec<AllocationChange>

Gets all reallocation changes.

```rust
let history = engine.get_reallocation_history().await;
```

#### get_recent_reallocations(&self, hours: i64) → Vec<AllocationChange>

Gets recent reallocations.

```rust
let recent = engine.get_recent_reallocations(24).await;
```

#### estimate_reallocation_cost(&self, protocol_count: usize) → f64

Estimates cost of reallocation.

```rust
let cost = engine.estimate_reallocation_cost(4);
```

---

## 📊 MODULE 4: REAL-TIME MONITOR

### RealtimeMonitor

```rust
pub struct RealtimeMonitor {
    config: MonitorConfig,
    alerts: Arc<RwLock<Vec<Alert>>>,
    metrics_snapshots: Arc<RwLock<Vec<AggregatedMetrics>>>,
    last_dashboard_update: Arc<RwLock<Option<DateTime<Utc>>>>,
}

pub struct MonitorConfig {
    pub low_earnings_threshold: f64,        // $5/hour default
    pub optimization_threshold: f64,        // $0.25/hour default
    pub connection_timeout: Duration,       // 5 minutes default
    pub max_alerts: usize,                  // 1000 default
}
```

### Key Methods

#### get_dashboard_metrics(&self, current_metrics: &AggregatedMetrics, optimal_allocation: &HashMap<...>, opportunities: &[...]) → OrchestrationResult<DashboardSnapshot>

Gets complete dashboard snapshot.

```rust
let dashboard = monitor.get_dashboard_metrics(&metrics, &optimal, &opps).await?;
println!("Current: ${:.2}/hr", dashboard.total_earnings_per_hour);
println!("Optimal: ${:.2}/hr", dashboard.optimal_allocation);
```

#### check_alerts(&self, current_metrics: &AggregatedMetrics, opportunities: &[OptimizationOpportunity]) → OrchestrationResult<Vec<Alert>>

Checks for new alerts.

```rust
let alerts = monitor.check_alerts(&metrics, &opportunities).await?;
for alert in alerts {
    println!("{} (severity: {:.1})", alert.message, alert.severity);
}
```

#### generate_report(&self, period_start: DateTime<Utc>, period_end: DateTime<Utc>) → OrchestrationResult<PerformanceReport>

Generates performance report.

```rust
let start = Utc::now() - Duration::days(1);
let end = Utc::now();
let report = monitor.generate_report(start, end).await?;
println!("Total earnings: ${:.2}", report.total_earnings);
```

#### get_earnings_trends(&self, hours: i64) → OrchestrationResult<Vec<(DateTime<Utc>, f64)>>

Gets earnings over time.

```rust
let trends = monitor.get_earnings_trends(24).await?;
```

#### get_alert_history(&self) → Vec<Alert>

Gets all alerts.

```rust
let all_alerts = monitor.get_alert_history().await;
```

#### acknowledge_alert(&self, alert_timestamp: DateTime<Utc>) → OrchestrationResult<()>

Acknowledges an alert.

```rust
monitor.acknowledge_alert(alert.timestamp).await?;
```

#### cleanup_old_data(&self, retention_days: i64)

Removes old data.

```rust
monitor.cleanup_old_data(30).await;  // Keep 30 days
```

---

## 🔄 ORCHESTRATION WORKFLOW

### Complete Optimization Cycle

```rust
// 1. Poll all protocols
let metrics = coordinator.poll_all().await?;

// 2. Analyze opportunities
let opportunities = optimizer.analyze_opportunities(&metrics)?;

// 3. Calculate optimal allocation
let plan = optimizer.calculate_optimal_allocation(&metrics)?;

// 4. Check if should reallocate
if optimizer.should_reallocate(&opportunities, Some(&plan)) {
    // 5. Execute reallocation
    engine.execute_reallocation(&plan, &adapters).await?;
}

// 6. Generate dashboard
let dashboard = monitor.get_dashboard_metrics(&metrics, &plan.allocation, &opportunities).await?;

// 7. Check for alerts
let alerts = monitor.check_alerts(&metrics, &opportunities).await?;

// 8. Update snapshot
monitor.update_snapshot(metrics).await;
```

---

## 🧪 USAGE EXAMPLES

### Example 1: Basic Monitoring

```rust
let coordinator = ProtocolCoordinator::new(1000);

// Register adapters
coordinator.register_adapter("streamr".to_string(), streamr_adapter);
coordinator.register_adapter("storj".to_string(), storj_adapter);

// Poll every 5 seconds
loop {
    match coordinator.poll_all().await {
        Ok(metrics) => {
            println!("Total earnings: ${:.2}/hr", metrics.total_earnings_per_hour);
        }
        Err(e) => eprintln!("Poll error: {}", e),
    }
    tokio::time::sleep(Duration::seconds(5)).await;
}
```

### Example 2: Automated Optimization

```rust
let mut optimizer = EarningsOptimizer::new(OptimizerConfig::default());
let engine = ReallocationEngine::new(ReallocationConfig::default());

loop {
    let metrics = coordinator.poll_all().await?;
    optimizer.update_metrics(metrics.clone());

    let opportunities = optimizer.analyze_opportunities(&metrics)?;
    let plan = optimizer.calculate_optimal_allocation(&metrics)?;

    if optimizer.should_reallocate(&opportunities, Some(&plan)) {
        engine.execute_reallocation(&plan, &adapters).await?;
    }

    tokio::time::sleep(Duration::minutes(1)).await;
}
```

### Example 3: Dashboard Updates

```rust
let monitor = RealtimeMonitor::new(MonitorConfig::default());

loop {
    let metrics = coordinator.poll_all().await?;
    let opportunities = optimizer.analyze_opportunities(&metrics)?;

    let dashboard = monitor.get_dashboard_metrics(
        &metrics,
        &plan.allocation,
        &opportunities
    ).await?;

    let alerts = monitor.check_alerts(&metrics, &opportunities).await?;
    monitor.update_snapshot(metrics).await;

    // Send dashboard to WebSocket clients
    send_to_clients(dashboard).await;

    tokio::time::sleep(Duration::seconds(5)).await;
}
```

---

## 📈 ERROR HANDLING

All operations return `OrchestrationResult<T>` which is `Result<T, OrchestrationError>`:

```rust
pub enum OrchestrationError {
    CoordinationError(String),
    OptimizationError(String),
    ReallocationError(String),
    MonitoringError(String),
    ProtocolError(String),
    ConfigurationError(String),
    DataError(String),
    CalculationError(String),
}
```

### Error Handling Pattern

```rust
match coordinator.poll_all().await {
    Ok(metrics) => {
        // Process metrics
    }
    Err(OrchestrationError::CoordinationError(msg)) => {
        eprintln!("Coordination failed: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

---

## 🎯 CONFIGURATION

### Coordinator Config

- `max_history_size`: Max metrics snapshots (default: 1000)

### Optimizer Config

- `min_improvement_threshold`: Min USD/hour (default: $0.25)
- `min_improvement_percent`: Min % improvement (default: 5%)
- `max_allocation_change`: Max % per change (default: 20%)
- `analysis_window_hours`: History window (default: 24)

### Reallocation Config

- `min_hold_duration`: Min time between reallocations (default: 1 hour)
- `max_per_hour`: Rate limit (default: 4/hour)
- `auto_rollback`: Revert on failure (default: true)
- `require_confirmation`: Need approval (default: false)

### Monitor Config

- `low_earnings_threshold`: Alert threshold (default: $5/hr)
- `optimization_threshold`: Optimization alert (default: $0.25/hr)
- `connection_timeout`: Disconnect timeout (default: 5 min)
- `max_alerts`: Alert history size (default: 1000)

---

## 📞 NEXT STEPS

1. **HTTP API Integration** - RESTful endpoints for orchestration
2. **WebSocket Server** - Real-time dashboard updates
3. **Database Persistence** - Store metrics and history
4. **Advanced Algorithms** - Predictive optimization
5. **ML Integration** - Earnings forecasting

---

_API Reference: January 13, 2026_  
_Status: COMPLETE & TESTED_
