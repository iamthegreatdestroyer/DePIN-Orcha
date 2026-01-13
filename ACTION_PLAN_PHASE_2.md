# ACTION PLAN - PHASE 2: Protocol Adapters Implementation

**Reference:** REF:P2-PROTO  
**Timeline:** 7-10 days  
**Status:** NOT STARTED  
**Dependencies:** Phase 1 must be complete

---

## 📋 PHASE 2 OBJECTIVES

- [ ] Implement base protocol trait and error handling
- [ ] Create Streamr protocol adapter
- [ ] Create Storj protocol adapter
- [ ] Create Golem protocol adapter
- [ ] Create Grass protocol adapter
- [ ] Write comprehensive tests for all adapters
- [ ] Document protocol-specific configurations

---

## ✅ TASK CHECKLIST

### 2.1 BASE PROTOCOL INTERFACE

#### Task 2.1.1: Protocol Trait Definition

**File:** `src/protocols/mod.rs`

- [ ] Define `ProtocolError` enum with all error variants

  - [ ] ConnectionError
  - [ ] AuthenticationError
  - [ ] ApiError
  - [ ] AllocationError
  - [ ] ParseError
  - [ ] UnsupportedError

- [ ] Define data structures

  - [ ] `EarningsData` (earnings tracking)
  - [ ] `ResourceMetrics` (resource usage)
  - [ ] `AllocationStrategy` (resource allocation)
  - [ ] `ConnectionStatus` enum
  - [ ] `HealthStatus` struct

- [ ] Implement `ProtocolAdapter` trait with required methods
  - [ ] `protocol_name() -> &str`
  - [ ] `connect() -> Result`
  - [ ] `disconnect() -> Result`
  - [ ] `connection_status() -> ConnectionStatus`
  - [ ] `get_current_earnings() -> Result<EarningsData>`
  - [ ] `get_historical_earnings(hours: u32) -> Result<Vec<EarningsData>>`
  - [ ] `get_resource_usage() -> Result<ResourceMetrics>`
  - [ ] `apply_allocation(AllocationStrategy) -> Result`
  - [ ] `get_current_allocation() -> Result<AllocationStrategy>`
  - [ ] `health_check() -> Result<HealthStatus>`
  - [ ] `get_config() -> Value`

**Deliverables:**

- Complete trait definition
- All error types defined
- All data structures defined
- Trait is properly documented

**Tests Required:**

- [ ] Unit tests for error handling
- [ ] Tests for data structure serialization/deserialization

---

#### Task 2.1.2: Protocol Module Organization

- [ ] Create protocol submodules

  - [ ] `src/protocols/streamr.rs`
  - [ ] `src/protocols/storj.rs`
  - [ ] `src/protocols/golem.rs`
  - [ ] `src/protocols/grass.rs`

- [ ] Update `src/protocols/mod.rs` to export all adapters
- [ ] Ensure all protocols compile without warnings

**Deliverables:**

- All module files created
- Proper module hierarchy
- Clean imports and exports

---

### 2.2 STREAMR NETWORK ADAPTER

#### Task 2.2.1: Configuration & Initialization

**File:** `src/protocols/streamr.rs`

- [ ] Define `StreamrConfig` struct with:

  - [ ] `api_endpoint: String`
  - [ ] `private_key: String` (from config)
  - [ ] `streams: Vec<String>` (list of stream IDs)
  - [ ] `publish_interval_seconds: u64`
  - [ ] `min_allocation_percent: f64`
  - [ ] `max_allocation_percent: f64`

- [ ] Define internal `StreamrMetrics` struct

  - [ ] `messages_published: u64`
  - [ ] `bytes_published: u64`
  - [ ] `last_publish_time: Option<DateTime<Utc>>`
  - [ ] `connection_uptime: f64`

- [ ] Implement `StreamrAdapter::new(config: StreamrConfig)`
  - [ ] Initialize HTTP client with timeout
  - [ ] Setup internal state (status, metrics, allocation)
  - [ ] Configure default allocation strategy

**Deliverables:**

- StreamrAdapter struct fully initialized
- Configuration validated
- Default state set correctly

---

#### Task 2.2.2: Connection Management

- [ ] Implement `connect()` method

  - [ ] Establish WebSocket connection to Streamr broker
  - [ ] Authenticate with private key
  - [ ] Subscribe to configured streams
  - [ ] Set connection status to Connected
  - [ ] Log connection details

- [ ] Implement `disconnect()` method

  - [ ] Close WebSocket connections gracefully
  - [ ] Save final metrics to database
  - [ ] Flush any pending messages
  - [ ] Set connection status to Disconnected

- [ ] Implement `connection_status()` method
  - [ ] Return current connection state

**Deliverables:**

- Full WebSocket integration
- Graceful connection lifecycle
- Proper error handling

