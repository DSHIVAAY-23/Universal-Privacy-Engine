import { parseAbi } from 'viem';

/**
 * PrivatePayroll Contract Configuration
 * 
 * NOTE: Update this address after deploying to Sapphire Testnet
 */
export const PRIVATE_PAYROLL_ADDRESS = import.meta.env.VITE_PRIVATE_PAYROLL_ADDRESS as `0x${string}` || '0x0000000000000000000000000000000000000000';

// Safety check: Warn if contract address is not configured
if (!import.meta.env.VITE_PRIVATE_PAYROLL_ADDRESS || PRIVATE_PAYROLL_ADDRESS === '0x0000000000000000000000000000000000000000') {
    console.warn(
        '⚠️  VITE_PRIVATE_PAYROLL_ADDRESS is not configured or is set to placeholder.\n' +
        'Please deploy PrivatePayroll.sol to Sapphire Testnet and update your .env file:\n' +
        'VITE_PRIVATE_PAYROLL_ADDRESS=0xYourDeployedContractAddress'
    );
}

/**
 * PrivatePayroll Contract ABI
 * Parsed from Solidity contract in contracts/oasis/src/PrivatePayroll.sol
 */
export const PRIVATE_PAYROLL_ABI = parseAbi([
    'function verifyAndStoreSalary(uint256 salary, uint256 timestamp, bytes signature) external',
    'function getMySalary() external view returns (uint256)',
    'function TRUSTED_NOTARY() external view returns (address)',
    'event SalaryVerified(address indexed employee, uint256 timestamp)'
]);

/**
 * Full ABI for reference (includes constructor and internal functions)
 */
export const PRIVATE_PAYROLL_ABI_FULL = [
    {
        "inputs": [
            { "internalType": "address", "name": "_trustedNotary", "type": "address" }
        ],
        "stateMutability": "nonpayable",
        "type": "constructor"
    },
    {
        "anonymous": false,
        "inputs": [
            { "indexed": true, "internalType": "address", "name": "employee", "type": "address" },
            { "indexed": false, "internalType": "uint256", "name": "timestamp", "type": "uint256" }
        ],
        "name": "SalaryVerified",
        "type": "event"
    },
    {
        "inputs": [],
        "name": "TRUSTED_NOTARY",
        "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "getMySalary",
        "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            { "internalType": "uint256", "name": "salary", "type": "uint256" },
            { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
            { "internalType": "bytes", "name": "signature", "type": "bytes" }
        ],
        "name": "verifyAndStoreSalary",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    }
] as const;
