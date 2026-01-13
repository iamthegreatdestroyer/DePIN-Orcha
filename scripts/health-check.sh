#!/bin/bash
set -e

echo "🏥 DePIN Orcha Health Check"
echo "================================"

# Check Rust compilation
echo -n "🦀 Checking Rust compilation... "
if cargo check >/dev/null 2>&1; then
    echo "✅"
else
    echo "❌ Build failed"
    exit 1
fi

# Check Python syntax
echo -n "🐍 Checking Python syntax... "
if python3 -m py_compile src/ml/**/*.py 2>/dev/null; then
    echo "✅"
else
    echo "⚠️  Warning: Some Python files have syntax issues"
fi

# Check Docker
echo -n "🐳 Checking Docker... "
if command -v docker >/dev/null 2>&1; then
    echo "✅"
else
    echo "❌ Docker not installed"
fi

# Check Node
echo -n "⚛️  Checking Node.js... "
if command -v node >/dev/null 2>&1; then
    echo "✅"
else
    echo "❌ Node.js not installed"
fi

# Check directories
echo -n "📁 Checking directories... "
dirs_ok=true
for dir in src config data models logs tests docs migrations; do
    if [ ! -d "$dir" ]; then
        echo "❌ Missing directory: $dir"
        dirs_ok=false
    fi
done
if [ "$dirs_ok" = true ]; then
    echo "✅"
fi

echo ""
echo "Health check complete!"
