// TypeScript interfaces for UPE Frontend

export interface STLOPProof {
    salary: string;            // uint256 as string (e.g., "75000")
    timestamp: number;         // Unix timestamp
    signature: string;         // Hex-encoded signature (0x...)
    notary_pubkey: string;     // Notary's public key (for verification)
}

export interface GenerateProofRequest {
    employee_address: string;  // Ethereum address (0x...)
    api_url?: string;          // Optional: Custom payroll API
}

export interface HealthResponse {
    status: "ok" | "error";
    notary_address: string;
    timestamp: number;
}

export type VerifyStep = 'idle' | 'fetching-proof' | 'preview' | 'submitting' | 'success';

export interface VerifyState {
    step: VerifyStep;
    proof: STLOPProof | null;
    txHash: string | null;
    error: string | null;
}

export interface AppState {
    // Wallet
    isConnected: boolean;
    address: string | null;

    // Proof
    currentProof: STLOPProof | null;
    setProof: (proof: STLOPProof | null) => void;

    // Salary
    verifiedSalary: bigint | null;
    setSalary: (salary: bigint | null) => void;

    // UI
    isLoading: boolean;
    setLoading: (loading: boolean) => void;
    error: string | null;
    setError: (error: string | null) => void;
}
