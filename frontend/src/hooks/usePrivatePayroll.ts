import { useWriteContract, useWaitForTransactionReceipt, useReadContract } from 'wagmi';
import { PRIVATE_PAYROLL_ADDRESS, PRIVATE_PAYROLL_ABI } from '../lib/contracts';
import type { STLOPProof } from '../types';

/**
 * Custom hook for PrivatePayroll contract write operations
 * Handles salary verification submission
 */
export function useVerifySalary() {
    const { writeContract, data: hash, error, isPending } = useWriteContract();
    const { isLoading: isConfirming, isSuccess } = useWaitForTransactionReceipt({ hash });

    const verifySalary = async (proof: STLOPProof) => {
        try {
            await writeContract({
                address: PRIVATE_PAYROLL_ADDRESS,
                abi: PRIVATE_PAYROLL_ABI,
                functionName: 'verifyAndStoreSalary',
                args: [
                    BigInt(proof.salary),
                    BigInt(proof.timestamp),
                    proof.signature as `0x${string}`,
                ],
            });
        } catch (err) {
            console.error('Error verifying salary:', err);
            throw err;
        }
    };

    return {
        verifySalary,
        txHash: hash,
        error,
        isPending,
        isConfirming,
        isSuccess,
    };
}

/**
 * Custom hook for reading encrypted salary from contract
 * Only works for the connected wallet address
 */
export function useGetSalary(address?: `0x${string}`) {
    const { data: salary, error, isLoading, refetch } = useReadContract({
        address: PRIVATE_PAYROLL_ADDRESS,
        abi: PRIVATE_PAYROLL_ABI,
        functionName: 'getMySalary',
        query: {
            enabled: !!address, // Only query if address is provided
        },
    });

    return {
        salary: salary as bigint | undefined,
        error,
        isLoading,
        refetch,
    };
}

/**
 * Custom hook for reading the trusted notary address
 */
export function useTrustedNotary() {
    const { data: notaryAddress, error, isLoading } = useReadContract({
        address: PRIVATE_PAYROLL_ADDRESS,
        abi: PRIVATE_PAYROLL_ABI,
        functionName: 'TRUSTED_NOTARY',
    });

    return {
        notaryAddress: notaryAddress as `0x${string}` | undefined,
        error,
        isLoading,
    };
}
