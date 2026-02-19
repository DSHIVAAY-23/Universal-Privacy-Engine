import { useState } from 'react';
import { notaryAPI } from '../lib/notary';
import type { STLOPProof } from '../types';

/**
 * Custom hook for Notary API interactions
 * Handles proof generation with proper error handling
 * NO MOCK FALLBACK - Strictly uses real Rust Notary API
 */
export function useNotaryAPI() {
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [proof, setProof] = useState<STLOPProof | null>(null);

    /**
     * Generate STLOP proof for an employee address
     * Strictly uses the real Rust Notary API - no mock fallback
     * @param employeeAddress - Employee's Ethereum address
     * @returns STLOP proof or null if error occurs
     */
    const generateProof = async (employeeAddress: string): Promise<STLOPProof | null> => {
        setIsLoading(true);
        setError(null);
        setProof(null);

        try {
            // Call the real Rust Notary API (no fallback)
            const generatedProof = await notaryAPI.generateProof({ employee_address: employeeAddress });

            console.log('✅ Proof generated from Rust Notary API:');
            console.log('├─ Salary:', generatedProof.salary);
            console.log('├─ Timestamp:', generatedProof.timestamp);
            console.log('├─ Notary:', generatedProof.notary_pubkey);
            console.log('└─ Signature:', generatedProof.signature.slice(0, 20) + '...');

            setProof(generatedProof);
            return generatedProof;
        } catch (err) {
            const errorMessage = err instanceof Error ? err.message : 'Failed to generate proof';
            console.error('❌ Notary API Error:', errorMessage);
            setError(errorMessage);
            return null;
        } finally {
            setIsLoading(false);
        }
    };

    /**
     * Check Notary service health
     * @returns true if Notary service is healthy, false otherwise
     */
    const checkHealth = async () => {
        try {
            const health = await notaryAPI.checkHealth();
            console.log('✅ Notary service is healthy:', health.notary_address);
            return health.status === 'ok';
        } catch (err) {
            console.error('❌ Notary service health check failed:', err);
            return false;
        }
    };

    return {
        generateProof,
        checkHealth,
        isLoading,
        error,
        proof,
    };
}
