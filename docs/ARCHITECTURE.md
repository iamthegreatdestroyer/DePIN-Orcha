# DePIN Orcha System Architecture

## Overview

DePIN Orcha is a multi-layered system that coordinates resource allocation and earnings optimization across four decentralized infrastructure networks. This document describes the high-level architecture and component interactions.

## System Layers

### 1. Protocol Layer (Rust)

Each protocol has an adapter implementing the `ProtocolAdapter` trait:

- **Connection Management** - Establish and maintain connections
- **Earnings Tracking** - Real-time earnings data collection
- **Resource Monitoring** - Monitor CPU, memory, bandwidth, storage
- **Allocation Control** - Apply resource allocation changes
- **Health Checks** - Periodic health verification

### 2. Orchestrator Layer (Rust)

Central coordination engine:

- **Event Loop** - Main event loop running optimization cycles
- **Configuration Management** - Load and manage system configuration
- **Database Operations** - Persist metrics and decisions
- **ML Integration** - Communicate with ML engine
- **Metrics Collection** - Collect and expose Prometheus metrics

### 3. ML Engine Layer (Python)

Machine learning optimization:

- **Earnings Predictor** - LSTM model for earnings forecasting
- **Resource Allocator** - Convex optimization for allocation decisions
- **Anomaly Detection** - Identify unusual patterns
- **Feature Engineering** - Prepare features for ML models
- **REST API** - Expose predictions via FastAPI

### 4. API & UI Layer

User-facing interfaces:

- **REST API** - Rust or Python APIs for external access
- **Dashboard** - Tauri desktop application with React UI
- **WebSockets** - Real-time updates
- **Configuration UI** - Manage settings and credentials

### 5. Monitoring & Observability

Infrastructure monitoring:

- **Prometheus** - Metrics collection and storage
- **Grafana** - Dashboard visualization
- **Structured Logging** - JSON-formatted logs
- **Health Checks** - Service health monitoring

## Component Interactions

```
┌─────────────────────────────────────────────┐
│          User Dashboard (React/Tauri)        │
└────────────┬────────────────────────────────┘
             │ HTTP/WebSocket
             ▼
┌─────────────────────────────────────────────┐
│           REST API Gateway                   │
│    (Rust Axum or Python FastAPI)            │
└────────────┬────────────────────────────────┘
             │
     ┌───────┴──────────┐
     │                  │
     ▼                  ▼
┌─────────────┐  ┌──────────────────┐
│   Rust Core │  │  Python ML Engine│
│Orchestrator │  │  (FastAPI)       │
└──────┬──────┘  └──────┬───────────┘
       │                │
       ▼                │
  ┌─────────────────┐   │
  │ Protocol Layer  │   │
  │ - Streamr      │   │
  │ - Storj        │◄──┘
  │ - Golem        │
  │ - Grass        │
  └─────────────────┘
       │
       ▼
  ┌─────────────────┐
  │  SQLite Database│
  │  (Metrics, etc) │
  └─────────────────┘

  ┌──────────────────────┐
  │ Monitoring Stack     │
  │ - Prometheus         │
  │ - Grafana            │
  │ - Structured Logs    │
  └──────────────────────┘
```

## Data Flow

### Optimization Cycle (Every 15 minutes)

1. **Collect Data** - Orchestrator gathers metrics from all protocols
2. **Predict** - ML engine predicts earnings for next 24 hours
3. **Optimize** - ML engine solves allocation optimization problem
4. **Evaluate** - Decide if reallocation is beneficial
5. **Apply** - Apply new allocation if improvements expected
6. **Monitor** - Track results and update metrics

### Earnings Tracking

1. **Protocol** collects earnings data
2. **Orchestrator** fetches and normalizes data
3. **Database** stores historical data
4. **ML Engine** uses for predictions
5. **Dashboard** displays to user
6. **Prometheus** collects metrics
7. **Grafana** visualizes trends

## Protocol Adapters

Each protocol implements:

