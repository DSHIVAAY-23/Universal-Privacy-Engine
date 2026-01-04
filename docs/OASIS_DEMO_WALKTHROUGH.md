# Oasis Sapphire End-to-End Demo Flow

> **⚠️ DEPLOYMENT STATUS: Local Network Testing**  
> This demo currently runs on **local Hardhat network** for development and testing.  
> **Sapphire Testnet deployment** is planned for the next phase of the grant.  
> All contract logic and cryptographic verification work correctly on local network.

## Architecture

The **Universal Privacy Engine (UPE)** enables private data settlement on Oasis Sapphire through the following flow:

1.  **Notary (Off-Chain)**: Captures web2 data (e.g., payroll API), hashes it, and signs `(User, Data, Timestamp)` using an EIP-191 compatible wallet. This creates a **STLOP Proof**.
2.  **Sapphire Contract (On-Chain)**: The `PrivatePayroll` contract receives the proof. It:
    *   Verifies the Notary's signature.
    *   Validates the timestamp.
    *   Stores the data in **Sapphire Encrypted State**.
3.  **User View (Decrypted)**: The user interacts with the contract to retrieve their own data. Because Sapphire runs in a TEE, only the designated user can decrypt their own state.

```mermaid
graph LR
    N[Notary (Off-Chain)] -- Signed Proof --> C[PrivatePayroll.sol (Sapphire)]
    C -- Encrypted State --> S[(Sapphire Storage)]
    S -- Decrypted View --> U[User (Employee)]
```

## How to Run

1.  **Install Dependencies**:
    ```bash
    cd contracts/oasis
    npm install
    ```

2.  **Run the Demo Script**:
    ```bash
    npx hardhat run scripts/demo_sapphire_flow.js
    ```
    *(Note: For Sapphire Testnet, you would add `--network sapphire_testnet`)*

## Key Concept: Input Visibility vs. Storage Privacy

It is important to understand the difference between **transaction inputs** and **storage state**:

*   **Transaction Inputs**: When sending the proof, the `salary` value is visible in the transaction data (mempool/block).
    *   *Mitigation*: In a full production deployment, users would use the **Sapphire Wrapper** (Oasis SDK) to encrypt the transaction inputs *before* they reach the network, ensuring end-to-end privacy.
*   **Storage State**: Once stored in the `mapping(address => uint256) private salaries`, the data is **fully encrypted** by the Sapphire ParaTime key. It cannot be read by node operators or via `getStorageAt`.

This demo focuses on demonstrating the **Encrypted Storage** and **Access Control** capabilities.
