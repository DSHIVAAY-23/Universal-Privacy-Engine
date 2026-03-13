# Deliverables — Oasis ROSE Bloom Grant

**Requested:** $20,000 in ROSE  
**Grant:** Oasis ROSE Bloom  
**Status:** Milestone 1 complete

---

## Milestone 1 — Complete ✅

**Scope:** Core pipeline working end-to-end on Sapphire Testnet.

| What | Evidence |
|---|---|
| Rust Notary REST API (Axum + secp256k1 ECDSA) | `core/src/main.rs`, `core/src/notary/mod.rs` |
| `PrivatePayroll.sol` deployed on Sapphire Testnet | `0x55bB3b7871fBf8a5BeB289079aAC9Dc13AA97024` |
| React frontend with live API calls (no mocks) | [universal-privacy-engine.vercel.app](https://universal-privacy-engine-a1kfpf0no-dshivaay23s-projects.vercel.app) |
| CORS + Cloudflare tunnel working from Vercel | `allow_headers(Any)` in `core/src/main.rs` |
| On-chain transaction confirmed | `0x9def61f121055ff791ba8780ce1ba6596c5a7a6cce995bb035adaaecc9eb2211` |
| Open-source repo | [github.com/DSHIVAAY-23/Universal-Privacy-Engine](https://github.com/DSHIVAAY-23/Universal-Privacy-Engine) |

---

## Milestone 2 — TLSNotary Integration ($7,500)

**Scope:** Replace simulated oracle with cryptographically trustless Web2 data proofs.

The goal is that the Notary becomes mathematically unable to sign fabricated data. The user's banking credentials never leave their browser.

| Deliverable |
|---|
| TLSNotary browser extension integration |
| MPC handshake between user browser and Rust Notary |
| Selective disclosure — user redacts credentials, reveals salary field |
| Rust Notary updated to verify TLS transcript before signing |
| End-to-end demo with a real payroll provider (or staging environment) |

**Acceptance criteria:** Demonstrate that a compromised Notary server cannot produce a valid signed proof without a real TLS session from the target server.

---

## Milestone 3 — Mainnet + Developer SDK ($7,500)

**Scope:** Production deployment and tooling for third-party dApps.

| Deliverable |
|---|
| `PrivatePayroll.sol` deployed to Sapphire Mainnet |
| TypeScript SDK — submit STLOP proofs from any dApp |
| Integration docs |
| Mock DeFi demo — undercollateralized loan using UPE-verified salary |
| Security review of Notary signing logic and contract |

---

## Technical validation (Phase 1)

- `ecrecover` on Sapphire correctly recovers the Notary address from EIP-191 signed proofs
- `getMySalary()` returns correctly only to `msg.sender`; all other callers revert
- `eth_getStorageAt` on the contract slot returns ciphertext, not plaintext salary
- `cargo test` and `npm run build` both pass clean from fresh clone

**Gas costs:**

| Tx | Gas |
|---|---|
| `verifyAndStoreSalary()` | ~50,000 |
| `getMySalary()` (view) | ~23,000 |

For reference, zkSNARK verification runs around 500,000 gas. UPE gets confidentiality at 10x lower cost by using Sapphire's TEE instead of ZK circuits on-chain.

---

## Out of scope (this grant)

- Formal security audit
- ROFL decentralized notary
- Multi-chain support (UPE is Sapphire-native by design)

---

*February 2026 | Oasis ROSE Bloom*
