# DePIN Orcha Development Guide

## Development Environment Setup

### Prerequisites

1. **Rust 1.75+**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   rustc --version
   ```

2. **Python 3.11+**

   ```bash
   python3 --version
   python3 -m venv venv
   source venv/bin/activate
   pip install -r requirements.txt
   ```

3. **Node.js 18+**

   ```bash
   node --version
   npm --version
   ```

4. **Docker & Docker Compose**
   ```bash
   docker --version
   docker-compose --version
   ```

### Initial Setup

```bash
# Clone repository
git clone https://github.com/iamthegreatdestroyer/DePIN-Orcha.git
cd DePIN-Orcha

# Run setup script
./scripts/setup-env.sh

# Verify setup
./scripts/health-check.sh
```

## Code Style & Standards

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Document public APIs with examples

```bash
cargo fmt
cargo clippy -- -D warnings
```

### Python

- Follow [PEP 8](https://www.python.org/dev/peps/pep-0008/)
- Use `black` for formatting
- Use `flake8` for linting
- Use `mypy` for type checking

```bash
black src/ml
flake8 src/ml
mypy src/ml
```

### JavaScript/TypeScript

- Use ESLint configuration
- Use Prettier for formatting
- Follow React best practices

```bash
cd src/ui
npm run lint
npm run format
npm run type-check
```

## Git Workflow

### Branch Naming

- `feature/feature-name` - New features
- `bugfix/bug-name` - Bug fixes
- `refactor/refactor-name` - Code refactoring
- `docs/doc-name` - Documentation updates

### Commit Messages

```
<type>(<scope>): <subject>

<body>

<footer>

Types: feat, fix, docs, style, refactor, test, chore
Scope: core, protocols, ml, api, ui, config, etc.

Example:
feat(protocols): implement Streamr adapter
- Add WebSocket connection handling
- Implement earnings tracking
- Add health checks

Closes #123
```

### Pull Requests

1. Create feature branch: `git checkout -b feature/my-feature`
2. Make changes with tests
3. Commit regularly: `git commit -m "..."`
4. Push: `git push origin feature/my-feature`
5. Create PR on GitHub
6. Address review comments
7. Merge after approval

## Testing

### Unit Tests

#### Rust

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

#### Python

```bash
# Run all tests
pytest

# Run specific test
pytest src/ml/tests/test_file.py::test_function

# Run with coverage
pytest --cov=src/ml --cov-report=html

# Run with verbose output
pytest -vv
```

### Integration Tests

```bash
# Run integration tests
cargo test --test '*'
pytest tests/integration/

# Run full integration test suite
./scripts/test-integration.sh
```

### Test Coverage

**Targets:**

- Rust: 80%+ line coverage
- Python: 85%+ line coverage
- Overall: 85%+ coverage

**Check coverage:**

```bash
cargo tarpaulin --out Html --output-dir coverage/
pytest --cov=src/ml --cov-report=html
```

## Running Locally

### Rust Orchestrator

```bash
# Development
cargo run

# With logging
RUST_LOG=debug cargo run

# Release build
cargo build --release
./target/release/depin-orcha
```

### Python ML Engine

```bash
cd src/ml

# Development
python -m uvicorn api.main:app --reload --port 8001

# Production
uvicorn api.main:app --host 0.0.0.0 --port 8001 --workers 4
```

### Frontend UI

```bash
cd src/ui

# Development
npm run dev

# Build
npm run build

# Preview
npm run preview
```

### Docker Services

```bash
# Start all services
docker-compose up

# Start in background
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Rebuild images
docker-compose up --build
```

## Debugging

### Rust Debugging

```bash
# With rust-gdb
rust-gdb ./target/debug/depin-orcha

# With rust-lldb
rust-lldb ./target/debug/depin-orcha

# With logging
RUST_LOG=trace cargo run
```

### Python Debugging

```bash
# With print debugging
print(variable)

# With pdb
import pdb; pdb.set_trace()

