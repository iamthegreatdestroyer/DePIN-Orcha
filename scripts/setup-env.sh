#!/bin/bash
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
source venv/bin/activate 2>/dev/null || . venv/Scripts/activate 2>/dev/null
pip install --upgrade pip
pip install -r requirements.txt
echo "✅ Python environment ready"

# Setup Rust dependencies
echo "🦀 Building Rust dependencies..."
cargo build 2>&1 | grep -v "warning:" || true
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
touch models/.gitkeep
echo "✅ Directories created"

# Create .env if it doesn't exist
if [ ! -f .env ]; then
    echo "⚙️  Creating .env file..."
    cat > .env << 'EOF'
# Environment Configuration
RUST_LOG=info,depin_orcha=debug
DATABASE_URL=sqlite:///data/depin-orcha.db
CONFIG_PATH=./config/default.toml
ML_API_URL=http://localhost:6702
PROMETHEUS_PORT=6703
EOF
    echo "✅ .env file created"
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Activate Python environment: source venv/bin/activate"
echo "  2. Configure settings in config/default.toml"
echo "  3. Run tests: cargo test && pytest"
echo "  4. Start development: cargo run"