**Tests Required:**

- [ ] Test successful connection
- [ ] Test connection failure scenarios
- [ ] Test disconnect with pending data
- [ ] Test connection status updates

---

#### Task 2.2.3: Earnings & Resource Tracking

- [ ] Implement `get_current_earnings()` method

  - [ ] Query Streamr indexer/API for current earnings
  - [ ] Parse DATA token amounts
  - [ ] Convert to USD using price oracle
  - [ ] Calculate hourly rate
  - [ ] Return EarningsData struct

- [ ] Implement `get_historical_earnings(hours: u32)` method

  - [ ] Query database for historical data
  - [ ] Aggregate earnings by hour
  - [ ] Return vector of EarningsData
  - [ ] Handle missing data gracefully

- [ ] Implement `get_resource_usage()` method
  - [ ] Measure actual CPU usage (publish throughput)
  - [ ] Measure memory consumption
  - [ ] Track bandwidth usage (published bytes)
  - [ ] Estimate network latency
  - [ ] Return ResourceMetrics struct

**Deliverables:**

- Earnings tracking functional
- Historical data aggregation working
- Resource metrics collected

**Tests Required:**

- [ ] Mock earnings API responses
- [ ] Test earnings calculation
- [ ] Test historical aggregation
- [ ] Test resource usage collection

---

#### Task 2.2.4: Resource Allocation

- [ ] Implement `apply_allocation(allocation: AllocationStrategy)` method

  - [ ] Validate allocation constraints
  - [ ] Adjust publish rate based on bandwidth allocation
  - [ ] Update resource throttling
  - [ ] Persist allocation to database
  - [ ] Log allocation change

- [ ] Implement `get_current_allocation()` method
  - [ ] Return current allocation strategy

**Deliverables:**

- Allocation application working
- Constraints enforced
- Changes persisted

**Tests Required:**

- [ ] Test valid allocation
- [ ] Test allocation outside bounds
- [ ] Test allocation persistence

---

#### Task 2.2.5: Health & Monitoring

- [ ] Implement `health_check()` method

  - [ ] Check WebSocket connection health
  - [ ] Verify recent successful publishes
  - [ ] Check broker connectivity
  - [ ] Return HealthStatus with metrics

- [ ] Implement `get_config()` method
  - [ ] Serialize StreamrConfig to JSON (redact sensitive fields)

**Deliverables:**

- Health checks operational
- Config retrieval functional
- All diagnostics available

**Tests Required:**

- [ ] Test health check success
- [ ] Test health check failure scenarios
- [ ] Test config serialization

---

### 2.3 STORJ NETWORK ADAPTER

#### Task 2.3.1: Configuration & Initialization

**File:** `src/protocols/storj.rs`

- [ ] Define `StorjConfig` struct with:

  - [ ] `node_id: String`
  - [ ] `storage_path: String`
  - [ ] `wallet_address: String`
  - [ ] `api_endpoint: String`
  - [ ] `allocated_storage_gb: f64`
  - [ ] `min_allocation_percent: f64`
  - [ ] `max_allocation_percent: f64`

- [ ] Define internal `StorjMetrics` struct

  - [ ] `storage_used_gb: f64`
  - [ ] `ingress_gb: f64`
  - [ ] `egress_gb: f64`
  - [ ] `audit_success_rate: f64`
  - [ ] `uptime_percent: f64`

- [ ] Implement `StorjAdapter::new(config: StorjConfig)`

**Deliverables:**

- StorjAdapter properly initialized
- Configuration validated

---

#### Task 2.3.2: Node Operations

- [ ] Implement `connect()` method

  - [ ] Initialize storage node
  - [ ] Verify wallet address validity
  - [ ] Check storage path accessibility
  - [ ] Register with satellites

- [ ] Implement `disconnect()` method

  - [ ] Graceful node shutdown
  - [ ] Ensure data transfer completion

- [ ] Implement `get_current_earnings()` method

  - [ ] Query node API for earnings
  - [ ] Calculate earnings from storage + bandwidth
  - [ ] Factor in held amounts and payout schedules
  - [ ] Convert to USD

- [ ] Implement `get_resource_usage()` method
  - [ ] Measure disk I/O
  - [ ] Track bandwidth usage (ingress + egress)
  - [ ] Monitor CPU during piece verification

**Deliverables:**

- Node connection lifecycle working
- Earnings calculation implemented
- Resource tracking operational

**Tests Required:**

- [ ] Test node startup/shutdown
- [ ] Test earnings calculation
- [ ] Test resource usage tracking
- [ ] Test error scenarios

---

#### Task 2.3.3: Allocation & Health

- [ ] Implement `apply_allocation()` and `get_current_allocation()`

  - [ ] Validate storage allocation within bounds
  - [ ] Adjust storage allocation
  - [ ] Update bandwidth limits

