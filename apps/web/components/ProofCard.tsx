"use client";

import { Wallet, Shield } from "lucide-react";

interface ProofCardProps {
    assetContract: string;
    onAssetContractChange: (val: string) => void;
    collateralValue: string;
    onCollateralValueChange: (val: string) => void;
    onGenerate: () => void;
    isRunning: boolean;
    selectedNetwork: string;
    walletConnected?: boolean;
}

export default function ProofCard({
    assetContract,
    onAssetContractChange,
    collateralValue,
    onCollateralValueChange,
    onGenerate,
    isRunning,
    selectedNetwork,
    walletConnected = false,
}: ProofCardProps) {
    const canGenerate = !isRunning && (assetContract || "").trim() && (collateralValue || "").trim();

    return (
        <div className="relative overflow-hidden rounded-2xl border border-gray-200 dark:border-gray-800 bg-white/90 dark:bg-gray-900/90 backdrop-blur-xl p-8 shadow-sm dark:shadow-none">
            {/* Corner accent lines */}
            <div className="pointer-events-none absolute left-0 top-0 h-16 w-16">
                <div className="absolute left-0 top-0 h-px w-12 bg-gradient-to-r from-green-500/60 to-transparent" />
                <div className="absolute left-0 top-0 h-12 w-px bg-gradient-to-b from-green-500/60 to-transparent" />
            </div>
            <div className="pointer-events-none absolute right-0 bottom-0 h-16 w-16">
                <div className="absolute right-0 bottom-0 h-px w-12 bg-gradient-to-l from-green-500/60 to-transparent" />
                <div className="absolute right-0 bottom-0 h-12 w-px bg-gradient-to-t from-green-500/60 to-transparent" />
            </div>

            {/* Header */}
            <div className="flex items-start justify-between mb-7">
                <div>
                    <div className="flex items-center gap-2.5 mb-2">
                        <div className="flex h-8 w-8 items-center justify-center rounded-lg border border-green-500/30 bg-green-500/10">
                            <Shield className="h-4 w-4 text-green-600 dark:text-green-400" />
                        </div>
                        <span className="font-mono text-xs text-green-700 dark:text-green-400 uppercase tracking-widest bg-green-500/10 px-2 py-0.5 rounded border border-green-500/20">
                            RWA-SHIELD-v1
                        </span>
                    </div>
                    <h1 className="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                        Private RWA Collateral Shield
                    </h1>
                    <p className="mt-1.5 text-sm text-gray-500 leading-relaxed max-w-lg">
                        Prove ownership of tokenized assets on a source chain to utilize in DeFi on a
                        destination chain, without revealing your wallet address or total net worth.
                        Verified natively on{" "}
                        <span className="text-cyan-600 dark:text-cyan-400 font-medium">{selectedNetwork}</span>.
                    </p>
                </div>

                {/* Status badge */}
                <div className="shrink-0 flex items-center gap-2 rounded-full border border-green-500/20 bg-green-500/5 px-3 py-1.5">
                    <div className="h-2 w-2 rounded-full bg-green-500 animate-pulse-slow" />
                    <span className="font-mono text-xs text-green-700 dark:text-green-400">ONLINE</span>
                </div>
            </div>

            {/* Stats row */}
            <div className="grid grid-cols-3 gap-3 mb-7">
                {[
                    { label: "RWA Proofs Issued", value: "3,241" },
                    { label: "Total Value Locked", value: "$1.2B" },
                    { label: "Nullifier Registry", value: "Active" },
                ].map((stat) => (
                    <div
                        key={stat.label}
                        className="rounded-lg border border-gray-100 dark:border-gray-800 bg-gray-50 dark:bg-gray-950/60 px-4 py-3"
                    >
                        <div className="font-mono text-lg font-bold text-gray-900 dark:text-white">{stat.value}</div>
                        <div className="text-xs text-gray-400 dark:text-gray-600 mt-0.5">{stat.label}</div>
                    </div>
                ))}
            </div>

            {/* Dual inputs */}
            <div className="grid grid-cols-2 gap-4 mb-5">
                {/* Input A — Asset Contract */}
                <div>
                    <label className="block text-xs font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500 mb-2">
                        Source Chain Asset Contract
                    </label>
                    <div className="relative flex items-center">
                        <div className="absolute left-3 flex items-center gap-1.5">
                            <div className="h-2 w-2 rounded-full bg-purple-500/60" />
                            <span className="font-mono text-xs text-gray-400">0x</span>
                        </div>
                        <input
                            type="text"
                            value={assetContract.startsWith("0x") ? assetContract.slice(2) : assetContract}
                            onChange={(e) => {
                                let val = e.target.value.trim();
                                if (val.startsWith("0x")) val = val.slice(2);
                                onAssetContractChange("0x" + val);
                            }}
                            placeholder=""
                            disabled={isRunning}
                            maxLength={42}
                            className="
                                w-full rounded-xl border border-gray-200 dark:border-gray-700
                                bg-gray-50 dark:bg-gray-950
                                pl-14 pr-3 py-3
                                font-mono text-sm text-gray-800 dark:text-gray-200 placeholder-gray-300 dark:placeholder-gray-600
                                focus:outline-none focus:border-purple-400 dark:focus:border-purple-500/60 focus:ring-1 focus:ring-purple-400/20 dark:focus:ring-purple-500/20
                                hover:border-gray-300 dark:hover:border-gray-600
                                disabled:opacity-50 disabled:cursor-not-allowed
                                transition-all duration-200
                            "
                        />
                    </div>
                </div>

                {/* Input B — Collateral Value */}
                <div>
                    <label className="block text-xs font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500 mb-2">
                        Minimum Required Collateral Value ($)
                    </label>
                    <div className="relative flex items-center">
                        <div className="absolute left-3 flex items-center gap-1.5">
                            <div className="h-2 w-2 rounded-full bg-cyan-500/60" />
                            <span className="font-mono text-xs text-gray-400">USD</span>
                        </div>
                        <input
                            type="number"
                            value={collateralValue}
                            onChange={(e) => onCollateralValueChange(e.target.value)}
                            placeholder="500000"
                            disabled={isRunning}
                            min={0}
                            className="
                                w-full rounded-xl border border-gray-200 dark:border-gray-700
                                bg-gray-50 dark:bg-gray-950
                                pl-14 pr-3 py-3
                                font-mono text-sm text-gray-800 dark:text-gray-200 placeholder-gray-300 dark:placeholder-gray-600
                                focus:outline-none focus:border-cyan-400 dark:focus:border-cyan-500/60 focus:ring-1 focus:ring-cyan-400/20 dark:focus:ring-cyan-500/20
                                hover:border-gray-300 dark:hover:border-gray-600
                                disabled:opacity-50 disabled:cursor-not-allowed
                                transition-all duration-200
                                [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
                            "
                        />
                    </div>
                </div>
            </div>

            {/* Wallet hint */}
            {!walletConnected && (
                <div className="mb-4 flex items-center gap-2 rounded-lg border border-amber-200 dark:border-amber-900/40 bg-amber-50 dark:bg-amber-950/20 px-3 py-2">
                    <Wallet className="h-3.5 w-3.5 shrink-0 text-amber-600 dark:text-amber-500" />
                    <span className="text-xs text-amber-700 dark:text-amber-400">
                        Connect your wallet to submit the nullifier on-chain to Oasis Sapphire. Without a wallet, a simulation runs.
                    </span>
                </div>
            )}

            {/* Generate Button */}
            <button
                onClick={onGenerate}
                disabled={!canGenerate}
                className={`
                    relative w-full rounded-xl px-6 py-4 font-bold text-base tracking-wide
                    transition-all duration-300 overflow-hidden group
                    disabled:cursor-not-allowed
                    ${isRunning
                        ? "bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-400 dark:text-gray-500 cursor-wait"
                        : !assetContract.trim() || !collateralValue.trim()
                            ? "bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-400 dark:text-gray-500"
                            : "bg-green-500 hover:bg-green-400 text-gray-950 glow-green hover:scale-[1.01] active:scale-[0.99]"
                    }
                `}
            >
                {/* Shimmer */}
                {canGenerate && (
                    <div className="absolute inset-0 -translate-x-full group-hover:translate-x-full transition-transform duration-700 bg-gradient-to-r from-transparent via-white/20 to-transparent" />
                )}

                <span className="relative flex items-center justify-center gap-3">
                    {isRunning ? (
                        <>
                            <svg className="h-5 w-5 animate-spin text-green-500" fill="none" viewBox="0 0 24 24">
                                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                            </svg>
                            <span className="font-mono text-green-600 dark:text-green-400">
                                {walletConnected ? "Proving & Anchoring Nullifier..." : "Generating RWA Proof..."}
                            </span>
                        </>
                    ) : (
                        <>
                            <Shield className="h-5 w-5" />
                            {walletConnected ? "Generate & Anchor Nullifier On-Chain" : "Generate ZK Proof"}
                            <svg className="h-4 w-4 opacity-70" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7l5 5m0 0l-5 5m5-5H6" />
                            </svg>
                        </>
                    )}
                </span>
            </button>
        </div>
    );
}
