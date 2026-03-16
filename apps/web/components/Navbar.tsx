"use client";

import { type Dispatch, type SetStateAction } from "react";
import ThemeToggle from "./ThemeToggle";
import { Wallet, ChevronDown, AlertTriangle } from "lucide-react";
import type { WalletState } from "@/hooks/useWallet";
import { Network } from "@/lib/contracts";

const NETWORKS = [
    { id: "oasis", label: Network.OasisSapphire },
    { id: "secret", label: Network.SecretNetwork },
    { id: "zksync", label: Network.zkSyncEra },
    { id: "aleo", label: Network.Aleo },
    { id: "mina", label: Network.Mina },
];

interface NavbarProps {
    selectedNetwork: Network;
    onNetworkChange: Dispatch<SetStateAction<Network>>;
    wallet: WalletState;
}

export default function Navbar({ selectedNetwork, onNetworkChange, wallet }: NavbarProps) {
    const { isConnected, isConnecting, shortAddress, isCorrectNetwork, connect, switchToOasis } = wallet;

    return (
        <header className="sticky top-0 z-50 border-b border-gray-200 dark:border-gray-800 bg-white/80 dark:bg-gray-950/80 backdrop-blur-xl">
            <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
                {/* Logo */}
                <div className="flex items-center gap-3">
                    <div className="relative flex h-9 w-9 items-center justify-center">
                        <svg viewBox="0 0 36 36" fill="none" className="absolute inset-0 h-full w-full">
                            <polygon
                                points="18,2 33,10.5 33,25.5 18,34 3,25.5 3,10.5"
                                stroke="#00cc66"
                                strokeWidth="1.5"
                                fill="rgba(0,204,102,0.08)"
                            />
                            <polygon
                                points="18,8 28,14 28,22 18,28 8,22 8,14"
                                fill="rgba(0,204,102,0.15)"
                            />
                        </svg>
                        <span className="relative text-xs font-bold text-green-600 dark:text-green-400">Σ</span>
                    </div>
                    <div>
                        <span className="text-lg font-bold tracking-tight text-gray-900 dark:text-white">
                            UPE<span className="text-green-600 dark:text-green-400"> Labs</span>
                        </span>
                        <div className="flex items-center gap-1.5 mt-0.5">
                            <span className="h-1.5 w-1.5 rounded-full bg-green-500 animate-pulse" />
                            <span className="text-[10px] font-mono text-gray-400 dark:text-gray-500 uppercase tracking-widest">
                                v0.4.1-alpha • Mainnet
                            </span>
                        </div>
                    </div>
                </div>

                {/* Right controls */}
                <div className="flex items-center gap-2">
                    {/* Network Selector */}
                    <div className="relative">
                        <select
                            value={selectedNetwork}
                            onChange={(e) => onNetworkChange(e.target.value as Network)}
                            className="
                                appearance-none cursor-pointer
                                rounded-lg border border-gray-200 dark:border-gray-700
                                bg-gray-50 dark:bg-gray-900
                                pl-3 pr-8 py-2
                                text-sm font-mono text-cyan-700 dark:text-cyan-300
                                focus:outline-none focus:border-cyan-400 dark:focus:border-cyan-500
                                focus:ring-1 focus:ring-cyan-400/30 dark:focus:ring-cyan-500/30
                                hover:border-gray-300 dark:hover:border-gray-600 transition-colors
                            "
                        >
                            {NETWORKS.map((n) => (
                                <option key={n.id} value={n.label} className="bg-white dark:bg-gray-900">
                                    {n.label}
                                </option>
                            ))}
                        </select>
                        <ChevronDown className="pointer-events-none absolute right-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-gray-400" />
                    </div>

                    {/* Wrong network warning */}
                    {isConnected && !isCorrectNetwork && (
                        <button
                            onClick={switchToOasis}
                            title="Click to switch to Oasis Sapphire"
                            className="flex items-center gap-1.5 rounded-lg border border-yellow-500/40 bg-yellow-500/10 px-3 py-2 text-xs font-semibold text-yellow-600 dark:text-yellow-400 hover:bg-yellow-500/20 transition-all"
                        >
                            <AlertTriangle className="h-3.5 w-3.5" />
                            Wrong Network
                        </button>
                    )}

                    {/* Theme Toggle */}
                    <ThemeToggle />

                    {/* Connect Wallet */}
                    {isConnected ? (
                        <div className="flex items-center gap-2 rounded-lg border border-green-500/40 bg-green-500/5 dark:bg-green-500/5 px-3 py-2">
                            <span className="h-2 w-2 rounded-full bg-green-500 shadow-[0_0_6px_#22c55e]" />
                            <span className="font-mono text-xs font-semibold text-green-700 dark:text-green-400">
                                {shortAddress}
                            </span>
                        </div>
                    ) : (
                        <button
                            onClick={connect}
                            disabled={isConnecting}
                            className="
                                flex items-center gap-2 rounded-lg border
                                border-gray-200 dark:border-gray-700
                                bg-gray-50 dark:bg-gray-900
                                px-4 py-2 text-sm font-semibold
                                text-gray-700 dark:text-gray-200
                                hover:border-green-500/50 hover:text-green-700 dark:hover:text-green-400
                                hover:bg-gray-100 dark:hover:bg-gray-800
                                disabled:opacity-60 disabled:cursor-wait
                                transition-all duration-200 group
                            "
                        >
                            <Wallet className="h-4 w-4" />
                            {isConnecting ? "Connecting..." : "Connect Wallet"}
                        </button>
                    )}
                </div>
            </div>
        </header>
    );
}

export { NETWORKS };