- [ ] Implement `health_check()` method
  - [ ] Check audit success rate (>95%)
  - [ ] Verify uptime (>99%)
  - [ ] Check satellite connectivity

**Deliverables:**

- Allocation management working
- Health monitoring operational

---

### 2.4 GOLEM NETWORK ADAPTER

#### Task 2.4.1: Configuration & Initialization

**File:** `src/protocols/golem.rs`

- [ ] Define `GolemConfig` struct with:

  - [ ] `provider_node_url: String`
  - [ ] `wallet_address: String`
  - [ ] `cpu_cores: u32`
  - [ ] `memory_gb: f64`
  - [ ] `gpu_enabled: bool`
  - [ ] `pricing_model: PricingModel`
  - [ ] `min_allocation_percent: f64`
  - [ ] `max_allocation_percent: f64`

- [ ] Define `PricingModel` struct

  - [ ] `per_hour_usd: f64`
  - [ ] `per_cpu_hour_usd: f64`
  - [ ] `start_price_usd: f64`

- [ ] Define internal `GolemMetrics` struct

  - [ ] `tasks_completed: u64`
  - [ ] `compute_hours: f64`
  - [ ] `total_earnings: f64`
  - [ ] `active_tasks: u32`

- [ ] Implement `GolemAdapter::new(config: GolemConfig)`

**Deliverables:**

- GolemAdapter properly initialized
- Pricing model configured

---

#### Task 2.4.2: Provider Operations

- [ ] Implement `connect()` method

  - [ ] Start provider daemon
  - [ ] Register with Golem Network
  - [ ] Configure pricing

- [ ] Implement `disconnect()` method

  - [ ] Complete active tasks gracefully
  - [ ] Stop accepting new tasks
  - [ ] Shutdown provider daemon

- [ ] Implement `get_current_earnings()` method

  - [ ] Query provider daemon for earnings
  - [ ] Convert GLM to USD
  - [ ] Calculate hourly rate from recent tasks

- [ ] Implement `get_resource_usage()` method
  - [ ] Measure CPU usage from active tasks
  - [ ] Track memory consumption
  - [ ] Monitor GPU usage if enabled

**Deliverables:**

- Provider lifecycle management
- Task earnings tracking
- Resource monitoring

**Tests Required:**

- [ ] Test provider startup/shutdown
- [ ] Test task acceptance
- [ ] Test earnings calculation
- [ ] Test resource allocation

---

#### Task 2.4.3: Dynamic Pricing & Health

- [ ] Implement task acceptance logic

  - [ ] Accept tasks based on current allocation
  - [ ] Apply dynamic pricing based on demand
  - [ ] Track task performance

- [ ] Implement `health_check()` method
  - [ ] Check provider daemon status
  - [ ] Verify task completion rate
  - [ ] Check network connectivity

**Deliverables:**

- Task management working
- Dynamic pricing applied
- Health monitoring functional

---

### 2.5 GRASS NETWORK ADAPTER

#### Task 2.5.1: Configuration & Initialization

**File:** `src/protocols/grass.rs`

- [ ] Define `GrassConfig` struct with:

  - [ ] `api_key: String`
  - [ ] `api_endpoint: String`
  - [ ] `max_bandwidth_mbps: f64`
  - [ ] `min_allocation_percent: f64`
  - [ ] `max_allocation_percent: f64`

- [ ] Define internal `GrassMetrics` struct

  - [ ] `bandwidth_shared_gb: f64`
  - [ ] `requests_proxied: u64`
  - [ ] `uptime_hours: f64`
  - [ ] `earnings_points: f64`

- [ ] Implement `GrassAdapter::new(config: GrassConfig)`

**Deliverables:**

- GrassAdapter properly initialized
- API authentication configured

---

#### Task 2.5.2: Proxy Operations

- [ ] Implement `connect()` method

  - [ ] Authenticate with API key
  - [ ] Start proxy service
  - [ ] Register node

- [ ] Implement `disconnect()` method

  - [ ] Stop accepting new requests
  - [ ] Complete active proxy sessions
  - [ ] Shutdown proxy service

- [ ] Implement `get_current_earnings()` method

  - [ ] Query Grass API for earnings points
  - [ ] Convert points to USD equivalent
  - [ ] Calculate hourly rate

- [ ] Implement `get_resource_usage()` method
  - [ ] Track bandwidth usage
  - [ ] Measure CPU usage for proxy operations
  - [ ] Monitor memory usage

**Deliverables:**

- Proxy service integration
- Earnings calculation
- Resource tracking

**Tests Required:**

- [ ] Test proxy startup/shutdown
- [ ] Test earnings calculation
- [ ] Test bandwidth tracking

---

