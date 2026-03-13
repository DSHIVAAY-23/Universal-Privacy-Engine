"use client";

import { useState, useCallback, useEffect } from "react";
import { ethers } from "ethers";

// Oasis Sapphire Testnet — chainId 0x5aff = 23295
// Oasis Sapphire Mainnet  — chainId 0x5afe = 23294
export const OASIS_SAPPHIRE_TESTNET = {
    chainId: "0x5aff",
    chainName: "Oasis Sapphire Testnet",
    nativeCurrency: { name: "TEST", symbol: "TEST", decimals: 18 },
    rpcUrls: ["https://testnet.sapphire.oasis.dev"],
    blockExplorerUrls: ["https://explorer.oasis.io/testnet/sapphire"],
};

export type WalletState = {
    address: string | null;
    shortAddress: string | null;
    isConnected: boolean;
    isConnecting: boolean;
    chainId: string | null;
    isCorrectNetwork: boolean;
    provider: ethers.BrowserProvider | null;
    signer: ethers.JsonRpcSigner | null;
    connect: () => Promise<void>;
    disconnect: () => void;
    switchToOasis: () => Promise<void>;
    error: string | null;
};

function shortenAddress(addr: string) {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
}

export function useWallet(): WalletState {
    const [address, setAddress] = useState<string | null>(null);
    const [chainId, setChainId] = useState<string | null>(null);
    const [isConnecting, setIsConnecting] = useState(false);
    const [provider, setProvider] = useState<ethers.BrowserProvider | null>(null);
    const [signer, setSigner] = useState<ethers.JsonRpcSigner | null>(null);
    const [error, setError] = useState<string | null>(null);

    const isConnected = !!address;
    const isCorrectNetwork = chainId === OASIS_SAPPHIRE_TESTNET.chainId;
    const shortAddress = address ? shortenAddress(address) : null;

    const switchToOasis = useCallback(async () => {
        if (typeof window === "undefined" || !window.ethereum) return;
        try {
            await window.ethereum.request({
                method: "wallet_switchEthereumChain",
                params: [{ chainId: OASIS_SAPPHIRE_TESTNET.chainId }],
            });
        } catch (switchError: unknown) {
            // Chain not added — add it
            if ((switchError as { code: number }).code === 4902) {
                await window.ethereum.request({
                    method: "wallet_addEthereumChain",
                    params: [OASIS_SAPPHIRE_TESTNET],
                });
            } else {
                throw switchError;
            }
        }
    }, []);

    const connect = useCallback(async () => {
        if (typeof window === "undefined" || !window.ethereum) {
            setError("MetaMask not detected. Please install MetaMask.");
            return;
        }
        setIsConnecting(true);
        setError(null);
        try {
            const p = new ethers.BrowserProvider(window.ethereum);
            await p.send("eth_requestAccounts", []);
            const s = await p.getSigner();
            const addr = await s.getAddress();
            const network = await p.getNetwork();
            const cid = "0x" + network.chainId.toString(16);

            setProvider(p);
            setSigner(s);
            setAddress(addr);
            setChainId(cid);

            if (cid !== OASIS_SAPPHIRE_TESTNET.chainId) {
                await switchToOasis();
            }
        } catch (e: unknown) {
            setError((e as Error).message ?? "Wallet connection failed.");
        } finally {
            setIsConnecting(false);
        }
    }, [switchToOasis]);

    const disconnect = useCallback(() => {
        setAddress(null);
        setChainId(null);
        setProvider(null);
        setSigner(null);
        setError(null);
    }, []);

    // Sync chain/account changes from MetaMask
    useEffect(() => {
        if (typeof window === "undefined" || !window.ethereum) return;
        const eth = window.ethereum;

        const handleAccountsChanged = (...args: unknown[]) => {
            const accounts = args[0] as string[];
            if (accounts.length === 0) disconnect();
            else setAddress(accounts[0]);
        };
        const handleChainChanged = (...args: unknown[]) => {
            setChainId(args[0] as string);
        };

        eth.on("accountsChanged", handleAccountsChanged);
        eth.on("chainChanged", handleChainChanged);
        return () => {
            eth.removeListener("accountsChanged", handleAccountsChanged);
            eth.removeListener("chainChanged", handleChainChanged);
        };
    }, [disconnect]);

    return {
        address,
        shortAddress,
        isConnected,
        isConnecting,
        chainId,
        isCorrectNetwork,
        provider,
        signer,
        connect,
        disconnect,
        switchToOasis,
        error,
    };
}

// Extend window for TypeScript
declare global {
    interface Window {
        ethereum?: {
            request: (args: { method: string; params?: unknown[] }) => Promise<unknown>;
            on: (event: string, handler: (...args: unknown[]) => void) => void;
            removeListener: (event: string, handler: (...args: unknown[]) => void) => void;
        };
    }
}
