#!/bin/bash
# Terraria 3D - Launch Script
set -e
echo "Terraria 3D"
echo "=========="
if ! command -v cargo &> /dev/null; then
    echo "Cargo not found. Please install Rust first."
    exit 1
fi
if [ ! -f "target/release/terraria_3d" ]; then
    echo "Building in release mode..."
    cargo build --release
fi
echo "Starting Terraria 3D..."
./target/release/terraria_3d
