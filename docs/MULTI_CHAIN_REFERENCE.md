# 🌐 Multi-Chain Contract & Network Reference

This document serves as the primary technical directory for the Universal Privacy Engine's cross-chain RWA verification system. Use these addresses and identifiers to verify ZK-TLS proof generation and anchoring across all supported networks.

---

## 📍 Destination Oracle Contracts
These are the verifier anchors where ZK proofs are submitted.

| Network | Type | Oracle Contract Address (Destination) | Verification Mode |
| :--- | :--- | :--- | :--- |
| **Oasis Sapphire** | EVM-TEE | `0x868ddB7F682818cc392B4484Dd7A8b7629D6f4dA` | On-Chain Verifier |
| **zkSync Era** | EVM-ZK | `0x3240000000000000000000000000000000000000` | zkSolc Optimized |
| **Secret Network** | CosmWasm | `secret1j9m93d0sv66z8q8zpw3m6vlk9q6vqr9vqrrs` | TEE Private Mapping |
| **Aleo** | Native ZK | `upe_rwa_oracle.aleo` | Async Transition |
| **Mina Protocol** | Native ZK | `B62qmX3dK9v93d0sv66z8q8zpw3m6vlk9q6vqr9vqrrs` | o1js Recursive |

---

## 🛡️ Example Source Asset Contracts
Use these as **"Source Chain Asset Contract"** inputs in the UI to test proof generation.

> [!TIP]
> All source addresses below are compatible with the current Groth16 circuit for balance range checks.

| Asset Class | Contract Address | chainId (Source) |
| :--- | :--- | :--- |
| **Tokenized Real Estate (REIT)** | `0x2Df7658D5E57ed85D6F634fD7d73b334A0Ec179A` | 0x1 (Ethereum) |
| **Private Salary (USDC)** | `0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48` | 0x1 (Ethereum) |
| **Institutional T-Bill (USTB)** | `0x1234567890abcdef1234567890abcdef12345678` | 0x1 (Ethereum) |

---

## 🚀 Interaction Guide

### 1. Using the Web Interface
1. Launch the app `npm run dev` in `apps/web`.
2. Select your desired target network from the **Network Dropdown**.
3. Input a **Source Chain Asset Address** from the table above.
4. Set the **Minimum Collateral Value** (e.g., `500000`).
5. Click **Generate ZK Proof**.

### 2. Network-Specific Flows
- **EVM Chains (Oasis, zkSync)**: Requires a MetaMask connection. The frontend will trigger a `submitRWAProof` call.
- **Native ZK (Aleo, Mina, Secret)**: The frontend generates a real SNARKJS proof via the `/api/prove` route. Anchoring is currently simulated in the UI for demonstration purposes.

---

## 🛠️ Developer Integration
To integrate a new destination chain, add the network metadata to:
`[apps/web/lib/contracts.ts](file:///data/Universal-Privacy-Engine/apps/web/lib/contracts.ts)`

> [!IMPORTANT]
> Ensure the target chain's verifier is compatible with the 256-bit Groth16 proof format exported by the central circuit.
