export enum Network {
    OasisSapphire = 'Oasis Sapphire',
    zkSyncEra = 'zkSync Era',
    SecretNetwork = 'Secret Network (CosmWasm)',
    Aleo = 'Aleo',
    Mina = 'Mina Protocol',
}

export const CONTRACT_ADDRESSES: Record<string, string> = {
    [Network.OasisSapphire]: process.env.NEXT_PUBLIC_OASIS_ADDRESS ?? "0x868ddB7F682818cc392B4484Dd7A8b7629D6f4dA",
    [Network.zkSyncEra]: process.env.NEXT_PUBLIC_ZKSYNC_ADDRESS ?? "0x0000000000000000000000000000000000000000", // Will be updated after deployment
    [Network.SecretNetwork]: process.env.NEXT_PUBLIC_SECRET_ADDRESS ?? "secret1...",
    [Network.Aleo]: "upe_rwa_oracle.aleo",
    [Network.Mina]: process.env.NEXT_PUBLIC_MINA_ADDRESS ?? "B62...",
};

export const CONTRACT_ABI = [
    "function submitRWAProof(uint256[8] calldata proof, uint256 stateRoot, uint256 nullifierHash, uint256 minCollateral, address assetContract) external",
    "event RWAProofSubmitted(address indexed account, address indexed assetContract, uint256 minCollateral)",
];