# With pytest
pytest --pdb

# With VS Code debugger
# Add configuration to .vscode/launch.json
```

### Database Inspection

```bash
# SQLite CLI
sqlite3 data/depin-orcha.db

# List tables
.tables

# Query data
SELECT * FROM earnings;

# Run migrations
sqlx migrate run
```

## API Testing

### Using curl

```bash
# Health check
curl http://localhost:6701/health

# Get metrics
curl http://localhost:6701/metrics

# Get ML predictions
curl -X POST http://localhost:6702/predict/earnings/streamr \
  -H "Content-Type: application/json" \
  -d '{"hours": 24}'
```

### Using Postman

1. Import collection from `docs/postman-collection.json`
2. Configure environment variables
3. Run requests and tests

### Using Python

```python
import requests

# Get health
response = requests.get('http://localhost:6701/health')
print(response.json())

# Get predictions
response = requests.post(
    'http://localhost:6702/predict/earnings/streamr',
    json={'hours': 24}
)
print(response.json())
```

## Documentation

### Code Documentation

- Add docstrings to all public functions
- Include examples where helpful
- Document error cases
- Document assumptions

Rust example:

````rust
/// Calculates optimal resource allocation.
///
/// Uses convex optimization to maximize expected earnings
/// while respecting hardware constraints.
///
/// # Arguments
/// * `earnings_predictions` - Predicted hourly earnings per protocol
/// * `constraints` - Hardware resource constraints
///
/// # Returns
/// Optimal allocation percentages per protocol
///
/// # Example
/// ```
/// let allocation = optimize_allocation(&predictions, &constraints)?;
/// ```
///
/// # Errors
/// Returns an error if optimization fails or constraints are infeasible.
pub fn optimize_allocation(
    earnings_predictions: &[f64],
    constraints: &ResourceConstraints,
) -> Result<Vec<f64>, OptimizationError> {
    // ...
}
````

### Markdown Documentation

- Keep docs up-to-date with code changes
- Use clear headings and structure
- Include code examples
- Link between documents

## Build Artifacts

### Location

- **Rust:** `target/release/`
- **Python:** `src/ml/` (no build step)
- **Frontend:** `src/ui/dist/`
- **Docker:** Docker Hub registry

### Cleanup

```bash
# Remove build artifacts
cargo clean
rm -rf src/ui/dist
rm -rf src/ml/__pycache__

# Clean Docker
docker system prune
```

## Performance Profiling

### Rust

```bash
# Build with optimizations
cargo build --release

# Profile with perf (Linux)
perf record -g ./target/release/depin-orcha
perf report

# Profile with cargo-flamegraph
cargo install flamegraph
cargo flamegraph
```

### Python

```bash
# Profile with cProfile
python -m cProfile -o ml_engine.prof -m uvicorn api.main:app

# Analyze with pstats
python -m pstats ml_engine.prof

# Profile with py-spy
pip install py-spy
py-spy record -o profile.svg -- python script.py
```

## Continuous Integration

See `.github/workflows/ci-cd.yml` for automated testing and deployment.

**Runs on:**

- Every push to develop/main
- Every pull request
- Daily schedule

**Tests:**

- Rust: `cargo test`, `cargo clippy`
- Python: `pytest`, code coverage
- UI: `npm run build`, lint checks
- Docker: Build all images

## Troubleshooting

### Common Issues

**Build fails:**

```bash
cargo clean
cargo update
cargo build
```

**Python import errors:**

```bash
source venv/bin/activate
pip install -r requirements.txt
```

**Docker issues:**

```bash
docker system prune
docker-compose down -v
docker-compose up --build
```

**Database errors:**

```bash
rm data/depin-orcha.db
sqlx database create
sqlx migrate run
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Python Docs](https://docs.python.org/3/)
- [React Docs](https://react.dev/)
- [Docker Docs](https://docs.docker.com/)
- [Prometheus Docs](https://prometheus.io/docs/)

---

**Last Updated:** January 13, 2026
