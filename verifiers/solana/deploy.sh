#!/bin/bash
# Deployment script for RWA Verifier on Solana Devnet

set -e

echo "🚀 RWA Verifier - Solana Devnet Deployment"
echo "=========================================="
echo ""

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Install from: https://www.anchor-lang.com/docs/installation"
    exit 1
fi

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Install from: https://docs.solana.com/cli/install-solana-cli-tools"
    exit 1
fi

# Set cluster to devnet
echo "📡 Setting cluster to Devnet..."
solana config set --url https://api.devnet.solana.com

# Check wallet balance
echo ""
echo "💰 Checking wallet balance..."
BALANCE=$(solana balance | awk '{print $1}')
echo "Balance: $BALANCE SOL"

if (( $(echo "$BALANCE < 2" | bc -l) )); then
    echo "⚠️  Low balance! Requesting airdrop..."
    solana airdrop 2
    sleep 5
fi

# Build the program
echo ""
echo "🔨 Building Anchor program..."
anchor build

# Get program ID
PROGRAM_ID=$(solana address -k target/deploy/rwa_verifier-keypair.json)
echo "Program ID: $PROGRAM_ID"

# Deploy to devnet
echo ""
echo "📤 Deploying to Devnet..."
anchor deploy --provider.cluster devnet

# Initialize the verifier (requires verification key)
echo ""
echo "⚙️  To initialize the verifier, run:"
echo "  anchor run initialize --provider.cluster devnet"
echo ""
echo "✅ Deployment complete!"
echo "Program ID: $PROGRAM_ID"
echo "Explorer: https://explorer.solana.com/address/$PROGRAM_ID?cluster=devnet"
