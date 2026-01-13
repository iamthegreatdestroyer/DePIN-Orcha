DePIN Orcha - Master Action Plan
Intelligent Multi-Protocol DePIN Orchestrator
Repository: https://github.com/iamthegreatdestroyer/DePIN-Orcha.git
Project Name: DePIN Orcha

🎯 Executive Summary
This master action plan provides comprehensive instructions for implementing DePIN Orcha, an autonomous system that optimizes earnings across multiple decentralized infrastructure networks (Streamr, Storj, Golem, Grass) using ML-driven resource allocation and predictive analytics.
Implementation Approach: Sequential, test-driven development with autonomous decision-making capabilities within defined architectural boundaries.
Estimated Timeline: 100-150 hours of active development
Target Revenue: $500-2000/month once fully optimized

Phase 1: Foundation & Core Infrastructure [REF:P1-FOUND]
1.1 Project Initialization
First Steps:
bash# Clone the repository
git clone https://github.com/iamthegreatdestroyer/DePIN-Orcha.git
cd DePIN-Orcha
```

**Create the following directory structure:**
```
DePIN-Orcha/
├── src/
│   ├── core/                 # Rust core orchestration engine
│   ├── ml/                   # Python ML models & predictive analytics
│   ├── protocols/            # Protocol-specific adapters
│   ├── api/                  # FastAPI web service
│   └── ui/                   # React dashboard (Tauri app)
├── tests/
│   ├── unit/
│   ├── integration/
│   └── performance/
├── docker/
│   ├── Dockerfile.orchestrator
│   ├── Dockerfile.ml-engine
│   └── Dockerfile.ui
├── scripts/
│   ├── setup-env.sh
│   ├── deploy.sh
│   ├── health-check.sh
│   └── backup.sh
├── config/
│   ├── default.toml
│   ├── production.toml
│   ├── prometheus.yml
│   └── grafana/
├── docs/
│   ├── ARCHITECTURE.md
│   ├── API.md
│   ├── DEPLOYMENT.md
│   └── USER_GUIDE.md
├── migrations/
├── data/
├── models/
├── .github/
│   └── workflows/
│       └── ci-cd.yml
├── .gitignore
├── docker-compose.yml
├── Cargo.toml
├── requirements.txt
├── package.json
└── README.md
Actions:

Initialize each subdirectory with appropriate language-specific structure
Create comprehensive .gitignore combining Rust, Python, Node, and general patterns
Generate README.md with project overview and setup instructions

1.2 Dependency Management Setup
Rust Dependencies
Create Cargo.toml in project root:
toml[package]
name = "depin-orcha"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "Intelligent Multi-Protocol DePIN Orchestrator"
repository = "https://github.com/iamthegreatdestroyer/DePIN-Orcha"
license = "MIT"

[workspace]
members = ["src/core"]

