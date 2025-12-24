#!/bin/bash
# Deployment script for RWA Verifier on Stellar Testnet

set -e

echo "🌟 RWA Verifier - Stellar Testnet Deployment"
echo "============================================="
echo ""

# Check if Stellar CLI is installed
if ! command -v stellar &> /dev/null; then
    echo "❌ Stellar CLI not found. Install from: https://developers.stellar.org/docs/tools/developer-tools"
    exit 1
fi

# Build the contract
echo "🔨 Building Soroban contract..."
stellar contract build

# Optimize the WASM
echo "⚡ Optimizing WASM..."
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/rwa_verifier_stellar.wasm

# Deploy to testnet
echo ""
echo "📤 Deploying to Testnet..."
CONTRACT_ID=$(stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/rwa_verifier_stellar.optimized.wasm \
    --source <YOUR_SECRET_KEY> \
    --network testnet)

echo "Contract ID: $CONTRACT_ID"

# Initialize the contract (requires verification key)
echo ""
echo "⚙️  To initialize the verifier, run:"
echo "  stellar contract invoke \\"
echo "    --id $CONTRACT_ID \\"
echo "    --source <YOUR_SECRET_KEY> \\"
echo "    --network testnet \\"
echo "    -- initialize \\"
echo "    --admin <ADMIN_ADDRESS> \\"
echo "    --vkey <VKEY_JSON>"
echo ""
echo "✅ Deployment complete!"
echo "Explorer: https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
