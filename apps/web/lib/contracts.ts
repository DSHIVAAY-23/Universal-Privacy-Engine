export enum Network {
    OasisSapphire = 'Oasis Sapphire',
    zkSyncEra = 'zkSync Era',
    SecretNetwork = 'Secret Network (CosmWasm)',
    Aleo = 'Aleo',
    Mina = 'Mina Protocol',
}

// ── Deployed Contract Addresses ──────────────────────────────────────────────
// Oasis Sapphire: LIVE on testnet (Chain ID: 23295)
// All others:     Set via env vars after running deploy-all.sh
//                 or manually running per-chain deploy scripts.
export const CONTRACT_ADDRESSES: Record<string, string> = {
    [Network.OasisSapphire]:
        process.env.NEXT_PUBLIC_OASIS_ADDRESS ??
        '0x868ddB7F682818cc392B4484Dd7A8b7629D6f4dA', // ✅ Deployed — Sapphire Testnet

    [Network.zkSyncEra]:
        process.env.NEXT_PUBLIC_ZKSYNC_ADDRESS ??
        '0x0000000000000000000000000000000000000000', // ⏳ Run: cd adapters/zksync-solidity && npm run deploy:testnet

    [Network.SecretNetwork]:
        process.env.NEXT_PUBLIC_SECRET_ADDRESS ??
        'secret1placeholder000000000000000000000000', // ⏳ Run: deploy-all.sh (step 2)

    [Network.Aleo]:
        'upe_rwa_oracle.aleo', // 📜 Leo program — deploy via: leo deploy

    [Network.Mina]:
        process.env.NEXT_PUBLIC_MINA_ADDRESS ??
        'B62placeholder000000000000000000000000000', // ⏳ Run: zk deploy in adapters/mina-o1js/
};

export const CONTRACT_ABI = [
    'function submitRWAProof(uint256[8] calldata proof, uint256 stateRoot, uint256 nullifierHash, uint256 minCollateral, address assetContract) external',
    'event RWAProofSubmitted(address indexed account, address indexed assetContract, uint256 minCollateral)',
];
