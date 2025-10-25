#!/bin/bash

echo "Building Kanban Overlay..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed!"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "Building release version (optimized)..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "========================================"
    echo "Build successful!"
    echo "========================================"
    echo ""
    echo "Executable location:"
    echo "  target/release/kanban-overlay"
    echo ""
    echo "To run:"
    echo "  cargo run --release"
    echo "  or"
    echo "  ./target/release/kanban-overlay"
    echo ""
    echo "Press Ctrl+Shift+K to toggle the overlay"
    echo "Press Escape to hide it"
    echo ""
else
    echo ""
    echo "Build failed! Check the errors above."
    exit 1
fi
