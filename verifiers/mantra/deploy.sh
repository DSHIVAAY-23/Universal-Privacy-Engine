#!/bin/bash
# Deployment script for RWA Verifier on Mantra Testnet

set -e

echo "🕉️  RWA Verifier - Mantra Testnet Deployment"
echo "============================================"
echo ""

# Check if CosmWasm tools are installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Install Rust from: https://rustup.rs/"
    exit 1
fi

# Build the contract
echo "🔨 Building CosmWasm contract..."
cargo build --release --target wasm32-unknown-unknown

# Optimize the WASM
echo "⚡ Optimizing WASM..."
if command -v wasm-opt &> /dev/null; then
    wasm-opt -Oz target/wasm32-unknown-unknown/release/rwa_verifier_mantra.wasm \
        -o target/wasm32-unknown-unknown/release/rwa_verifier_mantra_optimized.wasm
else
    echo "⚠️  wasm-opt not found. Using unoptimized WASM."
    echo "Install from: https://github.com/WebAssembly/binaryen"
    cp target/wasm32-unknown-unknown/release/rwa_verifier_mantra.wasm \
       target/wasm32-unknown-unknown/release/rwa_verifier_mantra_optimized.wasm
fi

# Deploy to testnet (requires mantrachaind CLI)
echo ""
echo "📤 To deploy to Mantra Testnet, run:"
echo "  mantrachaind tx wasm store \\"
echo "    target/wasm32-unknown-unknown/release/rwa_verifier_mantra_optimized.wasm \\"
echo "    --from <YOUR_KEY> \\"
echo "    --chain-id mantra-testnet-1 \\"
echo "    --gas auto \\"
echo "    --gas-adjustment 1.3"
echo ""
echo "Then instantiate with:"
echo "  mantrachaind tx wasm instantiate <CODE_ID> \\"
echo "    '{\"verification_key\":\"<VKEY_BASE64>\"}' \\"
echo "    --from <YOUR_KEY> \\"
echo "    --label \"RWA Verifier\" \\"
echo "    --chain-id mantra-testnet-1 \\"
echo "    --gas auto"
echo ""
echo "✅ Build complete!"
