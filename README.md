# Universal Privacy Engine (UPE)

[![CI](https://github.com/DSHIVAAY-23/Universal-Privacy-Engine/actions/workflows/ci.yml/badge.svg)](https://github.com/DSHIVAAY-23/Universal-Privacy-Engine/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Network: Oasis Sapphire](https://img.shields.io/badge/Network-Oasis%20Sapphire%20Testnet-teal.svg)](https://testnet.explorer.sapphire.oasis.io/address/0x2Df7658D5E57ed05D6F634fD7d73b334ADEc179A)
[![Groth16](https://img.shields.io/badge/ZK-Groth16%20%7C%20BN254-purple.svg)](https://eprint.iacr.org/2016/260)
[![Circom](https://img.shields.io/badge/Circuit-Circom%202.1.6-orange.svg)](https://docs.circom.io/)

> **Zero-Knowledge collateral verification for Real World Assets.  
> Prove your balance meets a threshold. Reveal nothing else.**

---

## The Problem

DeFi protocols need to verify that a user's off-chain collateral (tokenized bonds, real estate, T-bills) meets a minimum threshold before granting access to liquidity. Today, users must either:

- **Reveal their full balance** to a centralized verifier (privacy lost), or  
- **Trust an oracle** with raw private financial data (trust assumption added).

**UPE eliminates both tradeoffs.**

---

## How It Works

```
┌─────────────────────────────────────────────────────────────────┐
│                  Universal Privacy Engine — Full Flow            │
└─────────────────────────────────────────────────────────────────┘

  [User]
    │  Private inputs:
    │   ├─ tokenBalance      (hidden — never leaves local machine)
    │   ├─ userAddress       (hidden)
    │   ├─ secretTrapdoor    (hidden — random 256-bit salt)
    │   └─ merklePathElements (hidden — Merkle inclusion proof)
    │
    │  Public inputs:
    │   ├─ stateRoot         (source chain Merkle root)
    │   ├─ minRequiredValue  (DeFi protocol's threshold)
    │   └─ nullifierHash     (replay-prevention token)
    ▼
 ┌──────────────────────┐
 │   rwa_shield.circom  │  — Circom 2.1.6 / Groth16 / BN254
 │                      │
 │  1. Leaf hash        │  Poseidon(userAddress, tokenBalance)
 │  2. Merkle proof     │  20-level tree → root must equal stateRoot
 │  3. Range check      │  tokenBalance >= minRequiredValue (252-bit)
 │  4. Nullifier        │  Poseidon(userAddress, secretTrapdoor)
 └──────────────────────┘
    │
    │  Outputs:
    │   └─ π = (A, B, C)   Groth16 proof (~800 bytes)
    ▼
 ┌──────────────────────────────────┐
 │   RWAOracle.sol                  │  Oasis Sapphire Testnet
 │                                  │  0x2Df7658D5E57ed05D6F634fD7d73b334ADEc179A
 │   verifyProof(A, B, C, signals)  │  — Groth16 on-chain verifier
 │   nullifiers[hash] = true        │  — Double-spend prevention
 │   activeCollateral[msg.sender]++ │  — TEE-encrypted credit
 └──────────────────────────────────┘
    │
    ▼
  ✅  Collateral verified. Balance never revealed.
```

**The cryptographic guarantee:** An adversary who observes every on-chain transaction, every RPC call, and the full contract state _cannot_ learn the user's actual balance. The Groth16 SNARK is zero-knowledge under the discrete log assumption over BN254.

---

## Live Deployment

| Item | Value |
|---|---|
| **Contract (Sapphire Testnet)** | [`0x2Df7658D5E57ed05D6F634fD7d73b334ADEc179A`](https://testnet.explorer.sapphire.oasis.io/address/0x2Df7658D5E57ed05D6F634fD7d73b334ADEc179A) |
| **Verified block** | `#16119759` |
| **Sample TX** | [`0xa47165fb...`](https://testnet.explorer.sapphire.oasis.io/tx/0xa47165fb82f6d34c5da912a2dee2aec97e9be466647d8b03f7afcf8a62c1e475) |
| **Dashboard** | [`localhost:3000`](http://localhost:3000) (run locally) |

---

## Performance Benchmarks

| Metric | Value |
|---|---|
| Groth16 Prover Time | **1.8 s – 6.1 s** (client-side, varies by hardware) |
| On-chain Verification Gas | **267,491 gas** |
| Proof Size | ~800 bytes (Groth16 compressed) |
| Merkle Tree Depth | 20 levels |
| Range Check Bits | 252 (supports full EVM uint256 range) |
| Hash Function | Poseidon (~250 constraints vs. Keccak's ~150k) |

---

## Multi-Chain Deployment Targets

| Chain | Adapter | Status |
|---|---|---|
| **Oasis Sapphire** | `adapters/evm-contracts` | ✅ Live on Testnet |
| **zkSync Era** | `adapters/zksync-solidity` | 🔨 In Progress |
| **Secret Network** | `adapters/secret-network-cosmwasm` | 🔨 In Progress |
| **Aleo** | `adapters/aleo-leo` | 🔬 Research |
| **Mina** | `adapters/mina-o1js` | 🔬 Research |

---

## Repository Structure

```
Universal-Privacy-Engine/
├── apps/
│   └── web/                        # Next.js 14 dashboard — proof generation UI
├── packages/
│   ├── upe-core-circuits/          # Circom circuits (rwa_shield.circom)
│   └── core/                       # Rust logic — witness generation, proof helpers
├── adapters/
│   ├── evm-contracts/              # Hardhat — RWAOracle.sol + Groth16Verifier.sol
│   ├── zksync-solidity/            # zkSync Era adapter
│   ├── secret-network-cosmwasm/    # CosmWasm adapter
│   ├── aleo-leo/                   # Aleo Leo adapter
│   └── mina-o1js/                  # Mina o1js adapter
└── docs/                           # Architecture, ARCHITECTURE.md
```

---

## Quick Start

**Prerequisites:** Node 18+, Rust stable, `circom` CLI.

```bash
# 1. Clone
git clone https://github.com/DSHIVAAY-23/Universal-Privacy-Engine.git
cd Universal-Privacy-Engine

# 2. Start the dashboard
cd apps/web
npm install
npm run dev
# → http://localhost:3000
```

For circuit compilation and proof generation, see [`packages/upe-core-circuits/README.md`](packages/upe-core-circuits/README.md).

---

## Zero-Knowledge — Simple Explanation

A ZK proof lets you convince someone that a statement is true **without revealing why it's true**.

Here, the statement is: _"My token balance is ≥ $100,000."_

You generate a Groth16 proof locally. The on-chain verifier runs a single pairing check on BN254 — it either passes or fails. If it passes, the contract knows your balance exceeds the threshold. At no point does your actual balance appear in any transaction, calldata, or event log.

The Poseidon hash binds your balance to the Merkle state root of the source chain, so you can't lie about it. The nullifier prevents the same proof from being submitted twice.

---

## Roadmap

- [x] **Phase 1** — Groth16 circuit, Sapphire testnet, Next.js dashboard
- [ ] **Phase 2** — zkSync Era + Secret Network adapters, mainnet deploy
- [ ] **Phase 3** — zkTLS integration (trustless Web2 data proofs via TLSNotary)
- [ ] **Phase 4** — SDK (`@upe/client`), DeFi protocol integrations, Aleo + Mina adapters

---

## Contributing

Contributions are welcome. Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) first.

For security-critical circuit issues, use the [Circuit Vulnerability Report](.github/ISSUE_TEMPLATE/circuit_vulnerability.md) template — **do not open a public issue**.

---

## License

MIT — see [`LICENSE`](LICENSE) for details.

---

*Built by [DSHIVAAY-23](https://github.com/DSHIVAAY-23) · Oasis Foundation Grantee Candidate*
