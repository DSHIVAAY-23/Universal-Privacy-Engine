# Universal Privacy Engine (UPE)

> **Bring private Web2 data on-chain — without exposing it.**

UPE is a privacy-preserving oracle that lets users prove facts about their off-chain data (salary, credit score, assets) to smart contracts on **Oasis Sapphire**, without ever revealing the raw data on-chain. Built for the [Oasis ROSE Bloom Grant](https://oasisprotocol.org/).

---

## 🔴 Live Demo

**[https://universal-privacy-engine.vercel.app](https://universal-privacy-engine-a1kfpf0no-dshivaay23s-projects.vercel.app)**

Connect MetaMask on Oasis Sapphire Testnet → click **Start Verification** → your salary is cryptographically proven and stored encrypted on-chain.

---

## How It Works

```
User Wallet
    │
    ▼
React Frontend (Vercel)
    │  POST /api/generate-proof
    ▼
Rust Notary Service  ──── fetches payroll data
    │  ECDSA-signed STLOP proof
    ▼
PrivatePayroll.sol (Oasis Sapphire Testnet)
    │  verifies signature on-chain
    ▼
Encrypted Contract State (TEE)
    │  only the employee's wallet can decrypt
    ▼
User sees their salary ✅
```

**Key properties:**
- The proof is **public and auditable** — anyone can verify the notary signed it
- The salary data is **private** — Sapphire's TEE encrypts it; no one else can read it
- The frontend has **zero mock data** — every proof comes from the live Rust API

---

## Architecture

| Component | Technology | Status |
|---|---|---|
| **Notary Service** | Rust, Axum, ECDSA (secp256k1) | ✅ Live |
| **Smart Contract** | Solidity, Oasis Sapphire | ✅ Deployed |
| **Frontend** | React, Wagmi, RainbowKit, Vite | ✅ Live on Vercel |

### Contract
- **Network**: Oasis Sapphire Testnet (Chain ID: 23295)
- **Address**: `0x55bB3b7871fBf8a5BeB289079aAC9Dc13AA97024`
- **Notary Address**: `0xFCAd0B19bB29D4674531d6f115237E16AfCE377c`

---

## Repository Structure

```
Universal-Privacy-Engine/
├── core/               # Rust Notary REST API
│   ├── src/
│   │   ├── main.rs     # Axum server (CORS, /api/health, /api/generate-proof)
│   │   └── notary/     # ECDSA signing (EIP-191 compatible)
│   └── Cargo.toml
├── contracts/
│   └── oasis/          # Solidity contracts for Sapphire
│       └── src/
│           └── PrivatePayroll.sol
├── frontend/           # React application
│   └── src/
│       ├── lib/notary.ts       # Notary API client
│       ├── hooks/              # Wagmi contract hooks
│       └── components/         # UI components
└── docs/               # Architecture & grant documentation
```

---

## Run Locally

### Prerequisites
- Rust (stable) + Cargo
- Node.js 18+
- MetaMask with [Oasis Sapphire Testnet](https://docs.oasis.io/dapp/sapphire/network) configured

### 1. Start the Rust Notary

```bash
cd core
cp .env.example .env
# Edit .env — set NOTARY_PRIVATE_KEY to any secp256k1 private key

PORT=3002 cargo run --release
# → 🚀 Server listening on http://0.0.0.0:3002
```

### 2. Expose via Cloudflare Tunnel (for frontend access)

```bash
# Install once:
sudo apt install cloudflared

# Start tunnel:
cloudflared tunnel --url http://localhost:3002
# → Copy the https://XXXX.trycloudflare.com URL
```

### 3. Start the Frontend

```bash
cd frontend
cp .env.example .env
# Set VITE_NOTARY_API_URL=https://XXXX.trycloudflare.com

npm install
npm run dev
# → http://localhost:5173
```

---

## API Reference

### `GET /api/health`
```json
{ "status": "ok", "notary_address": "0xfcad..." }
```

### `POST /api/generate-proof`
```json
// Request
{ "employee_address": "0x..." }

// Response — STLOP Proof
{
  "salary": "75000",
  "timestamp": 1771315112,
  "signature": "0x...",
  "notary_pubkey": "0xfcad..."
}
```

---

## Grant Context

This project is built for the **Oasis ROSE Bloom Grant**, demonstrating:

1. **Confidential EVM** — Sapphire's TEE encrypts contract state by default
2. **Privacy-preserving oracles** — Web2 data verified without exposure
3. **STLOP (Signed TLS-Originated Proofs)** — cryptographic bridge between Web2 and Web3

**Roadmap:**
- Phase 2: Replace mock data with real [TLSNotary](https://tlsnotary.org/) proofs
- Phase 3: ROFL (decentralized notary network on Oasis)
- Phase 4: Multi-provider support (credit score, assets, identity)

---

## � Architecture Rationale: The Strategic Delay of zkTLS

For Phase 1 (MVP), I strictly focused on perfecting the **On-Chain Confidentiality Pipeline**: Rust Notary → secp256k1 ECDSA → Oasis Sapphire TEE. The off-chain payroll data is currently simulated.

**Why deliberately simulate the oracle data?**

Fetching real Web2 banking/payroll data requires a trustless **zkTLS integration**. If I built a standard centralized backend scraper today, users would have to share their banking credentials with my Rust server — which **completely defeats the purpose of a Privacy Engine.**

A truly privacy-preserving oracle requires a client-side zkTLS prover, where:
1. The user's browser initiates and completes the TLS session with their bank
2. A cryptographic proof of that TLS transcript is generated *locally*
3. The Rust Notary **mathematically verifies** the proof without ever touching the user's password or raw data

This is a significant cryptography and engineering undertaking (see [TLSNotary](https://tlsnotary.org/)). **I refuse to compromise user security for a quick demo.**

Building this trustless zkTLS infrastructure is the exact focus of **Phase 2**, and the primary justification for this grant request. The Phase 1 architecture is not a shortcut — it is the correct foundation that Phase 2 will build upon directly.

---

## �🧪 Testing & Reproducibility

Security and reliability are top priorities. Run the full test suite locally in under 2 minutes:

```bash
# 1. Rust Notary — ECDSA signing & API logic
cd core
cargo test

# 2. Smart Contract — Sapphire integration & signature verification
cd contracts/oasis
forge test -vvv
```

---

## 💰 Grant Request & Milestones

**Requested Amount:** $20,000 (paid in ROSE)

| Milestone | Timeline | Amount | Deliverable |
|---|---|---|---|
| **1 — Core Notary Hardening & Testnet Polish** | Month 1 | $5,000 | Finalize Rust Notary REST API + ECDSA signing, optimize `PrivatePayroll.sol`, open-source repo with full test coverage and stable Vercel/Cloudflare frontend |
| **2 — TLSNotary Integration & Security** | Month 2 | $7,500 | Replace REST data fetching with cryptographically secure TLSNotary proofs; trustless proof-of-concept where the Notary cannot spoof Web2 data |
| **3 — Mainnet Launch & Developer SDK** | Month 3 | $7,500 | Deploy to Oasis Sapphire Mainnet, release integration SDK/docs, at least one mock DeFi integration (e.g. undercollateralized lending based on UPE data) |

---

## 🤝 Value to the Oasis Ecosystem

UPE acts as critical infrastructure for Oasis Sapphire, enabling a new wave of privacy-first DeFi applications — undercollateralized lending based on private credit scores, payroll-backed loans, and verified asset ownership — all without ever exposing the underlying data on-chain.

This directly **drives developer adoption** and brings Web2 liquidity into the Oasis network, demonstrating the unique power of Sapphire's Confidential EVM and TEEs in a way standard EVMs simply cannot replicate.

---

## 👨‍💻 About the Builder

**Shivaay Labs** — UPE is developed by a solo founder with 3+ years of Web3 engineering experience, specializing in Rust, Solidity, and ZK/TEE architectures.

- **Focus:** Bridging Web2 data privacy with Web3 verifiable compute
- **Commitment:** High-intensity execution, transitioning to full-time solo founder upon grant approval
- **GitHub:** [github.com/DSHIVAAY-23](https://github.com/DSHIVAAY-23)

---

## License

MIT OR Apache-2.0