#### Task 2.5.3: Traffic & Health

- [ ] Implement request filtering and logging

  - [ ] Log all proxied requests
  - [ ] Filter by geographic location
  - [ ] Track success/failure rates

- [ ] Implement IP reputation management

  - [ ] Track blocked IPs
  - [ ] Maintain whitelist/blacklist

- [ ] Implement `health_check()` method
  - [ ] Check proxy service status
  - [ ] Verify API connectivity
  - [ ] Monitor request success rate

**Deliverables:**

- Request logging operational
- IP reputation tracking
- Health monitoring functional

---

### 2.6 PROTOCOL MANAGEMENT

#### Task 2.6.1: Protocol Registry

**File:** `src/core/protocol_registry.rs` (new)

- [ ] Create `ProtocolRegistry` struct

  - [ ] Registry of all active protocol adapters
  - [ ] Methods to get/register adapters

- [ ] Implement registry methods
  - [ ] `register_adapter(protocol: Box<dyn ProtocolAdapter>)`
  - [ ] `get_adapter(name: &str) -> Option<&dyn ProtocolAdapter>`
  - [ ] `get_all_adapters() -> Vec<&dyn ProtocolAdapter>`
  - [ ] `list_protocols() -> Vec<String>`

**Deliverables:**

- Protocol registry implemented
- Adapter management centralized

**Tests Required:**

- [ ] Test adapter registration
- [ ] Test adapter retrieval
- [ ] Test protocol listing

---

#### Task 2.6.2: Configuration Loading

- [ ] Create protocol configuration loader

  - [ ] Load all protocol configs from TOML
  - [ ] Validate configurations
  - [ ] Initialize all adapters

- [ ] Add to orchestrator initialization
  - [ ] Load protocols on startup
  - [ ] Initialize connections

**Deliverables:**

- Configuration loading functional
- All protocols initialized

**Tests Required:**

- [ ] Test config loading
- [ ] Test invalid config handling
- [ ] Test adapter initialization

---

### 2.7 COMPREHENSIVE TESTING

#### Task 2.7.1: Unit Tests

- [ ] Test all trait implementations

  - [ ] Test each protocol adapter
  - [ ] Mock external API calls
  - [ ] Test error scenarios

- [ ] Achieve 85%+ code coverage
- [ ] Test all error paths

**Test Files:**

- [ ] `tests/unit/protocols/test_streamr.rs`
- [ ] `tests/unit/protocols/test_storj.rs`
- [ ] `tests/unit/protocols/test_golem.rs`
- [ ] `tests/unit/protocols/test_grass.rs`
- [ ] `tests/unit/test_registry.rs`

**Deliverables:**

- All tests passing
- 85%+ coverage
- Error scenarios covered

---

#### Task 2.7.2: Integration Tests

- [ ] Test protocol registry
- [ ] Test configuration loading
- [ ] Test adapter switching
- [ ] Test concurrent operations

**Test Files:**

- [ ] `tests/integration/test_protocols.rs`

**Deliverables:**

- Integration tests passing
- Concurrent operation verified

---

### 2.8 DOCUMENTATION

#### Task 2.8.1: Protocol Documentation

- [ ] Document each protocol adapter
  - [ ] Configuration requirements
  - [ ] API endpoints used
  - [ ] Earnings calculation formulas
  - [ ] Resource constraints
  - [ ] Error handling

**Deliverables:**

- Protocol documentation complete
- Configuration examples provided

#### Task 2.8.2: Code Documentation

- [ ] Add comprehensive docstrings

  - [ ] All public methods documented
  - [ ] Usage examples provided
  - [ ] Error conditions explained

- [ ] Update README with protocol details

**Deliverables:**

- Code fully documented
- README updated

---

## 📊 PHASE 2 SUCCESS CRITERIA

**All of the following must be true:**

- [ ] All 4 protocol adapters implemented
- [ ] Base trait properly designed and documented
- [ ] Each adapter implements all required methods
- [ ] 85%+ code coverage achieved
- [ ] All tests passing
- [ ] No compilation warnings
- [ ] Configuration templates created for each protocol
- [ ] Protocol registry working correctly
- [ ] Error handling comprehensive
- [ ] All code properly documented
- [ ] All changes committed to Git

---

## 🚀 NEXT PHASE

Once Phase 2 is complete and approved:
→ **Phase 3: ML-Driven Optimization Engine** (REF:P3-ML)

---

## 📝 DEVELOPMENT NOTES

- Use tokio for async operations (all protocols async)
- Mock external APIs in tests (use mockito)
- Each protocol operates independently
- Registry pattern allows easy protocol addition
- Configuration-driven (all settings in TOML)
- Comprehensive error handling with custom error types

---

**Last Updated:** January 13, 2026  
**Status:** READY FOR EXECUTION
