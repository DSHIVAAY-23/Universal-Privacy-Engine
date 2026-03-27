#!/usr/bin/env bash
# ── UPE Multi-Chain Deploy Script ─────────────────────────────────────────────
# Deploys RWAOracle to all supported testnets.
# Usage: ./deploy-all.sh <PRIVATE_KEY>
#   PRIVATE_KEY — hex deployer key (0x-prefixed)
#
# Prerequisites:
#   - Node 18+, Rust, cargo
#   - zkSync Sepolia ETH (https://portal.zksync.io/drpc)
#   - Secret Network testnet tokens (https://faucet.secretsaturn.net)
#   - Mina testnet tokens (https://faucet.minaprotocol.com)

set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

PRIVATE_KEY="${1:-${DEPLOYER_PRIVATE_KEY}}"
if [ -z "$PRIVATE_KEY" ] || [ "$PRIVATE_KEY" = "0x0000000000000000000000000000000000000000000000000000000000000000" ]; then
    echo "❌  No private key supplied."
    echo "    Usage: ./deploy-all.sh 0x<your-private-key>"
    echo "    Or:    export DEPLOYER_PRIVATE_KEY=0x... && ./deploy-all.sh"
    exit 1
fi

export DEPLOYER_PRIVATE_KEY="$PRIVATE_KEY"

echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║   UPE Multi-Chain RWA Oracle — Testnet Deploy   ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""

# ── 1. zkSync Era Sepolia ──────────────────────────────────────────────────────
echo "▶  [1/3] Deploying to zkSync Era Sepolia..."
cd "$ROOT/adapters/zksync-solidity"
[ ! -d node_modules ] && npm install --silent
ZKSYNC_ADDRESS=$(npx hardhat deploy-zksync --script scripts/deploy.ts --network zkSyncTestnet 2>&1 | grep "was deployed to" | awk '{print $NF}')
if [ -z "$ZKSYNC_ADDRESS" ]; then
    echo "    ⚠  zkSync deploy output not captured — check above for errors."
else
    echo "    ✅  zkSync RWAOracle: $ZKSYNC_ADDRESS"
fi

# ── 2. Secret Network Testnet ──────────────────────────────────────────────────
echo ""
echo "▶  [2/3] Building Secret Network WASM..."
cd "$ROOT/adapters/secret-network-cosmwasm"
if command -v cargo &> /dev/null; then
    cargo build --release --target wasm32-unknown-unknown 2>&1 | tail -3
    echo "    ✅  WASM built at target/wasm32-unknown-unknown/release/upe_secret_adapter.wasm"
    echo "    ℹ   To deploy, run:"
    echo "        secretcli tx wasm store target/wasm32-unknown-unknown/release/upe_secret_adapter.wasm \\"
    echo "            --from <your-secret-wallet> --chain-id pulsar-3 --gas auto"
    echo "    ℹ   Then instantiate with:"
    echo "        secretcli tx wasm instantiate <CODE_ID> '{\"attester\":\"<YOUR_ADDRESS>\"}' \\"
    echo "            --label 'UPE RWA Oracle' --from <wallet> --chain-id pulsar-3"
    SECRET_ADDRESS="<Deploy and paste address here>"
else
    echo "    ⚠  Rust/cargo not found — skipping Secret Network build."
    SECRET_ADDRESS="<Rust required — see README>"
fi

# ── 3. Mina — instructions (o1js deploy requires Berkeley wallet) ──────────────
echo ""
echo "▶  [3/3] Mina Protocol (o1js) deployment instructions..."
cd "$ROOT/adapters/mina-o1js"
if [ ! -d node_modules ]; then
    npm install --silent 2>&1 | tail -2
fi
echo "    ℹ   To deploy RWAOracle to Mina Berkeley testnet:"
echo "        1. Install Mina zkApp CLI: npm install -g zkapp-cli"
echo "        2. Run: zk config  (configure your Berkeley wallet)"
echo "        3. Run: zk deploy  (in adapters/mina-o1js/)"
echo "    ℹ   Faucet: https://faucet.minaprotocol.com"
MINA_ADDRESS="<Deploy via zk deploy and paste address here>"

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║                 Deployment Summary               ║"
echo "╚══════════════════════════════════════════════════╝"
echo "  Oasis Sapphire (deployed): 0x868ddB7F682818cc392B4484Dd7A8b7629D6f4dA"
echo "  zkSync Era Sepolia:        ${ZKSYNC_ADDRESS:-<pending>}"
echo "  Secret Network Pulsar-3:   ${SECRET_ADDRESS:-<pending>}"
echo "  Mina Berkeley:             ${MINA_ADDRESS:-<pending>}"
echo ""
echo "  👉 Update apps/web/lib/contracts.ts and set env vars:"
echo "     NEXT_PUBLIC_ZKSYNC_ADDRESS=<zksync address>"
echo "     NEXT_PUBLIC_SECRET_ADDRESS=<secret address>"
echo "     NEXT_PUBLIC_MINA_ADDRESS=<mina address>"
echo ""
