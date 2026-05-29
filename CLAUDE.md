# DePIN-Orcha — Autonomous Completion Brief

## Project Identity
- **Repo:** `iamthegreatdestroyer/DePIN-Orcha`
- **Local path:** `S:\DePIN-Orcha`
- **Language:** Rust
- **Castle Layer:** Layer 6 — Operational Intelligence (DePIN Orchestration)
- **Current completion:** ~55%
- **Mission:** Decentralized Physical Infrastructure Network orchestrator — manages distributed hardware resources, routing, load balancing, and health monitoring for DePIN node networks

## Sprint Plan

### Sprint 1 — Build & Diagnose (Day 1)
```
@APEX run: cargo build && cargo test
Fix all compilation errors. Run cargo clippy -- -D warnings.
Read src/main.rs or src/lib.rs. Document what's implemented vs stubbed in BUILD_STATUS.md.
```

### Sprint 2 — Node Registration + Discovery (Days 1–2)
```
@APEX implement or complete NodeRegistry in src/:
  - Node { id, address, hardware_specs, capacity, status, last_seen }
  - register(node: Node) -> Result<NodeId>
  - deregister(node_id: NodeId) -> Result<()>
  - discover(requirements: HardwareRequirements) -> Vec<Node>
  - health_check() -> HashMap<NodeId, HealthStatus>

Storage: in-memory HashMap<NodeId, Node> with RwLock.
Tests: test_register_deregister, test_discover_by_requirements, test_health_check.
```

### Sprint 3 — Task Routing + Load Balancing (Day 2–3)
```
@APEX implement TaskRouter in src/:
  - route(task: Task) -> Result<NodeId>  // select best node for task
  - Strategies: CapacityFirst, LowestLatency, RoundRobin
  - Respect node hardware requirements (GPU, RAM, bandwidth)
  - Track active task assignments per node

Wire: after task completes or fails, update node availability.
Tests: test_route_to_best_capacity, test_routing_load_balance.
```

### Sprint 4 — gRPC API + Tag (Day 3)
```
@APEX if proto/ exists: implement gRPC server wiring for NodeRegistry and TaskRouter.
If no proto: add HTTP/JSON API via axum or warp.
Run: cargo test --workspace && cargo build --release
git tag v0.3.0 && git push origin v0.3.0
```

## Done Criteria
- [x] `cargo build --release` succeeds
- [x] `cargo test` passes — zero failures (78 tests)
- [x] NodeRegistry: register/discover/health_check work
- [x] TaskRouter: routes tasks to best node by capacity
- [x] `v0.3.0` tag pushed

## Completion Signal
```bash
git tag v0.3.0 && git push origin v0.3.0
```
