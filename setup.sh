#!/bin/bash
# Universal Privacy Engine - Initialization and Build Commands
# This script documents the commands to set up and verify the workspace

set -e

echo "🚀 Universal Privacy Engine - Setup Script"
echo "=========================================="
echo ""

# The workspace structure is already created with the following files:
# - Cargo.toml (root workspace)
# - core/Cargo.toml, core/src/lib.rs
# - adapters/sp1/Cargo.toml, adapters/sp1/src/lib.rs
# - cli/Cargo.toml, cli/src/main.rs

echo "✓ Workspace structure already created"
echo ""

# Verify the workspace builds
echo "📦 Building workspace..."
cargo build --workspace

echo ""
echo "✓ Build successful!"
echo ""

# Run tests
echo "🧪 Running tests..."
cargo test --workspace

echo ""
echo "✓ Tests passed!"
echo ""

# Check the CLI
echo "🔍 Verifying CLI..."
cargo run -p universal-privacy-engine-cli -- --help

echo ""
echo "✅ Universal Privacy Engine is ready!"
echo ""
echo "Next steps:"
echo "  1. Create a guest program (SP1 RISC-V program)"
echo "  2. Compile it to ELF: cargo prove build"
echo "  3. Generate a proof: cargo run -p universal-privacy-engine-cli -- prove --input <hex> --elf guest.elf"
echo "  4. Verify the proof: cargo run -p universal-privacy-engine-cli -- verify --receipt proof.bin --elf guest.elf"
echo ""
