# Universal Privacy Engine (UPE)

[![CI](https://github.com/DSHIVAAY-23/Universal-Privacy-Engine/actions/workflows/ci.yml/badge.svg)](https://github.com/DSHIVAAY-23/Universal-Privacy-Engine/actions/workflows/ci.yml)

UPE is a privacy oracle. Users prove facts about their off-chain data — salary, credit score, asset value — to smart contracts on **Oasis Sapphire**, without the raw data ever touching the chain.

---

## Live Demo

**[universal-privacy-engine.vercel.app](https://universal-privacy-engine-a1kfpf0no-dshivaay23s-projects.vercel.app)**

### First 60 seconds — what a reviewer should do

1. Open the link above
2. Connect MetaMask → switch to **Oasis Sapphire Testnet** (Chain ID: `23295`, RPC: `https://testnet.sapphire.oasis.io`)
3. Click **Start Verification** — the frontend calls the Rust Notary and receives a signed STLOP proof
4. Review the proof in the UI (salary, timestamp, ECDSA signature)
5. Click **Submit** → approve the MetaMask transaction
6. Wait for **Transaction Confirmed** — the `SalaryVerified` event is emitted on-chain
7. Click **View My Salary** — `getMySalary()` returns your salary. Any other address gets a revert.

**Sample verified transaction:** [`0x9def61f121055ff791ba8780ce1ba6596c5a7a6cce995bb035adaaecc9eb2211`](https://testnet.explorer.sapphire.oasis.io/tx/0x9def61f121055ff791ba8780ce1ba6596c5a7a6cce995bb035adaaecc9eb2211)

### Trust model

This contract relies on **Oasis Sapphire ParaTime** to keep state private. Deploy only to Sapphire — on any standard EVM chain (Ethereum, Polygon, Arbitrum, etc.) the `salaries` mapping is readable by anyone via `eth_getStorageAt`. The on-chain ECDSA verification is sound on any EVM; the storage confidentiality guarantee is not.

---

## How it works

```
Browser Wallet
    │
    ▼
React Frontend (Vercel)
    │  POST /api/generate-proof
    ▼
Rust Notary  ──── signs salary with secp256k1 ECDSA
    │  STLOP proof: { salary, timestamp, signature }
    ▼
PrivatePayroll.sol (Sapphire Testnet)
    │  ecrecover → verify → write to TEE-encrypted state
    ▼
getMySalary()  ──── only msg.sender can read it
```

The proof is public. The stored salary is private. That's the whole point.

---

## Stack

| Component | Tech | Status |
|---|---|---|
| Notary API | Rust, Axum, secp256k1 | Live |
| Smart Contract | Solidity, Oasis Sapphire | Deployed |
| Frontend | React, Wagmi, RainbowKit, Vite | Live on Vercel |

**Contract**: `0x55bB3b7871fBf8a5BeB289079aAC9Dc13AA97024` (Sapphire Testnet, Chain ID: 23295)  
**Notary**: `0xFCAd0B19bB29D4674531d6f115237E16AfCE377c`

---

## Repo layout

```
core/               # Rust Notary — Axum server, ECDSA signing
contracts/oasis/    # PrivatePayroll.sol
frontend/           # React app — Wagmi hooks, Notary API client
docs/               # Architecture, deliverables, grant notes
```

---

## Run locally

**You need:** Rust stable, Node 18+, MetaMask on Sapphire Testnet.

```bash
# 1. Start the Notary
cd core && cp .env.example .env
# Set NOTARY_PRIVATE_KEY in .env
PORT=3002 cargo run --release

# 2. Expose it (Cloudflare tunnel, free, no account needed)
cloudflared tunnel --url http://localhost:3002
# Copy the https://XXXX.trycloudflare.com URL

# 3. Start the frontend
cd frontend && cp .env.example .env
# Set VITE_NOTARY_API_URL=https://XXXX.trycloudflare.com
npm install && npm run dev
```

---

## API

```bash
GET  /api/health           → { "status": "ok", "notary_address": "0x..." }

POST /api/generate-proof   body: { "employee_address": "0x..." }
                           → { "salary": "75000", "timestamp": 1771315112,
                               "signature": "0x...", "notary_pubkey": "0x..." }
```

---

## Why the payroll data is simulated in Phase 1

Fetching real bank/payroll data from a backend server means users hand over their credentials to that server. That's not a privacy engine — that's just a normal scraper with branding.

The correct approach is client-side **zkTLS** via [TLSNotary](https://tlsnotary.org/): the user's browser opens the TLS session with their bank directly, generates a local cryptographic proof of the transcript, and the Notary verifies the proof without seeing any plaintext. Building that correctly takes serious engineering time — it's Phase 2.

Phase 1 gets the on-chain pipeline right: ECDSA proof → Sapphire encrypted state. Phase 2 makes the oracle trustless.

---

## Testing

```bash
cd core && cargo test                    # ECDSA signing + API logic
cd contracts/oasis && forge test -vvv    # signature verification + access control
```

---

## Grant Milestones — $20,000 in ROSE

| # | Scope | Timeline | Budget |
|---|---|---|---|
| 1 | Notary API + Sapphire testnet + Vercel frontend | Month 1 | $5,000 |
| 2 | TLSNotary integration — trustless Web2 data proofs | Month 2 | $7,500 |
| 3 | Sapphire mainnet + developer SDK + DeFi integration demo | Month 3 | $7,500 |

---

## Why Sapphire

On any standard EVM, `private` mappings are readable via `eth_getStorageAt`. Sapphire is the only production Confidential EVM — TEE-based encryption makes contract storage actually private. UPE uses that property to store salary data that only the employee's wallet can decrypt.

---

## Builder

**Shivaay Labs** — solo founder, 3+ years building on Rust + Solidity + ZK/TEE stacks.  
GitHub: [github.com/DSHIVAAY-23](https://github.com/DSHIVAAY-23)

---

## License

MIT OR Apache-2.0
