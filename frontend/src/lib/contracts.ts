import { parseAbi } from 'viem';

export enum Network {
    OasisSapphire = 'oasis-sapphire',
    zkSyncEra = 'zksync-era',
    SecretNetwork = 'secret-network',
    Aleo = 'aleo',
    Mina = 'mina',
}

export const CONTRACT_ADDRESSES: Record<Network, `0x${string}` | string> = {
    [Network.OasisSapphire]: import.meta.env.VITE_OASIS_ORACLE_ADDRESS || '0x0000000000000000000000000000000000000000',
    [Network.zkSyncEra]: import.meta.env.VITE_ZKSYNC_ORACLE_ADDRESS || '0x0000000000000000000000000000000000000000',
    [Network.SecretNetwork]: import.meta.env.VITE_SECRET_ORACLE_ADDRESS || 'secret1placeholder',
    [Network.Aleo]: 'rwa_oracle.aleo',
    [Network.Mina]: 'B62placeholderminaaddress',
};

export const RWA_ORACLE_ABI = parseAbi([
    'function submitRWAProof(uint256[8] calldata proof, uint256 stateRoot, uint256 nullifierHash, uint256 minCollateral, address assetContract) external',
    'function getActiveCollateral() external view returns (uint256)',
    'event RWAProofSubmitted(address indexed account, address indexed assetContract, uint256 minCollateral)'
]);

/**
 * PrivatePayroll Contract Configuration (Legacy / Oasis Specific)
 */
export const PRIVATE_PAYROLL_ADDRESS = import.meta.env.VITE_PRIVATE_PAYROLL_ADDRESS as `0x${string}` || '0x0000000000000000000000000000000000000000';

export const PRIVATE_PAYROLL_ABI = parseAbi([
    'function verifyAndStoreSalary(uint256 salary, uint256 timestamp, bytes signature) external',
    'function getMySalary() external view returns (uint256)',
    'function TRUSTED_NOTARY() external view returns (address)',
    'event SalaryVerified(address indexed employee, uint256 timestamp)'
]);