```rust
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    fn protocol_name(&self) -> &str;
    async fn connect(&mut self) -> Result<(), ProtocolError>;
    async fn disconnect(&mut self) -> Result<(), ProtocolError>;
    async fn get_current_earnings(&self) -> Result<EarningsData, ProtocolError>;
    async fn get_resource_usage(&self) -> Result<ResourceMetrics, ProtocolError>;
    async fn apply_allocation(&mut self, allocation: AllocationStrategy) -> Result<(), ProtocolError>;
    async fn health_check(&self) -> Result<HealthStatus, ProtocolError>;
    // ... more methods
}
```

## ML Engine Architecture

### LSTM Earnings Predictor

- **Input:** 24 hours of historical data (hourly)
- **Features:** Earnings, resource usage, time-based features
- **Architecture:** 2-layer LSTM with dropout
- **Output:** 24-hour hourly predictions
- **Accuracy Target:** >85% MAE

### Resource Allocator

- **Algorithm:** Convex optimization (CVXPY + ECOS)
- **Objective:** Maximize total expected earnings
- **Constraints:** CPU, memory, bandwidth, storage limits
- **Per-Protocol Limits:** Min/max allocation percentages
- **Output:** Optimal allocation percentages for each protocol

### Anomaly Detector

- **Algorithm:** Isolation Forest
- **Features:** Earnings deviation, resource spikes, connection status
- **Output:** Anomaly scores and alerts

## Deployment Architecture

### Docker Services

```yaml
services:
  orchestrator: # Rust core engine (port 8080)
  ml-engine: # Python ML service (port 8001)
  prometheus: # Metrics collection (port 9090)
  grafana: # Dashboards (port 3000)
```

### Data Persistence

- **SQLite Database** - Local metrics and configuration
- **Model Files** - Saved ML models (TensorFlow/Keras)
- **Logs** - Structured JSON logs for analysis

## Security Considerations

### Credentials Management

- Protocol credentials stored in encrypted config
- Environment variables override config file
- No credentials in logs or metrics
- Secure API key validation

### Network Security

- HTTPS for external APIs
- mTLS for inter-service communication (optional)
- Firewall rules for port access
- Rate limiting on API endpoints

### Data Protection

- Database encryption at rest (optional)
- Encrypted backups
- Access control to sensitive data
- Audit logging of configuration changes

## Scalability

### Horizontal Scaling

- **Multiple Orchestrators:** Load balanced
- **ML Services:** Scaled independently
- **Database:** Connection pooling

### Vertical Scaling

- **Resource Limits:** Configurable per component
- **Optimization Interval:** Adjustable frequency
- **Batch Sizes:** Configurable for ML operations

## Monitoring & Observability

### Key Metrics

- Earnings per protocol (hourly, daily, monthly)
- Resource utilization (CPU, memory, bandwidth, storage)
- Allocation percentages
- Model prediction accuracy
- System uptime and availability
- API response times
- Optimization cycle times

### Health Checks

- Protocol connectivity
- ML service availability
- Database connectivity
- API responsiveness
- Disk space usage

### Alerting

- Earnings drop >30%
- Service unavailability
- High resource usage
- Prediction errors
- Allocation failures

## Configuration Management

### Environment Variables

```bash
RUST_LOG=info,depin_orcha=debug
DATABASE_URL=sqlite:///data/depin-orcha.db
ML_API_URL=http://localhost:6702
CONFIG_PATH=./config/default.toml
```

### TOML Configuration

```toml
[orchestrator]
api_port = 8080
optimization_interval = 900

[protocols.streamr]
enabled = true
api_endpoint = "..."
```

## Future Enhancements

- Multi-region deployment
- Load balancing across multiple instances
- Advanced ML models (ensemble, reinforcement learning)
- Real-time dashboard updates via WebSocket
- Mobile application
- Advanced alerting (email, SMS, Discord)
- Custom protocol support
- Automated trading integration

---

**Last Updated:** January 13, 2026
