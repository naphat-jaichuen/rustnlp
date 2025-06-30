#!/bin/bash

echo "=== RustLM UDP Broadcast Demo ==="
echo ""
echo "This demo will start the RustLM server and then show you how to"
echo "discover it using the UDP broadcast client."
echo ""

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust first."
    exit 1
fi

echo "📦 Building the project..."
cargo build --release --quiet

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi

echo "✅ Build successful!"
echo ""

echo "🚀 Starting the RustLM server..."
echo "   The server will start broadcasting its availability via UDP."
echo "   You can stop the server with Ctrl+C."
echo ""
echo "   To test UDP discovery, open another terminal and run:"
echo "   cargo run --example client_discovery"
echo ""
echo "Starting server in 3 seconds..."
sleep 3

# Start the server
cargo run --release
