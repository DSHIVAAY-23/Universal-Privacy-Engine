import { create } from 'zustand';
import { Network } from '../types';
import type { AppState, STLOPProof } from '../types';

/**
 * Global Application State Store (Zustand)
 * Manages proof data, salary data, and UI states
 */
export const useAppStore = create<AppState>((set) => ({
    // Wallet state
    isConnected: false,
    address: null,

    // Proof state
    currentProof: null,
    setProof: (proof: STLOPProof | null) => set({ currentProof: proof }),

    // Network state
    selectedNetwork: Network.OasisSapphire,
    setSelectedNetwork: (network: Network) => set({ selectedNetwork: network }),

    // Salary state
    verifiedSalary: null,
    setSalary: (salary: bigint | null) => set({ verifiedSalary: salary }),

    // UI state
    isLoading: false,
    setLoading: (loading: boolean) => set({ isLoading: loading }),
    error: null,
    setError: (error: string | null) => set({ error }),
}));