[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio", "migrate"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
anyhow = "1.0"
thiserror = "1.0"
config = "0.13"
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
prometheus = "0.13"
lazy_static = "1.4"

[dev-dependencies]
mockito = "1.2"
tokio-test = "0.4"
criterion = "0.5"

[[bin]]
name = "depin-orcha"
path = "src/core/main.rs"
Python Dependencies
Create requirements.txt in project root:
txt# Core ML/AI Libraries
numpy>=1.24.0,<2.0.0
pandas>=2.0.0,<3.0.0
scikit-learn>=1.3.0,<2.0.0
tensorflow>=2.13.0,<3.0.0
keras>=2.13.0

# Optimization
cvxpy>=1.4.0
scipy>=1.11.0

# API Framework
fastapi>=0.104.0,<1.0.0
uvicorn[standard]>=0.24.0,<1.0.0
pydantic>=2.5.0,<3.0.0
pydantic-settings>=2.0.0

# Database
sqlalchemy>=2.0.0,<3.0.0
aiosqlite>=0.19.0

# Monitoring & Metrics
prometheus-client>=0.19.0
python-json-logger>=2.0.0

# Utilities
python-dotenv>=1.0.0
httpx>=0.25.0
aiofiles>=23.0.0

# Testing
pytest>=7.4.0
pytest-asyncio>=0.21.0
pytest-cov>=4.1.0
pytest-mock>=3.12.0

# Development
black>=23.0.0
flake8>=6.1.0
mypy>=1.7.0
isort>=5.12.0
Create pyproject.toml in project root:
toml[build-system]
requires = ["setuptools>=68.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "depin-orcha"
version = "0.1.0"
description = "Intelligent Multi-Protocol DePIN Orchestrator - ML Engine"
readme = "README.md"
requires-python = ">=3.11"
license = {text = "MIT"}

[tool.black]
line-length = 100
target-version = ['py311']

[tool.isort]
profile = "black"
line_length = 100

[tool.mypy]
python_version = "3.11"
warn_return_any = true
warn_unused_configs = true
disallow_untyped_defs = true

[tool.pytest.ini_options]
asyncio_mode = "auto"
testpaths = ["tests"]
python_files = ["test_*.py"]
python_classes = ["Test*"]
python_functions = ["test_*"]
Frontend Dependencies
Create package.json in src/ui/:
json{
  "name": "depin-orcha-ui",
  "version": "0.1.0",
  "description": "DePIN Orcha Dashboard UI",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "format": "prettier --write \"src/**/*.{ts,tsx,css}\""
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "recharts": "^2.10.3",
    "axios": "^1.6.2",
    "zustand": "^4.4.7",
    "date-fns": "^3.0.6",
    "lucide-react": "^0.294.0",
    "@tanstack/react-query": "^5.14.2"
  },
  "devDependencies": {
    "@tauri-apps/api": "^1.5.1",
    "@tauri-apps/cli": "^1.5.8",
    "@types/react": "^18.2.43",
    "@types/react-dom": "^18.2.17",
    "@typescript-eslint/eslint-plugin": "^6.14.0",
    "@typescript-eslint/parser": "^6.14.0",
    "@vitejs/plugin-react": "^4.2.1",
    "autoprefixer": "^10.4.16",
    "eslint": "^8.55.0",
    "eslint-plugin-react-hooks": "^4.6.0",
    "eslint-plugin-react-refresh": "^0.4.5",
    "postcss": "^8.4.32",
    "prettier": "^3.1.1",
    "tailwindcss": "^3.3.6",
    "typescript": "^5.3.3",
    "vite": "^5.0.8"
  }
}
1.3 Environment Setup Script
Create scripts/setup-env.sh:
bash#!/bin/bash
set -e

echo "🚀 Setting up DePIN Orcha development environment..."

# Check prerequisites
echo "📋 Checking prerequisites..."
command -v rustc >/dev/null 2>&1 || { echo "❌ Rust not found. Install from https://rustup.rs/"; exit 1; }
command -v python3 >/dev/null 2>&1 || { echo "❌ Python 3.11+ not found."; exit 1; }
command -v node >/dev/null 2>&1 || { echo "❌ Node.js not found."; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "❌ Docker not found."; exit 1; }

echo "✅ Prerequisites check passed"

# Setup Python environment
echo "🐍 Setting up Python environment..."
python3 -m venv venv
source venv/bin/activate
pip install --upgrade pip
pip install -r requirements.txt
echo "✅ Python environment ready"

# Setup Rust dependencies
echo "🦀 Building Rust dependencies..."
cargo build
echo "✅ Rust dependencies built"

# Setup UI dependencies
echo "⚛️  Setting up UI dependencies..."
cd src/ui
npm install
cd ../..
echo "✅ UI dependencies installed"

# Create necessary directories
echo "📁 Creating data directories..."
mkdir -p data models logs config/grafana/dashboards
echo "✅ Directories created"

# Initialize database
echo "💾 Initializing database..."
sqlx database create
sqlx migrate run
echo "✅ Database initialized"

# Create default configuration
echo "⚙️  Creating default configuration..."
cp config/default.toml.example config/default.toml 2>/dev/null || echo "Config template not found, skipping"

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Activate Python environment: source venv/bin/activate"
echo "  2. Configure settings in config/default.toml"
echo "  3. Run tests: cargo test && pytest"
echo "  4. Start development: cargo run"
Make executable:
bashchmod +x scripts/setup-env.sh
1.4 Initial Git Configuration
Create comprehensive .gitignore:
gitignore# Rust
/target/
**/*.rs.bk
Cargo.lock

# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
venv/
env/
ENV/
.venv
pip-log.txt
pip-delete-this-directory.txt
.pytest_cache/
.coverage
htmlcov/
*.egg-info/
dist/
build/

# Node
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*
dist-ui/
*.local

# IDEs
.vscode/*
!.vscode/extensions.json
!.vscode/settings.json
.idea/
*.swp
*.swo
*~
.DS_Store

# Data & Models
/data/*.db
/data/*.sqlite
/data/*.db-journal
/models/*.h5
/models/*.pkl
/models/*.joblib
!/models/.gitkeep

# Logs
/logs/*.log
*.log

# Environment
.env
.env.local
.env.*.local
config/production.toml

# OS
Thumbs.db
.DS_Store

# Docker
.dockerignore

# Monitoring
prometheus-data/
grafana-data/

# Temporary files
*.tmp
*.temp
.cache/
1.5 Initial README
Create README.md:
markdown# DePIN Orcha 🎯

**Intelligent Multi-Protocol DePIN Orchestrator**

[![CI/CD](https://github.com/iamthegreatdestroyer/DePIN-Orcha/workflows/CI-CD/badge.svg)](https://github.com/iamthegreatdestroyer/DePIN-Orcha/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

DePIN Orcha is an autonomous passive income system that optimizes earnings across multiple decentralized physical infrastructure networks (DePIN) using machine learning-driven resource allocation and predictive analytics.

### Supported Protocols

- **Streamr Network** - Decentralized real-time data streaming
- **Storj** - Distributed cloud storage
- **Golem Network** - Decentralized computing power marketplace
- **Grass** - Residential bandwidth sharing

### Key Features

- 🤖 **ML-Driven Optimization** - Predictive earnings models with LSTM neural networks
- 📊 **Convex Resource Allocation** - Mathematically optimal resource distribution
- 🔍 **Anomaly Detection** - Automated monitoring and self-healing
- 📈 **Real-time Dashboard** - Beautiful Tauri-based desktop application
- 🐳 **Docker Deployment** - Production-ready containerized deployment
- 📡 **Prometheus Metrics** - Comprehensive monitoring and alerting

## Quick Start

### Prerequisites

- Rust 1.75+ ([Install](https://rustup.rs/))
- Python 3.11+ ([Install](https://www.python.org/downloads/))
- Node.js 18+ ([Install](https://nodejs.org/))
- Docker & Docker Compose ([Install](https://docs.docker.com/get-docker/))

### Installation
```bash
# Clone repository
git clone https://github.com/iamthegreatdestroyer/DePIN-Orcha.git
cd DePIN-Orcha

# Run setup script
./scripts/setup-env.sh

# Configure settings
cp config/default.toml.example config/default.toml
# Edit config/default.toml with your protocol credentials
```

### Development
```bash
# Activate Python environment
source venv/bin/activate

# Run Rust orchestrator
cargo run

# In separate terminal: Run ML engine
cd src/ml
uvicorn api:app --reload --port 8001

# In separate terminal: Run UI
cd src/ui
npm run tauri:dev
```

### Docker Deployment
```bash
# Build and start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Access dashboard
# Open http://localhost:3000
```

## Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                      DePIN Orcha System                      │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐      ┌────────────────┐                   │
│  │   Tauri UI   │◄────►│  FastAPI REST  │                   │
│  │  Dashboard   │      │      API       │                   │
│  └──────────────┘      └────────┬───────┘                   │
│                                  │                           │
│                         ┌────────▼────────┐                  │
│                         │  Rust Core      │                  │
│                         │  Orchestrator   │                  │
│                         └────────┬────────┘                  │
│                                  │                           │
│         ┌────────────────────────┼────────────────────┐      │
│         │                        │                    │      │
│    ┌────▼─────┐         ┌───────▼──────┐      ┌─────▼────┐ │
│    │  Python  │         │   Protocol   │      │ SQLite   │ │
│    │ML Engine │         │   Adapters   │      │ Database │ │
│    │          │         │              │      │          │ │
│    │ • LSTM   │         │ • Streamr    │      │          │ │
│    │ • CVXPY  │         │ • Storj      │      │          │ │
│    │ • Anomaly│         │ • Golem      │      │          │ │
│    └──────────┘         │ • Grass      │      └──────────┘ │
│                         └──────────────┘                    │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Documentation

- [Architecture Guide](docs/ARCHITECTURE.md)
- [API Reference](docs/API.md)
- [Deployment Guide](docs/DEPLOYMENT.md)
- [User Guide](docs/USER_GUIDE.md)
- [Development Guide](docs/DEVELOPMENT.md)

## Project Status

**Current Phase:** Foundation & Infrastructure Setup

- [x] Repository initialization
- [x] Project structure
- [ ] Core orchestration engine
- [ ] Protocol adapters
- [ ] ML optimization engine
- [ ] API & Dashboard
- [ ] Production deployment

## Target Performance

- **Revenue:** $500-2000/month (protocol-dependent)
- **Uptime:** 99.9% autonomous operation
- **Optimization Cycle:** 15-minute intervals
- **Prediction Accuracy:** >85% for 24-hour forecasts

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Streamr Network for decentralized data streaming
- Storj Labs for distributed storage protocol
- Golem Network for decentralized computing
- Grass Network for bandwidth sharing protocol

---

**Built with ❤️ for passive income automation**

Phase 2: Protocol Adapters Implementation [REF:P2-PROTO]
2.1 Base Protocol Interface
Create src/protocols/mod.rs:
rustuse async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Protocol-specific errors
#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    #[error("API request failed: {0}")]
    ApiError(String),
    
    #[error("Resource allocation failed: {0}")]
    AllocationError(String),
    
    #[error("Data parsing error: {0}")]
    ParseError(String),
    
    #[error("Protocol not supported: {0}")]
    UnsupportedError(String),
}

/// Earnings data from a protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsData {
    pub timestamp: DateTime<Utc>,
    pub earnings_usd: f64,
    pub earnings_native: f64,  // In protocol's native token
    pub native_token_symbol: String,
    pub hourly_rate_usd: f64,
    pub protocol_specific: HashMap<String, serde_json::Value>,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub bandwidth_mbps: f64,
    pub storage_gb: f64,
    pub disk_io_mbps: Option<f64>,
    pub network_latency_ms: Option<f64>,
}

/// Resource allocation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationStrategy {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub bandwidth_percent: f64,
    pub storage_percent: f64,
    pub priority_level: u8,  // 1-10
    pub optimization_params: HashMap<String, f64>,
}

/// Protocol connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error(String),
}

/// Protocol health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub uptime_percent: f64,
    pub last_error: Option<String>,
    pub last_check: DateTime<Utc>,
}

/// Base trait that all protocol adapters must implement
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    /// Protocol identifier (e.g., "streamr", "storj")
    fn protocol_name(&self) -> &str;
    
    /// Connect to the protocol network
    async fn connect(&mut self) -> Result<(), ProtocolError>;
    
    /// Disconnect from the protocol network
    async fn disconnect(&mut self) -> Result<(), ProtocolError>;
    
    /// Get current connection status
    async fn connection_status(&self) -> ConnectionStatus;
    
    /// Get current earnings data
    async fn get_current_earnings(&self) -> Result<EarningsData, ProtocolError>;
    
    /// Get historical earnings for specified duration
    async fn get_historical_earnings(
        &self,
        hours: u32,
    ) -> Result<Vec<EarningsData>, ProtocolError>;
    
    /// Get current resource usage
    async fn get_resource_usage(&self) -> Result<ResourceMetrics, ProtocolError>;
    
    /// Apply new resource allocation
    async fn apply_allocation(
        &mut self,
        allocation: AllocationStrategy,
    ) -> Result<(), ProtocolError>;
    
    /// Get current allocation
    async fn get_current_allocation(&self) -> Result<AllocationStrategy, ProtocolError>;
    
    /// Perform health check
    async fn health_check(&self) -> Result<HealthStatus, ProtocolError>;
    
    /// Get protocol-specific configuration
    fn get_config(&self) -> serde_json::Value;
}

// Re-export protocol implementations
pub mod streamr;
pub mod storj;
pub mod golem;
pub mod grass;

pub use streamr::StreamrAdapter;
pub use storj::StorjAdapter;
pub use golem::GolemAdapter;
pub use grass::GrassAdapter;
2.2 Streamr Network Adapter
Create src/protocols/streamr.rs:
rustuse super::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamrConfig {
    pub api_endpoint: String,
    pub private_key: String,
    pub streams: Vec<String>,
    pub publish_interval_seconds: u64,
    pub min_allocation_percent: f64,
    pub max_allocation_percent: f64,
}

#[derive(Debug)]
struct StreamrMetrics {
    messages_published: u64,
    bytes_published: u64,
    last_publish_time: Option<DateTime<Utc>>,
    connection_uptime: f64,
}

pub struct StreamrAdapter {
    config: StreamrConfig,
    client: Client,
    status: Arc<RwLock<ConnectionStatus>>,
    metrics: Arc<RwLock<StreamrMetrics>>,
    current_allocation: Arc<RwLock<AllocationStrategy>>,
}

impl StreamrAdapter {
    pub fn new(config: StreamrConfig) -> Self {
        Self {
            config,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            metrics: Arc::new(RwLock::new(StreamrMetrics {
                messages_published: 0,
                bytes_published: 0,
                last_publish_time: None,
                connection_uptime: 0.0,
            })),
            current_allocation: Arc::new(RwLock::new(AllocationStrategy {
                cpu_percent: 10.0,
                memory_percent: 10.0,
                bandwidth_percent: 20.0,
                storage_percent: 0.0,
                priority_level: 5,
                optimization_params: HashMap::new(),
            })),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for StreamrAdapter {
    fn protocol_name(&self) -> &str {
        "streamr"
    }

    async fn connect(&mut self) -> Result<(), ProtocolError> {
        info!("Connecting to Streamr Network...");
        
        // TODO: Implement WebSocket connection to Streamr broker
        // TODO: Authenticate with private key
        // TODO: Subscribe to specified streams
        
        *self.status.write().await = ConnectionStatus::Connecting;
        
        // Simulate connection (replace with actual implementation)
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        *self.status.write().await = ConnectionStatus::Connected;
        info!("Successfully connected to Streamr Network");
        
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), ProtocolError> {
        info!("Disconnecting from Streamr Network...");
        
        // TODO: Close WebSocket connections gracefully
        // TODO: Save final metrics
        
        *self.status.write().await = ConnectionStatus::Disconnected;
        info!("Disconnected from Streamr Network");
        
        Ok(())
    }

    async fn connection_status(&self) -> ConnectionStatus {
        self.status.read().await.clone()
    }

    async fn get_current_earnings(&self) -> Result<EarningsData, ProtocolError> {
        debug!("Fetching current Streamr earnings...");
        
        // TODO: Query Streamr indexer for earnings data
        // TODO: Convert DATA tokens to USD using price oracle
        // TODO: Calculate hourly rate based on recent activity
        
        // Placeholder implementation
        Ok(EarningsData {
            timestamp: Utc::now(),
            earnings_usd: 0.0,
            earnings_native: 0.0,
            native_token_symbol: "DATA".to_string(),
            hourly_rate_usd: 0.0,
            protocol_specific: HashMap::new(),
        })
    }

    async fn get_historical_earnings(
        &self,
        hours: u32,
    ) -> Result<Vec<EarningsData>, ProtocolError> {
        debug!("Fetching {} hours of Streamr earnings history...", hours);
        
        // TODO: Query historical earnings from database
        // TODO: Aggregate by hour
        
        Ok(Vec::new())
    }

    async fn get_resource_usage(&self) -> Result<ResourceMetrics, ProtocolError> {
        debug!("Collecting Streamr resource metrics...");
        
        // TODO: Measure actual CPU usage
        // TODO: Measure memory consumption
        // TODO: Track bandwidth usage (published bytes)
        
        Ok(ResourceMetrics {
            timestamp: Utc::now(),
            cpu_percent: 5.0,
            memory_mb: 100.0,
            bandwidth_mbps: 10.0,
            storage_gb: 0.0,
            disk_io_mbps: None,
            network_latency_ms: Some(50.0),
        })
    }

    async fn apply_allocation(
        &mut self,
        allocation: AllocationStrategy,
    ) -> Result<(), ProtocolError> {
        info!("Applying new Streamr allocation: {:?}", allocation);
        
        // Validate allocation constraints
        if allocation.bandwidth_percent < self.config.min_allocation_percent
            || allocation.bandwidth_percent > self.config.max_allocation_percent
        {
            return Err(ProtocolError::AllocationError(format!(
                "Bandwidth allocation {}% outside allowed range {}-{}%",
                allocation.bandwidth_percent,
                self.config.min_allocation_percent,
                self.config.max_allocation_percent
            )));
        }
        
        // TODO: Adjust publish rate based on bandwidth allocation
        // TODO: Update resource throttling
        
        *self.current_allocation.write().await = allocation;
        info!("Streamr allocation applied successfully");
        
        Ok(())
    }

    async fn get_current_allocation(&self) -> Result<AllocationStrategy, ProtocolError> {
        Ok(self.current_allocation.read().await.clone())
    }

    async fn health_check(&self) -> Result<HealthStatus, ProtocolError> {
        debug!("Performing Streamr health check...");
        
        let status = self.status.read().await;
        let is_connected = matches!(*status, ConnectionStatus::Connected);
        
        // TODO: Check WebSocket connection health
        // TODO: Verify recent successful publishes
        // TODO: Check broker connectivity
        
        Ok(HealthStatus {
            is_healthy: is_connected,
            uptime_percent: 99.5,
            last_error: None,
            last_check: Utc::now(),
        })
    }

    fn get_config(&self) -> serde_json::Value {
        serde_json::to_value(&self.config).unwrap_or_default()
    }
}

// TODO: Implement StreamrPublisher for actual message publishing
// TODO: Implement StreamrEarningsTracker for monitoring earnings
// TODO: Add comprehensive unit tests
2.3 Storj Network Adapter
Create src/protocols/storj.rs:
rustuse super::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorjConfig {
    pub node_id: String,
    pub storage_path: String,
    pub wallet_address: String,
    pub api_endpoint: String,
    pub allocated_storage_gb: f64,
    pub min_allocation_percent: f64,
    pub max_allocation_percent: f64,
}

#[derive(Debug)]
struct StorjMetrics {
    storage_used_gb: f64,
    ingress_gb: f64,
    egress_gb: f64,
    audit_success_rate: f64,
    uptime_percent: f64,
}

pub struct StorjAdapter {
    config: StorjConfig,
    client: Client,
    status: Arc<RwLock<ConnectionStatus>>,
    metrics: Arc<RwLock<StorjMetrics>>,
    current_allocation: Arc<RwLock<AllocationStrategy>>,
}

impl StorjAdapter {
    pub fn new(config: StorjConfig) -> Self {
        Self {
            config,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            metrics: Arc::new(RwLock::new(StorjMetrics {
                storage_used_gb: 0.0,
                ingress_gb: 0.0,
                egress_gb: 0.0,
                audit_success_rate: 0.0,
                uptime_percent: 0.0,
            })),
            current_allocation: Arc::new(RwLock::new(AllocationStrategy {
                cpu_percent: 5.0,
                memory_percent: 15.0,
                bandwidth_percent: 25.0,
                storage_percent: 50.0,
                priority_level: 5,
                optimization_params: HashMap::new(),
            })),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for StorjAdapter {
    fn protocol_name(&self) -> &str {
        "storj"
    }

    async fn connect(&mut self) -> Result<(), ProtocolError> {
        info!("Connecting to Storj Network...");
        
        // TODO: Initialize storage node
        // TODO: Verify wallet address
        // TODO: Check storage path accessibility
        // TODO: Register with satellites
        
        *self.status.write().await = ConnectionStatus::Connecting;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        *self.status.write().await = ConnectionStatus::Connected;
        
        info!("Successfully connected to Storj Network");
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), ProtocolError> {
        info!("Disconnecting from Storj Network...");
        
        // TODO: Gracefully shutdown storage node
        // TODO: Ensure all data transfers complete
        
        *self.status.write().await = ConnectionStatus::Disconnected;
        info!("Disconnected from Storj Network");
        
        Ok(())
    }

    async fn connection_status(&self) -> ConnectionStatus {
        self.status.read().await.clone()
    }

    async fn get_current_earnings(&self) -> Result<EarningsData, ProtocolError> {
        debug!("Fetching current Storj earnings...");
        
        // TODO: Query node API for earnings
        // TODO: Calculate earnings from storage and bandwidth
        // TODO: Factor in held amounts and payout schedules
        
        Ok(EarningsData {
            timestamp: Utc::now(),
            earnings_usd: 0.0,
            earnings_native: 0.0,
            native_token_symbol: "STORJ".to_string(),
            hourly_rate_usd: 0.0,
            protocol_specific: HashMap::new(),
        })
    }

    async fn get_historical_earnings(
        &self,
        hours: u32,
    ) -> Result<Vec<EarningsData>, ProtocolError> {
        debug!("Fetching {} hours of Storj earnings history...", hours);
        Ok(Vec::new())
    }

    async fn get_resource_usage(&self) -> Result<ResourceMetrics, ProtocolError> {
        debug!("Collecting Storj resource metrics...");
        
        // TODO: Measure disk I/O
        // TODO: Track bandwidth usage (ingress + egress)
        // TODO: Monitor CPU usage during piece verification
        
        Ok(ResourceMetrics {
            timestamp: Utc::now(),
            cpu_percent: 3.0,
            memory_mb: 200.0,
            bandwidth_mbps: 15.0,
            storage_gb: self.config.allocated_storage_gb,
            disk_io_mbps: Some(50.0),
            network_latency_ms: Some(30.0),
        })
    }

    async fn apply_allocation(
        &mut self,
        allocation: AllocationStrategy,
    ) -> Result<(), ProtocolError> {
        info!("Applying new Storj allocation: {:?}", allocation);
        
        // Validate storage allocation
        if allocation.storage_percent < self.config.min_allocation_percent
            || allocation.storage_percent > self.config.max_allocation_percent
        {
            return Err(ProtocolError::AllocationError(format!(
                "Storage allocation {}% outside allowed range {}-{}%",
                allocation.storage_percent,
                self.config.min_allocation_percent,
                self.config.max_allocation_percent
            )));
        }
        
        // TODO: Adjust storage allocation
        // TODO: Update bandwidth limits
        
        *self.current_allocation.write().await = allocation;
        info!("Storj allocation applied successfully");
        
        Ok(())
    }

    async fn get_current_allocation(&self) -> Result<AllocationStrategy, ProtocolError> {
        Ok(self.current_allocation.read().await.clone())
    }

    async fn health_check(&self) -> Result<HealthStatus, ProtocolError> {
        debug!("Performing Storj health check...");
        
        let status = self.status.read().await;
        let is_connected = matches!(*status, ConnectionStatus::Connected);
        
        // TODO: Check audit success rate (should be >95%)
        // TODO: Verify uptime (should be >99%)
        // TODO: Check satellite connectivity
        
        Ok(HealthStatus {
            is_healthy: is_connected,
            uptime_percent: 99.8,
            last_error: None,
            last_check: Utc::now(),
        })
    }

    fn get_config(&self) -> serde_json::Value {
        serde_json::to_value(&self.config).unwrap_or_default()
    }
}

// TODO: Implement StorjNodeManager for node operations
// TODO: Implement reputation tracking
// TODO: Add comprehensive unit tests
2.4 Golem Network Adapter
Create src/protocols/golem.rs:
rustuse super::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GolemConfig {
    pub provider_node_url: String,
    pub wallet_address: String,
    pub cpu_cores: u32,
    pub memory_gb: f64,
    pub gpu_enabled: bool,
    pub pricing_model: PricingModel,
    pub min_allocation_percent: f64,
    pub max_allocation_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingModel {
    pub per_hour_usd: f64,
    pub per_cpu_hour_usd: f64,
    pub start_price_usd: f64,
}

#[derive(Debug)]
struct GolemMetrics {
    tasks_completed: u64,
    compute_hours: f64,
    total_earnings: f64,
    active_tasks: u32,
}

pub struct GolemAdapter {
    config: GolemConfig,
    client: Client,
    status: Arc<RwLock<ConnectionStatus>>,
    metrics: Arc<RwLock<GolemMetrics>>,
    current_allocation: Arc<RwLock<AllocationStrategy>>,
}

impl GolemAdapter {
    pub fn new(config: GolemConfig) -> Self {
        Self {
            config,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            metrics: Arc::new(RwLock::new(GolemMetrics {
                tasks_completed: 0,
                compute_hours: 0.0,
                total_earnings: 0.0,
                active_tasks: 0,
            })),
            current_allocation: Arc::new(RwLock::new(AllocationStrategy {
                cpu_percent: 40.0,
                memory_percent: 40.0,
                bandwidth_percent: 15.0,
                storage_percent: 5.0,
                priority_level: 5,
                optimization_params: HashMap::new(),
            })),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for GolemAdapter {
    fn protocol_name(&self) -> &str {
        "golem"
    }

    async fn connect(&mut self) -> Result<(), ProtocolError> {
        info!("Connecting to Golem Network...");
        
        // TODO: Start provider daemon
        // TODO: Register with Golem Network
        // TODO: Configure pricing
        
        *self.status.write().await = ConnectionStatus::Connecting;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        *self.status.write().await = ConnectionStatus::Connected;
        
        info!("Successfully connected to Golem Network");
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), ProtocolError> {
        info!("Disconnecting from Golem Network...");
        
        // TODO: Complete active tasks
        // TODO: Stop accepting new tasks
        // TODO: Shutdown provider daemon
        
        *self.status.write().await = ConnectionStatus::Disconnected;
        info!("Disconnected from Golem Network");
        
        Ok(())
    }

    async fn connection_status(&self) -> ConnectionStatus {
        self.status.read().await.clone()
    }

    async fn get_current_earnings(&self) -> Result<EarningsData, ProtocolError> {
        debug!("Fetching current Golem earnings...");
        
        // TODO: Query provider daemon for earnings
        // TODO: Convert GLM to USD
        // TODO: Calculate hourly rate from recent tasks
        
        Ok(EarningsData {
            timestamp: Utc::now(),
            earnings_usd: 0.0,
            earnings_native: 0.0,
            native_token_symbol: "GLM".to_string(),
            hourly_rate_usd: 0.0,
            protocol_specific: HashMap::new(),
        })
    }

    async fn get_historical_earnings(
        &self,
        hours: u32,
    ) -> Result<Vec<EarningsData>, ProtocolError> {
        debug!("Fetching {} hours of Golem earnings history...", hours);
        Ok(Vec::new())
    }

    async fn get_resource_usage(&self) -> Result<ResourceMetrics, ProtocolError> {
        debug!("Collecting Golem resource metrics...");
        
        // TODO: Measure CPU usage from active tasks
        // TODO: Track memory consumption
        // TODO: Monitor GPU usage if enabled
        
        Ok(ResourceMetrics {
            timestamp: Utc::now(),
            cpu_percent: 35.0,
            memory_mb: 4096.0,
            bandwidth_mbps: 5.0,
            storage_gb: 10.0,
            disk_io_mbps: Some(20.0),
            network_latency_ms: Some(40.0),
        })
    }

    async fn apply_allocation(
        &mut self,
        allocation: AllocationStrategy,
    ) -> Result<(), ProtocolError> {
        info!("Applying new Golem allocation: {:?}", allocation);
        
        // Validate CPU allocation
        if allocation.cpu_percent < self.config.min_allocation_percent
            || allocation.cpu_percent > self.config.max_allocation_percent
        {
            return Err(ProtocolError::AllocationError(format!(
                "CPU allocation {}% outside allowed range {}-{}%",
                allocation.cpu_percent,
                self.config.min_allocation_percent,
                self.config.max_allocation_percent
            )));
        }
        
        // TODO: Adjust CPU core allocation
        // TODO: Update memory limits
        // TODO: Adjust pricing based on demand
        
        *self.current_allocation.write().await = allocation;
        info!("Golem allocation applied successfully");
        
        Ok(())
    }

    async fn get_current_allocation(&self) -> Result<AllocationStrategy, ProtocolError> {
        Ok(self.current_allocation.read().await.clone())
    }

    async fn health_check(&self) -> Result<HealthStatus, ProtocolError> {
        debug!("Performing Golem health check...");
        
        let status = self.status.read().await;
        let is_connected = matches!(*status, ConnectionStatus::Connected);
        
        // TODO: Check provider daemon status
        // TODO: Verify task completion rate
        // TODO: Check network connectivity
        
        Ok(HealthStatus {
            is_healthy: is_connected,
            uptime_percent: 99.5,
            last_error: None,
            last_check: Utc::now(),
        })
    }

    fn get_config(&self) -> serde_json::Value {
        serde_json::to_value(&self.config).unwrap_or_default()
    }
}

// TODO: Implement dynamic pricing strategy
// TODO: Implement task acceptance logic
// TODO: Add comprehensive unit tests
2.5 Grass Network Adapter
Create src/protocols/grass.rs:
rustuse super::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrassConfig {
    pub api_key: String,
    pub api_endpoint: String,
    pub max_bandwidth_mbps: f64,
    pub min_allocation_percent: f64,
    pub max_allocation_percent: f64,
}

#[derive(Debug)]
struct GrassMetrics {
    bandwidth_shared_gb: f64,
    requests_proxied: u64,
    uptime_hours: f64,
    earnings_points: f64,
}

pub struct GrassAdapter {
    config: GrassConfig,
    client: Client,
    status: Arc<RwLock<ConnectionStatus>>,
    metrics: Arc<RwLock<GrassMetrics>>,
    current_allocation: Arc<RwLock<AllocationStrategy>>,
}

impl GrassAdapter {
    pub fn new(config: GrassConfig) -> Self {
        Self {
            config,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            status: Arc::new(RwLock::new(ConnectionStatus::Disconnected)),
            metrics: Arc::new(RwLock::new(GrassMetrics {
                bandwidth_shared_gb: 0.0,
                requests_proxied: 0,
                uptime_hours: 0.0,
                earnings_points: 0.0,
            })),
            current_allocation: Arc::new(RwLock::new(AllocationStrategy {
                cpu_percent: 5.0,
                memory_percent: 10.0,
                bandwidth_percent: 30.0,
                storage_percent: 0.0,
                priority_level: 5,
                optimization_params: HashMap::new(),
            })),
        }
    }
}

#[async_trait]
impl ProtocolAdapter for GrassAdapter {
    fn protocol_name(&self) -> &str {
        "grass"
    }

    async fn connect(&mut self) -> Result<(), ProtocolError> {
        info!("Connecting to Grass Network...");
        
        // TODO: Authenticate with API key
        // TODO: Start proxy service
        // TODO: Register node
        
        *self.status.write().await = ConnectionStatus::Connecting;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        *self.status.write().await = ConnectionStatus::Connected;
        
        info!("Successfully connected to Grass Network");
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), ProtocolError> {
        info!("Disconnecting from Grass Network...");
        
        // TODO: Stop accepting new requests
        // TODO: Complete active proxy sessions
        // TODO: Shutdown proxy service
        
        *self.status.write().await = ConnectionStatus::Disconnected;
        info!("Disconnected from Grass Network");
        
        Ok(())
    }

    async fn connection_status(&self) -> ConnectionStatus {
        self.status.read().await.clone()
    }

    async fn get_current_earnings(&self) -> Result<EarningsData, ProtocolError> {
        debug!("Fetching current Grass earnings...");
        
        // TODO: Query Grass API for earnings points
        // TODO: Convert points to USD equivalent
        // TODO: Calculate hourly rate
        
        Ok(EarningsData {
            timestamp: Utc::now(),
            earnings_usd: 0.0,
            earnings_native: 0.0,
            native_token_symbol: "GRASS_POINTS".to_string(),
            hourly_rate_usd: 0.0,
            protocol_specific: HashMap::new(),
        })
    }

    async fn get_historical_earnings(
        &self,
        hours: u32,
    ) -> Result<Vec<EarningsData>, ProtocolError> {
        debug!("Fetching {} hours of Grass earnings history...", hours);
        Ok(Vec::new())
    }

    async fn get_resource_usage(&self) -> Result<ResourceMetrics, ProtocolError> {
        debug!("Collecting Grass resource metrics...");
        
        // TODO: Track bandwidth usage
        // TODO: Measure CPU usage for proxy operations
        // TODO: Monitor memory usage
        
        Ok(ResourceMetrics {
            timestamp: Utc::now(),
            cpu_percent: 3.0,
            memory_mb: 150.0,
            bandwidth_mbps: 20.0,
            storage_gb: 0.0,
            disk_io_mbps: None,
            network_latency_ms: Some(25.0),
        })
    }

    async fn apply_allocation(
        &mut self,
        allocation: AllocationStrategy,
    ) -> Result<(), ProtocolError> {
        info!("Applying new Grass allocation: {:?}", allocation);
        
        // Validate bandwidth allocation
        if allocation.bandwidth_percent < self.config.min_allocation_percent
            || allocation.bandwidth_percent > self.config.max_allocation_percent
        {
            return Err(ProtocolError::AllocationError(format!(
                "Bandwidth allocation {}% outside allowed range {}-{}%",
                allocation.bandwidth_percent,
                self.config.min_allocation_percent,
                self.config.max_allocation_percent
            )));
        }
        
        // TODO: Adjust bandwidth throttling
        // TODO: Update rate limits
        
        *self.current_allocation.write().await = allocation;
        info!("Grass allocation applied successfully");
        
        Ok(())
    }

    async fn get_current_allocation(&self) -> Result<AllocationStrategy, ProtocolError> {
        Ok(self.current_allocation.read().await.clone())
    }

    async fn health_check(&self) -> Result<HealthStatus, ProtocolError> {
        debug!("Performing Grass health check...");
        
        let status = self.status.read().await;
        let is_connected = matches!(*status, ConnectionStatus::Connected);
        
        // TODO: Check proxy service status
        // TODO: Verify API connectivity
        // TODO: Monitor request success rate
        
        Ok(HealthStatus {
            is_healthy: is_connected,
            uptime_percent: 99.7,
            last_error: None,
            last_check: Utc::now(),
        })
    }

    fn get_config(&self) -> serde_json::Value {
        serde_json::to_value(&self.config).unwrap_or_default()
    }
}

// TODO: Implement request filtering and logging
// TODO: Implement IP reputation management
// TODO: Add comprehensive unit tests

Phase 3: ML-Driven Optimization Engine [REF:P3-ML]
3.1 Project Structure
Create ML engine directory structure:
bashmkdir -p src/ml/{models,utils,api}
touch src/ml/__init__.py
touch src/ml/models/{__init__,earnings_predictor,resource_allocator,anomaly_detector}.py
touch src/ml/utils/{__init__,data_loader,feature_engineering,metrics}.py
touch src/ml/api/{__init__,main,schemas}.py
3.2 Earnings Predictor
Create src/ml/models/earnings_predictor.py:
python"""
LSTM-based earnings prediction model for DePIN protocols.

This module implements time-series forecasting to predict hourly earnings
for each protocol based on historical data and engineered features.
"""

import numpy as np
import pandas as pd
import tensorflow as tf
from tensorflow import keras
from sklearn.preprocessing import StandardScaler
from sklearn.model_selection import TimeSeriesSplit
from typing import List, Dict, Tuple, Optional
import joblib
from pathlib import Path
import logging

logger = logging.getLogger(__name__)


class EarningsPredictor:
    """
    LSTM neural network for predicting protocol earnings.
    
    Features:
    - Multi-protocol support (separate models per protocol)
    - Time-series based feature engineering
    - Rolling window predictions
    - Model versioning and persistence
    """
    
    def __init__(
        self,
        protocol_name: str,
        sequence_length: int = 24,
        model_dir: str = "models",
    ):
        self.protocol_name = protocol_name
        self.sequence_length = sequence_length
        self.model_dir = Path(model_dir)
        self.model_dir.mkdir(parents=True, exist_ok=True)
        
        self.model: Optional[keras.Model] = None
        self.scaler = StandardScaler()
        self.feature_names: List[str] = []
        
    def _build_model(self, input_shape: Tuple[int, int]) -> keras.Model:
        """
        Build LSTM neural network architecture.
        
        Architecture:
        - Input: (sequence_length, n_features)
        - LSTM layer 1: 128 units, return_sequences=True
        - Dropout: 0.2
        - LSTM layer 2: 64 units
        - Dropout: 0.2
        - Dense layer: 32 units, ReLU activation
        - Output layer: 1 unit (earnings prediction)
        
        Args:
            input_shape: Shape of input sequences (timesteps, features)
            
        Returns:
            Compiled Keras model
        """
        model = keras.Sequential([
            keras.layers.LSTM(
                128,
                return_sequences=True,
                input_shape=input_shape,
                name="lstm_1"
            ),
            keras.layers.Dropout(0.2, name="dropout_1"),
            keras.layers.LSTM(64, name="lstm_2"),
            keras.layers.Dropout(0.2, name="dropout_2"),
            keras.layers.Dense(32, activation="relu", name="dense_1"),
            keras.layers.Dense(1, name="output")
        ])
        
        model.compile(
            optimizer=keras.optimizers.Adam(learning_rate=0.001),
            loss="mse",
            metrics=["mae", "mape"]
        )
        
        logger.info(f"Built LSTM model for {self.protocol_name}")
        logger.info(f"Model parameters: {model.count_params():,}")
        
        return model
    
    def prepare_sequences(
        self,
        df: pd.DataFrame,
        target_col: str = "earnings_usd"
    ) -> Tuple[np.ndarray, np.ndarray]:
        """
        Prepare time-series sequences for training.
        
        Args:
            df: DataFrame with time-series data
            target_col: Column name for prediction target
            
        Returns:
            Tuple of (X sequences, y targets)
        """
        # Extract features and target
        feature_cols = [col for col in df.columns if col != target_col]
        self.feature_names = feature_cols
        
        features = df[feature_cols].values
        target = df[target_col].values
        
        # Scale features
        features_scaled = self.scaler.fit_transform(features)
        
        # Create sequences
        X, y = [], []
        for i in range(len(features_scaled) - self.sequence_length):
            X.append(features_scaled[i:i + self.sequence_length])
            y.append(target[i + self.sequence_length])
        
        return np.array(X), np.array(y)
    
    def train(
        self,
        df: pd.DataFrame,
        epochs: int = 100,
        batch_size: int = 32,
        validation_split: float = 0.2,
    ) -> Dict[str, List[float]]:
        """
        Train the earnings prediction model.
        
        Args:
            df: Training data DataFrame
            epochs: Number of training epochs
            batch_size: Training batch size
            validation_split: Fraction of data for validation
            
        Returns:
            Training history dictionary
        """
        logger.info(f"Training model for {self.protocol_name}")
        logger.info(f"Training samples: {len(df)}")
        
        # Prepare sequences
        X, y = self.prepare_sequences(df)
        logger.info(f"Sequence shape: {X.shape}, Target shape: {y.shape}")
        
        # Build model if not exists
        if self.model is None:
            input_shape = (X.shape[1], X.shape[2])
            self.model = self._build_model(input_shape)
        
        # Callbacks
        callbacks = [
            keras.callbacks.EarlyStopping(
                monitor="val_loss",
                patience=10,
                restore_best_weights=True,
                verbose=1
            ),
            keras.callbacks.ModelCheckpoint(
                self.model_dir / f"{self.protocol_name}_best.h5",
                monitor="val_loss",
                save_best_only=True,
                verbose=1
            ),
            keras.callbacks.ReduceLROnPlateau(
                monitor="val_loss",
                factor=0.5,
                patience=5,
                min_lr=1e-6,
                verbose=1
            )
        ]
        
        # Train
        history = self.model.fit(
            X, y,
            epochs=epochs,
            batch_size=batch_size,
            validation_split=validation_split,
            callbacks=callbacks,
            verbose=1
        )
        
        # Save final model and scaler
        self.save()
        
        logger.info(f"Training complete for {self.protocol_name}")
        logger.info(f"Final loss: {history.history['loss'][-1]:.4f}")
        logger.info(f"Final MAE: {history.history['mae'][-1]:.4f}")
        
        return history.history
    
    def predict_next_24h(
        self,
        recent_data: pd.DataFrame
    ) -> List[float]:
        """
        Predict hourly earnings for the next 24 hours.
        
        Args:
            recent_data: Recent historical data (last sequence_length hours)
            
        Returns:
            List of 24 hourly earnings predictions
        """
        if self.model is None:
            raise ValueError("Model not trained or loaded")
        
        if len(recent_data) < self.sequence_length:
            raise ValueError(
                f"Need at least {self.sequence_length} hours of recent data"
            )
        
        # Prepare input sequence
        features = recent_data[self.feature_names].values[-self.sequence_length:]
        features_scaled = self.scaler.transform(features)
        X = np.expand_dims(features_scaled, axis=0)
        
        # Predict next 24 hours iteratively
        predictions = []
        for _ in range(24):
            pred = self.model.predict(X, verbose=0)[0, 0]
            predictions.append(float(pred))
            
            # TODO: Update sequence with prediction for next iteration
            # This is a simplified version - in production, you'd need to
            # engineer features for the predicted time step
        
        logger.info(f"Generated 24-hour predictions for {self.protocol_name}")
        logger.info(f"Predicted range: ${min(predictions):.2f} - ${max(predictions):.2f}")
        
        return predictions
    
    def evaluate(
        self,
        test_df: pd.DataFrame
    ) -> Dict[str, float]:
        """
        Evaluate model performance on test data.
        
        Args:
            test_df: Test dataset
            
        Returns:
            Dictionary of evaluation metrics
        """
        if self.model is None:
            raise ValueError("Model not trained or loaded")
        
        X_test, y_test = self.prepare_sequences(test_df)
        
        results = self.model.evaluate(X_test, y_test, verbose=0)
        
        metrics = {
            "loss": results[0],
            "mae": results[1],
            "mape": results[2]
        }
        
        # Calculate additional metrics
        y_pred = self.model.predict(X_test, verbose=0).flatten()
        
        from sklearn.metrics import r2_score, mean_absolute_percentage_error
        
        metrics["r2_score"] = r2_score(y_test, y_pred)
        metrics["rmse"] = np.sqrt(metrics["loss"])
        
        logger.info(f"Evaluation metrics for {self.protocol_name}:")
        for metric, value in metrics.items():
            logger.info(f"  {metric}: {value:.4f}")
        
        return metrics
    
    def save(self):
        """Save model and scaler to disk."""
        self.model.save(self.model_dir / f"{self.protocol_name}_model.h5")
        joblib.dump(
            self.scaler,
            self.model_dir / f"{self.protocol_name}_scaler.pkl"
        )
        joblib.dump(
            self.feature_names,
            self.model_dir / f"{self.protocol_name}_features.pkl"
        )
        logger.info(f"Saved model for {self.protocol_name}")
    
    def load(self):
        """Load model and scaler from disk."""
        model_path = self.model_dir / f"{self.protocol_name}_model.h5"
        scaler_path = self.model_dir / f"{self.protocol_name}_scaler.pkl"
        features_path = self.model_dir / f"{self.protocol_name}_features.pkl"
        
        if not model_path.exists():
            raise FileNotFoundError(f"Model not found: {model_path}")
        
        self.model = keras.models.load_model(model_path)
        self.scaler = joblib.load(scaler_path)
        self.feature_names = joblib.load(features_path)
        
        logger.info(f"Loaded model for {self.protocol_name}")


# TODO: Implement feature engineering utilities
# TODO: Add hyperparameter tuning
# TODO: Implement ensemble predictions
# TODO: Add comprehensive unit tests
3.3 Resource Allocator
Create src/ml/models/resource_allocator.py:
python"""
Convex optimization-based resource allocator for DePIN protocols.

This module uses CVXPY to solve the resource allocation problem,
maximizing expected earnings while respecting hardware constraints.
"""

import cvxpy as cp
import numpy as np
from typing import Dict, List, Tuple, Optional
import logging
from dataclasses import dataclass

logger = logging.getLogger(__name__)


@dataclass
class ResourceConstraints:
    """Hardware resource constraints."""
    total_cpu_cores: float
    total_memory_gb: float
    total_bandwidth_mbps: float
    total_storage_gb: float


@dataclass
class ProtocolRequirements:
    """Resource requirements per protocol."""
    cpu_per_unit: float
    memory_per_unit: float
    bandwidth_per_unit: float
    storage_per_unit: float
    min_allocation: float  # Minimum allocation (0-1)
    max_allocation: float  # Maximum allocation (0-1)


@dataclass
class AllocationResult:
    """Optimization result."""
    allocations: Dict[str, float]  # Protocol -> allocation percentage
    expected_earnings: float
    resource_utilization: Dict[str, float]
    is_optimal: bool
    solver_status: str


class ResourceAllocator:
    """
    Convex optimization-based resource allocator.
    
    Solves the problem:
        Maximize: Σ(earnings_i × allocation_i) for all protocols
        
        Subject to:
        - Σ(cpu_i × allocation_i) ≤ total_cpu
        - Σ(memory_i × allocation_i) ≤ total_memory
        - Σ(bandwidth_i × allocation_i) ≤ total_bandwidth
        - Σ(storage_i × allocation_i) ≤ total_storage
        - min_i ≤ allocation_i ≤ max_i for all i
        - 0 ≤ allocation_i ≤ 1 for all i
    """
    
    def __init__(self, constraints: ResourceConstraints):
        self.constraints = constraints
        logger.info("Initialized ResourceAllocator with constraints:")
        logger.info(f"  CPU: {constraints.total_cpu_cores} cores")
        logger.info(f"  Memory: {constraints.total_memory_gb} GB")
        logger.info(f"  Bandwidth: {constraints.total_bandwidth_mbps} Mbps")
        logger.info(f"  Storage: {constraints.total_storage_gb} GB")
    
    def optimize_allocation(
        self,
        earnings_predictions: Dict[str, float],
        requirements: Dict[str, ProtocolRequirements],
        current_allocation: Optional[Dict[str, float]] = None,
    ) -> AllocationResult:
        """
        Optimize resource allocation to maximize expected earnings.
        
        Args:
            earnings_predictions: Predicted hourly earnings per protocol (USD)
            requirements: Resource requirements per protocol
            current_allocation: Current allocation (for reallocation cost calculation)
            
        Returns:
            AllocationResult with optimal allocations
        """
        protocols = list(earnings_predictions.keys())
        n = len(protocols)
        
        logger.info(f"Optimizing allocation for {n} protocols")
        logger.info(f"Earnings predictions: {earnings_predictions}")
        
        # Decision variables: allocation percentage (0-1) for each protocol
        allocations = cp.Variable(n, name="allocations")
        
        # Objective: Maximize total expected earnings
        earnings_vec = np.array([earnings_predictions[p] for p in protocols])
        objective = cp.Maximize(earnings_vec @ allocations)
        
        # Constraints
        constraints = []
        
        # Resource constraints
        cpu_usage = sum(
            allocations[i] * requirements[p].cpu_per_unit
            for i, p in enumerate(protocols)
        )
        constraints.append(cpu_usage <= self.constraints.total_cpu_cores)
        
        memory_usage = sum(
            allocations[i] * requirements[p].memory_per_unit
            for i, p in enumerate(protocols)
        )
        constraints.append(memory_usage <= self.constraints.total_memory_gb)
        
        bandwidth_usage = sum(
            allocations[i] * requirements[p].bandwidth_per_unit
            for i, p in enumerate(protocols)
        )
        constraints.append(bandwidth_usage <= self.constraints.total_bandwidth_mbps)
        
        storage_usage = sum(
            allocations[i] * requirements[p].storage_per_unit
            for i, p in enumerate(protocols)
        )
        constraints.append(storage_usage <= self.constraints.total_storage_gb)
        
        # Allocation bounds for each protocol
        for i, protocol in enumerate(protocols):
            req = requirements[protocol]
            constraints.append(allocations[i] >= req.min_allocation)
            constraints.append(allocations[i] <= req.max_allocation)
        
        # General bounds: 0 <= allocation <= 1
        constraints.append(allocations >= 0)
        constraints.append(allocations <= 1)
        
        # Solve optimization problem
        problem = cp.Problem(objective, constraints)
        
        try:
            problem.solve(solver=cp.ECOS, verbose=False)
            
            if problem.status == cp.OPTIMAL:
                logger.info("✓ Found optimal allocation")
                
                # Extract results
                allocation_dict = {
                    protocol: float(allocations.value[i])
                    for i, protocol in enumerate(protocols)
                }
                
                expected_earnings = float(problem.value)
                
                # Calculate resource utilization
                resource_util = {
                    "cpu": float(cpu_usage.value / self.constraints.total_cpu_cores * 100),
                    "memory": float(memory_usage.value / self.constraints.total_memory_gb * 100),
                    "bandwidth": float(bandwidth_usage.value / self.constraints.total_bandwidth_mbps * 100),
                    "storage": float(storage_usage.value / self.constraints.total_storage_gb * 100),
                }
                
                logger.info(f"Expected earnings: ${expected_earnings:.2f}/hour")
                logger.info("Allocations:")
                for protocol, alloc in allocation_dict.items():
                    logger.info(f"  {protocol}: {alloc*100:.1f}%")
                logger.info("Resource utilization:")
                for resource, util in resource_util.items():
                    logger.info(f"  {resource}: {util:.1f}%")
                
                return AllocationResult(
                    allocations=allocation_dict,
                    expected_earnings=expected_earnings,
                    resource_utilization=resource_util,
                    is_optimal=True,
                    solver_status=problem.status
                )
            
            else:
                logger.warning(f"Optimization failed: {problem.status}")
                # Return current allocation if available, otherwise equal split
                if current_allocation:
                    allocation_dict = current_allocation
                else:
                    allocation_dict = {p: 1.0/n for p in protocols}
                
                return AllocationResult(
                    allocations=allocation_dict,
                    expected_earnings=0.0,
                    resource_utilization={},
                    is_optimal=False,
                    solver_status=problem.status
                )
        
        except Exception as e:
            logger.error(f"Optimization error: {e}")
            # Fallback to current allocation
            if current_allocation:
                allocation_dict = current_allocation
            else:
                allocation_dict = {p: 1.0/n for p in protocols}
            
            return AllocationResult(
                allocations=allocation_dict,
                expected_earnings=0.0,
                resource_utilization={},
                is_optimal=False,
                solver_status="ERROR"
            )
    
    def should_reallocate(
        self,
        current_allocation: Dict[str, float],
        new_allocation: Dict[str, float],
        current_earnings: float,
        predicted_earnings: float,
        reallocation_cost_usd: float = 0.0,
    ) -> Tuple[bool, Dict[str, any]]:
        """
        Determine if reallocation is beneficial.
        
        Args:
            current_allocation: Current allocation percentages
            new_allocation: Proposed new allocation
            current_earnings: Current hourly earnings (USD)
            predicted_earnings: Predicted earnings with new allocation (USD)
            reallocation_cost_usd: Cost of switching allocations
            
        Returns:
            Tuple of (should_reallocate, analysis_dict)
        """
        # Calculate improvement
        improvement = predicted_earnings - current_earnings
        improvement_percent = (improvement / current_earnings * 100) if current_earnings > 0 else 0
        
        # Calculate allocation difference
        protocols = set(current_allocation.keys()) | set(new_allocation.keys())
        max_change = max(
            abs(new_allocation.get(p, 0) - current_allocation.get(p, 0))
            for p in protocols
        )
        
        # Decision criteria
        criteria = {
            "earnings_improvement_usd": improvement,
            "earnings_improvement_percent": improvement_percent,
            "max_allocation_change": max_change * 100,
            "reallocation_cost_usd": reallocation_cost_usd,
            "net_benefit_usd": improvement - reallocation_cost_usd,
        }
        
        # Decision logic
        should_switch = (
            improvement_percent > 10.0  # At least 10% improvement
            and (improvement - reallocation_cost_usd) > 0.5  # Net benefit > $0.50/hour
        )
        
        logger.info("Reallocation analysis:")
        logger.info(f"  Earnings improvement: ${improvement:.2f}/hour ({improvement_percent:.1f}%)")
        logger.info(f"  Max allocation change: {max_change*100:.1f}%")
        logger.info(f"  Reallocation cost: ${reallocation_cost_usd:.2f}")
        logger.info(f"  Net benefit: ${criteria['net_benefit_usd']:.2f}/hour")
        logger.info(f"  Decision: {'REALLOCATE' if should_switch else 'KEEP CURRENT'}")
        
        return should_switch, criteria
    
    def calculate_reallocation_cost(
        self,
        current_allocation: Dict[str, float],
        new_allocation: Dict[str, float],
        protocol_restart_times: Dict[str, float],  # seconds
        hourly_earnings: Dict[str, float],  # USD/hour
    ) -> float:
        """
        Calculate the cost of reallocating resources.
        
        Cost includes:
        - Lost earnings during protocol restarts
        - Resource migration overhead
        
        Args:
            current_allocation: Current allocation
            new_allocation: Proposed allocation
            protocol_restart_times: Time to restart each protocol (seconds)
            hourly_earnings: Current hourly earnings per protocol
            
        Returns:
            Reallocation cost in USD
        """
        total_cost = 0.0
        
        protocols = set(current_allocation.keys()) | set(new_allocation.keys())
        
        for protocol in protocols:
            current = current_allocation.get(protocol, 0)
            new = new_allocation.get(protocol, 0)
            
            # If allocation changes significantly, account for restart cost
            if abs(new - current) > 0.05:  # 5% threshold
                restart_time_hours = protocol_restart_times.get(protocol, 60) / 3600
                lost_earnings = hourly_earnings.get(protocol, 0) * restart_time_hours
                total_cost += lost_earnings
        
        return total_cost


# TODO: Add sensitivity analysis
# TODO: Implement multi-objective optimization
# TODO: Add portfolio optimization strategies
# TODO: Comprehensive unit tests
Due to character limits, I'll continue with the remaining phases in a structured summary format.

Remaining Phases Summary [REF:P4-P8]
Phase 4: Core Orchestrator Engine

Main orchestrator (src/core/orchestrator.rs): Event loop, metrics collection, optimization cycles
Configuration management (src/core/config.rs): TOML parsing, validation
Database operations (src/core/database.rs): SQLx migrations, CRUD operations
Logging and monitoring (src/core/monitoring.rs): Prometheus metrics, structured logging

Phase 5: API & User Interface

FastAPI backend (src/api/main.py): REST endpoints, WebSocket updates
React dashboard (src/ui/): Tauri app, real-time visualization, Recharts
API schemas (src/api/schemas.py): Pydantic models for validation

Phase 6: Testing & Quality Assurance

Rust unit tests: 80%+ coverage target
Python unit tests: pytest with 85%+ coverage
Integration tests: End-to-end workflows
Performance tests: Load testing, benchmarks

Phase 7: DevOps & Deployment

Docker configurations: Multi-stage builds
docker-compose.yml: Full stack orchestration
CI/CD pipeline (.github/workflows/ci-cd.yml): Automated testing and deployment
Monitoring setup: Prometheus + Grafana dashboards

Phase 8: Documentation

Technical docs: Architecture, API reference, deployment guide
User guide: Installation, configuration, usage
Code documentation: Comprehensive docstrings and comments


Implementation Execution Strategy [REF:EXEC]
Copilot Autonomy Guidelines
You have full autonomy to:

Make implementation decisions within architectural boundaries
Choose algorithms and data structures
Design internal APIs
Write comprehensive tests
Add logging and error handling
Optimize performance

Consult human for:

Major architectural changes
Security-sensitive implementations
External API integrations requiring credentials
Production deployment decisions

Sequential Implementation Order

Phase 1: Foundation (2-3 days) ✓
Phase 2: Protocol Adapters (7-10 days)
Phase 3: ML Engine (5-7 days)
Phase 4: Core Orchestrator (5-7 days)
Phase 5: API & UI (7-10 days)
Phase 6: Testing (3-5 days, ongoing)
Phase 7: DevOps (2-3 days)
Phase 8: Documentation (2-3 days, ongoing)

Quality Checkpoints
Before each phase:

 All tests passing
 Code coverage meets targets
 No critical linting errors
 Documentation complete
 Code committed to Git

Completion Criteria
Project complete when:

 All 8 phases implemented
 All tests passing
 Docker deployment functional
 Dashboard operational
 Optimizing across all protocols
 24+ hours autonomous operation
 Earnings tracking functional


🚀 BEGIN IMPLEMENTATION
First Tasks (Phase 1):

✓ Clone repository
Create directory structure
Initialize dependency files
Setup development environment
Create initial configuration
Implement logging infrastructure

Proceed autonomously through each phase, reporting completion before advancing.
