# 🚀 UPE Multi-Chain RWA — Testnet Launch Checklist

Follow these steps to run the complete multi-chain ZK Proof pipeline.

---

## ✅ Phase 1 — Oasis Sapphire (LIVE)

Already deployed and verified. See main [TESTNET_LAUNCH_CHECKLIST.md](./TESTNET_LAUNCH_CHECKLIST_LEGACY.md).

**RWAOracle:** `0x868ddB7F682818cc392B4484Dd7A8b7629D6f4dA` (Sapphire Testnet, Chain ID: 23295)

---

## Phase 2 — Multi-Chain RWA Deploy (`feat/multi-chain-rwa`)

### Step 1: Start the ZK Prover API
```bash
cd packages/upe-core-circuits
node scripts/generate_proof.js 500000 0x2Df7658D5E57ed85D6F634fD7d73b334A0Ec179A
```
*Verify you see a Groth16 proof JSON output before proceeding.*

---

### Step 2: Deploy — zkSync Era Sepolia
```bash
# Prerequisites: Fund wallet at https://portal.zksync.io/drpc (Sepolia ETH → zkSync)
cd adapters/zksync-solidity
cp .env.example .env
# Edit .env: set DEPLOYER_PRIVATE_KEY=0x<your-key>
npm install
npm run deploy:testnet
# Copy the deployed address → set NEXT_PUBLIC_ZKSYNC_ADDRESS in apps/web/.env.local
```
Testnet faucet: [https://faucet.quicknode.com/zksync/sepolia](https://faucet.quicknode.com/zksync/sepolia)

---

### Step 3: Deploy — Secret Network Pulsar-3
```bash
# Prerequisites: Install secretcli + fund wallet at https://faucet.secretsaturn.net
cd adapters/secret-network-cosmwasm
cargo build --release --target wasm32-unknown-unknown

# Deploy WASM binary
secretcli tx wasm store \
  target/wasm32-unknown-unknown/release/upe_secret_adapter.wasm \
  --from <wallet> --chain-id pulsar-3 --gas auto

# Instantiate (replace CODE_ID from output above)
secretcli tx wasm instantiate <CODE_ID> \
  '{"attester":"<YOUR_SECRET_ADDRESS>"}' \
  --label "UPE RWA Oracle" --from <wallet> --chain-id pulsar-3

# Copy contract address → set NEXT_PUBLIC_SECRET_ADDRESS in apps/web/.env.local
```

---

### Step 4: Deploy — Mina Protocol (Berkeley Testnet)
```bash
# Prerequisites: Install zkApp CLI
npm install -g zkapp-cli
cd adapters/mina-o1js
npm install && npm run build

# Configure wallet & deploy
zk config    # Follow prompts for Berkeley testnet
zk deploy

# Copy deployed address → set NEXT_PUBLIC_MINA_ADDRESS in apps/web/.env.local
```
Mina faucet: [https://faucet.minaprotocol.com](https://faucet.minaprotocol.com)

---

### Step 5: Configure Environment & Start Frontend
```bash
cd apps/web
cp .env.local.example .env.local  # if not already done
# Edit .env.local with deployed addresses from above steps

npm run dev
```
Open `http://localhost:3000` — select any network from the dropdown.

---

### Step 6: Test Full Pipeline in Browser
1. Go to `http://localhost:3000`
2. Select a network (e.g., **zkSync Era**)
3. Input a source asset contract (e.g., `0x2Df7658D5E57ed85D6F634fD7d73b334A0Ec179A`)
4. Set a collateral value (e.g., `500000`)
5. Click **Generate ZK Proof**
6. Observe the Prover Console — watch for `[SUCCESS]` with tx hash

---

## 🔗 Deployed Addresses Summary

| Network | Status | Contract Address |
|---|---|---|
| Oasis Sapphire Testnet | ✅ Live | `0x868ddB7F682818cc392B4484Dd7A8b7629D6f4dA` |
| zkSync Era Sepolia | ⏳ Run Step 2 | `NEXT_PUBLIC_ZKSYNC_ADDRESS` |
| Secret Network Pulsar-3 | ⏳ Run Step 3 | `NEXT_PUBLIC_SECRET_ADDRESS` |
| Mina Berkeley | ⏳ Run Step 4 | `NEXT_PUBLIC_MINA_ADDRESS` |
| Aleo Testnet | ⏳ `leo deploy` | `upe_rwa_oracle.aleo` |

---

## One-Command Deploy (EVM + Build)

```bash
./deploy-all.sh 0x<your-private-key>
```

This will deploy zkSync automatically and print instructions for Secret/Mina.
