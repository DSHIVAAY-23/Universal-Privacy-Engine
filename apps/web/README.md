# UPE Dashboard — `apps/web`

The Next.js 14 frontend for the Universal Privacy Engine. Users connect a wallet, initiate a ZK proof generation session, and submit the resulting Groth16 proof to the on-chain verifier — all from the browser.

---

## Screenshot

> **Run locally** (`npm run dev`) to see the live dashboard at `http://localhost:3000`.

---

## Features

- **Wallet connect** — MetaMask / WalletConnect via RainbowKit
- **Prover console** — real-time proof generation log with progress stages
- **One-click submit** — submits `(A, B, C, publicSignals)` to `RWAOracle.submitRWAProof`
- **Collateral display** — reads encrypted `activeCollateral` from Sapphire TEE
- **Multi-network switcher** — Oasis Sapphire Testnet pre-configured

---

## Environment Variables

Create `apps/web/.env.local`:

```env
# Oasis Sapphire RPC (testnet)
NEXT_PUBLIC_SAPPHIRE_RPC=https://testnet.sapphire.oasis.io

# Deployed RWAOracle contract address
NEXT_PUBLIC_CONTRACT_ADDRESS=0x2Df7658D5E57ed05D6F634fD7d73b334ADEc179A

# WalletConnect project ID (https://cloud.walletconnect.com)
NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID=your_project_id_here
```

---

## Running Locally

```bash
cd apps/web
npm install
npm run dev
# → http://localhost:3000
```

**Requirements:** Node 18+, MetaMask configured for Oasis Sapphire Testnet (Chain ID: `23295`, RPC: `https://testnet.sapphire.oasis.io`).

---

## How the Prover Console Works

1. User enters the **minimum collateral threshold** they want to prove.
2. The frontend fetches the **source chain Merkle root** (stateRoot) from the oracle feed.
3. A **web worker** calls SnarkJS's `groth16.prove()` with the generated witness — this takes **1.8 s – 6.1 s** depending on hardware.
4. The prover console streams log lines: `Generating witness…` → `Running Groth16…` → `Proof ready`.
5. The resulting `(proof, publicSignals)` are formatted and passed to the contract call.

---

## API Routes

### `POST /api/prove`

Generates a Groth16 witness + proof server-side (fallback for low-end devices).

**Request:**
```json
{
  "tokenBalance": "150000",
  "userAddress": "0xabc...",
  "secretTrapdoor": "0xdeadbeef...",
  "merklePathElements": ["0x...", "..."],
  "merklePathIndices": [0, 1, 0, ...],
  "stateRoot": "0x...",
  "minRequiredValue": "100000"
}
```

**Response:**
```json
{
  "proof": {
    "pi_a": ["...", "..."],
    "pi_b": [["...", "..."], ["...", "..."]],
    "pi_c": ["...", "..."]
  },
  "publicSignals": ["stateRoot", "minRequiredValue", "nullifierHash"]
}
```

---

## Tech Stack

| Layer | Library |
|---|---|
| Framework | Next.js 14 (App Router) |
| Wallet | RainbowKit + wagmi v2 |
| ZK (client) | SnarkJS 0.7 |
| Styling | Tailwind CSS |
| Contract ABI | ethers.js v6 |
