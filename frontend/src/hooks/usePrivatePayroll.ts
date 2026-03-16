import { useWriteContract, useWaitForTransactionReceipt, useReadContract } from 'wagmi';
import { CONTRACT_ADDRESSES, RWA_ORACLE_ABI, Network } from '../lib/contracts';
import { useAppStore } from '../store/useAppStore';

/**
 * Custom hook for RWAOracle contract write operations
 * Handles multi-chain ZK proof submission
 */
export function useVerifySalary() {
    const { writeContract, data: hash, error, isPending } = useWriteContract();
    const { isLoading: isConfirming, isSuccess } = useWaitForTransactionReceipt({ hash });
    const { selectedNetwork } = useAppStore();

    const verifySalary = async (proof: any) => {
        try {
            const address = CONTRACT_ADDRESSES[selectedNetwork];
            
            // For EVM chains (Oasis, zkSync)
            if (selectedNetwork === Network.OasisSapphire || selectedNetwork === Network.zkSyncEra) {
                await writeContract({
                    address: address as `0x${string}`,
                    abi: RWA_ORACLE_ABI,
                    functionName: 'submitRWAProof',
                    args: [
                        proof.proof,
                        proof.stateRoot,
                        proof.nullifierHash,
                        proof.minCollateral,
                        proof.assetContract,
                    ],
                });
            } else {
                console.log(`Verification for ${selectedNetwork} handled via custom adapter logic`);
                // TODO: Implement Secret/Aleo/Mina specific submission logic
            }
        } catch (err) {
            console.error('Error verifying salary:', err);
            throw err;
        }
    };

    return {
        verifySalary,
        txHash: hash,
        error: error as Error | null,
        isPending,
        isConfirming,
        isSuccess,
    };
}

/**
 * Custom hook for reading collateral from RWAOracle
 */
export function useGetSalary(address?: `0x${string}`) {
    const { selectedNetwork } = useAppStore();
    const contractAddress = CONTRACT_ADDRESSES[selectedNetwork];

    const { data: salary, error, isLoading, refetch } = useReadContract({
        address: contractAddress as `0x${string}`,
        abi: RWA_ORACLE_ABI,
        functionName: 'getActiveCollateral',
        query: {
            enabled: !!address && (selectedNetwork === Network.OasisSapphire || selectedNetwork === Network.zkSyncEra),
        },
    });

    return {
        salary: salary as bigint | undefined,
        error,
        isLoading,
        refetch,
    };
}
