#!/bin/bash
set -e

echo "🚀 DePIN Orcha Deployment"
echo "============================"

# Build Rust project
echo "🦀 Building Rust project..."
cargo build --release

# Build Python ML engine
echo "🐍 Preparing Python ML engine..."
pip install -r requirements.txt -q

# Build UI
echo "⚛️  Building UI..."
cd src/ui
npm run build
cd ../..

# Build Docker images
echo "🐳 Building Docker images..."
docker-compose build

# Start services
echo "🚀 Starting services..."
docker-compose up -d

echo ""
echo "✅ Deployment complete!"
echo ""
echo "Services:"
echo "  • Orchestrator: http://localhost:6701"
echo "  • ML Engine: http://localhost:6702"
echo "  • Prometheus: http://localhost:6703"
echo "  • Grafana: http://localhost:6704"
echo ""
echo "View logs: docker-compose logs -f"
