# Oasis Grant Notes — UPE

## How to talk about this project

- **What it is:** A privacy oracle for Oasis Sapphire. Proves facts about off-chain data without storing the data publicly on-chain.
- **Core claim:** On Sapphire, `private` mappings are actually private. TEE encryption makes it true at the protocol level. UPE exploits that.
- **The proof is public. The salary is not.** That's the whole pitch.
- **10x cheaper than zkSNARKs.** ~50k gas vs ~500k gas. Sapphire's TEE does the confidentiality work so you don't need ZK circuits on-chain.
- **Phase 1 is correct engineering, not a shortcut.** You can't build a trustless Web2 oracle without client-side zkTLS. Simulating the data now and upgrading the oracle in Phase 2 is the right sequence.

---

## Why Sapphire (anticipated questions)

**"Why not Ethereum + ZK proofs?"**  
zkSNARKs require custom circuits per data type, slow proving, and expensive on-chain verification (~500k gas). Sapphire handles confidentiality at the protocol level. No custom crypto needed. Standard Solidity contracts get encrypted storage for free.

**"Why not a private chain / Hyperledger?"**  
Private chains give up decentralization. Sapphire is a public parachain with encrypted state — you get both.

**"How is the data actually private?"**  
The Sapphire ParaTime runs inside a TEE. All storage reads/writes go through TEE-controlled encryption keys. Even validators can't call `eth_getStorageAt` and get plaintext. Try it — you get ciphertext.

---

## Why the data is simulated in Phase 1

Fetching real bank data from a server means users share their credentials with that server. That's not a privacy engine. The correct approach — client-side zkTLS via TLSNotary — requires the user's browser to open the TLS session, generate a local proof of the transcript, and send only the proof to the Notary. Building that is the focus of Phase 2 and the main reason this grant exists.

Phase 1 gets the on-chain half right. Phase 2 makes the oracle half trustless. The contract doesn't change between phases — it just keeps verifying the same ECDSA signature.

---

## Reviewer FAQ

**Is this production-ready?**  
No. It's a working testnet prototype. Production needs Phase 2 (trustless oracle), a security audit, and mainnet deployment — that's Milestones 2 and 3.

**Can this work on other chains?**  
No. Confidential storage is unique to Sapphire. Deploying the same contract on Ethereum exposes all the data via `eth_getStorageAt`.

**What's the long-term vision?**  
UPE becomes a general privacy oracle layer on Sapphire. Any dApp that needs to verify private off-chain data — credit scores, payroll, KYC documents — can use UPE without building the proof infrastructure themselves.

---

## Links

| | |
|---|---|
| Frontend | [universal-privacy-engine.vercel.app](https://universal-privacy-engine-a1kfpf0no-dshivaay23s-projects.vercel.app) |
| GitHub | [github.com/DSHIVAAY-23/Universal-Privacy-Engine](https://github.com/DSHIVAAY-23/Universal-Privacy-Engine) |
| Contract | `0x55bB3b7871fBf8a5BeB289079aAC9Dc13AA97024` (Sapphire Testnet) |
| Sapphire Explorer | [testnet.explorer.sapphire.oasis.io](https://testnet.explorer.sapphire.oasis.io) |
| Sapphire Docs | [docs.oasis.io/dapp/sapphire](https://docs.oasis.io/dapp/sapphire/) |
| TLSNotary | [tlsnotary.org](https://tlsnotary.org) |
| ROFL Docs | [docs.oasis.io/dapp/rofl](https://docs.oasis.io/dapp/rofl/) |
| ROSE Bloom Grants | [oasisprotocol.org/grants](https://oasisprotocol.org/grants) |

---

*February 2026 | Oasis ROSE Bloom | Milestone 1 complete*
