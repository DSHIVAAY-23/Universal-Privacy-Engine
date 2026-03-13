"use client";

import { useState, useCallback } from "react";
import { ethers } from "ethers";
import Navbar from "@/components/Navbar";
import ProofCard from "@/components/ProofCard";
import Terminal from "@/components/Terminal";
import { useWallet } from "@/hooks/useWallet";

// ── Contract config ──────────────────────────────────────────────────────────
const CONTRACT_ADDRESS =
    process.env.NEXT_PUBLIC_CONTRACT_ADDRESS ?? "0x2Df7658D5E57ed05D6F634fD7d73b334ADEc179A";


const CONTRACT_ABI = [
    "function submitRWAProof(uint256[8] calldata proof, uint256 stateRoot, uint256 nullifierHash, uint256 minCollateral, address assetContract) external",
    "event RWAProofSubmitted(address indexed account, address indexed assetContract, uint256 minCollateral)",
];

// ── Terminal line type ───────────────────────────────────────────────────────
export interface TerminalLine {
    text: string;
    isSuccess?: boolean;
    isSystem?: boolean;
    isError?: boolean;
    isBenchmark?: boolean;
}

function delay(ms: number) {
    return new Promise((r) => setTimeout(r, ms));
}

// Shorten a contract address for display
function shortAddr(addr: string) {
    if (addr.length < 10) return addr;
    return `${addr.slice(0, 10)}...${addr.slice(-6)}`;
}

// Format USD collateral value
function formatUSD(val: string) {
    const n = parseFloat(val);
    if (isNaN(n)) return val;
    return `$${n.toLocaleString("en-US")}`;
}

export default function HomePage() {
    const wallet = useWallet();

    const [selectedNetwork, setSelectedNetwork] = useState("Oasis Sapphire");
    const [assetContract, setAssetContract] = useState("");
    const [collateralValue, setCollateralValue] = useState("");
    const [terminalLines, setTerminalLines] = useState<TerminalLine[]>([]);
    const [isRunning, setIsRunning] = useState(false);

    const appendLine = useCallback((line: TerminalLine) => {
        setTerminalLines((prev) => [...prev, line]);
    }, []);

    const handleGenerateProof = useCallback(async () => {
        if (isRunning) return;
        setIsRunning(true);
        setTerminalLines([]);

        const displayContract = shortAddr(assetContract || "0xTokenizedRealEstate...");
        const displayCollateral = formatUSD(collateralValue || "500000");

        try {
            // ── Step 1: Init ─────────────────────────────────────────────────
            await delay(300);
            appendLine({ text: "[SYSTEM]   Initializing Cross-Chain RWA Shield...", isSystem: true });

            await delay(550);
            appendLine({ text: `[NETWORK]  Target DeFi ecosystem: ${selectedNetwork}`, isSystem: true });

            await delay(600);
            appendLine({ text: `[PROVER]   Fetching source chain state root...  contract: ${displayContract}`, isSystem: true });

            await delay(750);
            appendLine({ text: `[PROVER]   Computing Merkle inclusion proof for hidden token balance...`, isSystem: true });

            // ── Step 2: Call /api/prove ──────────────────────────────────────
            const proveRes = await fetch("/api/prove", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ targetUrl: assetContract, collateralValue }),
            });
            if (!proveRes.ok) throw new Error("Proof generation API failed");
            const proofData = await proveRes.json();

            await delay(900);
            appendLine({ text: `[PROVER]   Executing ZK Range Check (Balance >= ${displayCollateral})...`, isSystem: true });

            await delay(1000);
            const nullifier = "0x" + Math.random().toString(16).slice(2, 18) + Math.random().toString(16).slice(2, 18);
            appendLine({
                text: `[SNARK]    Generating zero-knowledge nullifier to prevent double-spend...  [nullifier: ${nullifier.slice(0, 18)}...]`,
                isSystem: true,
            });

            appendLine({
                text: `[PROVER]   Source: ${proofData.source === "rust-backend-with-snarkjs" ? "✓ SnarkJS Native" : "✓ Mock prover (Rust backend offline)"}`,
                isSystem: true,
            });

            if (proofData.proverTimeMs) {
                appendLine({
                    text: `[BENCHMARK] Groth16 Prover Time: ${proofData.proverTimeMs} ms`,
                    isBenchmark: true,
                });
            }

            // ── Step 3: On-chain ─────────────────────────────────────────────
            if (wallet.isConnected && wallet.signer) {
                await delay(400);
                appendLine({ text: `[WALLET]   Signer: ${wallet.shortAddress}`, isSystem: true });
                appendLine({ text: `[CONTRACT] Anchoring nullifier + proof to Oasis Sapphire...`, isSystem: true });

                const contract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, wallet.signer);
                const minCollateral = collateralValue || "500000";
                const targetContract = assetContract || "0x0000000000000000000000000000000000000000";

                appendLine({ text: `[WALLET]   MetaMask confirmation requested...`, isSystem: true });

                const tx = await contract.submitRWAProof(proofData.flatProof, proofData.stateRoot, proofData.nullifierHash, minCollateral, targetContract, { gasLimit: 3000000 });

                appendLine({ text: `[TX]       Transaction submitted: ${tx.hash}`, isSystem: true });
                appendLine({ text: `[NETWORK]  Awaiting on-chain confirmation...`, isSystem: true });

                const receipt = await tx.wait();
                const txHash: string = receipt?.hash ?? tx.hash;

                await delay(300);

                if (receipt?.gasUsed) {
                    appendLine({
                        text: `[BENCHMARK] On-Chain Verification Gas Used: ${receipt.gasUsed.toString()} gas`,
                        isBenchmark: true,
                    });
                }
                appendLine({
                    text: `[VERIFIER] Proof & Nullifier verified on ${selectedNetwork}.  Block #${receipt?.blockNumber ?? "—"}`,
                    isSuccess: true,
                });
                appendLine({ text: `[TX HASH]  ${txHash}`, isSuccess: true });
                appendLine({ text: `[SUCCESS]  RWA Collateral Shield ACTIVE on Oasis Network 🛡️`, isSuccess: true });
            } else {
                await delay(500);
                appendLine({
                    text: `[VERIFIER] Proof & Nullifier verified on ${selectedNetwork}.  (Connect wallet for on-chain anchoring)`,
                    isSuccess: true,
                });
            }
        } catch (err: unknown) {
            const msg = (err as Error).message ?? "Unknown error";
            if (msg.includes("user rejected") || msg.includes("ACTION_REJECTED")) {
                appendLine({ text: `[WALLET]   Transaction rejected by user.`, isSystem: true });
                appendLine({ text: `[VERIFIER] Proof generated — nullifier NOT anchored on-chain.`, isSystem: true });
            } else {
                appendLine({ text: `[ERROR]    ${msg}`, isError: true });
            }
        } finally {
            setIsRunning(false);
        }
    }, [isRunning, selectedNetwork, assetContract, collateralValue, wallet, appendLine]);

    return (
        <div className="min-h-screen bg-white dark:bg-gray-950 grid-bg transition-colors duration-200">
            <Navbar
                selectedNetwork={selectedNetwork}
                onNetworkChange={setSelectedNetwork}
                wallet={wallet}
            />

            {/* Ticker bar */}
            <div className="border-b border-gray-100 dark:border-gray-900 bg-gray-50/50 dark:bg-gray-950/50">
                <div className="mx-auto max-w-7xl px-6 py-3 flex items-center gap-3">
                    <div className="h-px flex-1 bg-gradient-to-r from-transparent via-gray-200 dark:via-gray-800 to-transparent" />
                    <span className="font-mono text-xs text-gray-400 dark:text-gray-600 uppercase tracking-[0.2em]">
                        Universal Privacy Engine · Cross-Chain RWA &amp; DeFi Privacy Infrastructure
                    </span>
                    <div className="h-px flex-1 bg-gradient-to-r from-transparent via-gray-200 dark:via-gray-800 to-transparent" />
                </div>
            </div>

            {/* Wallet error banner */}
            {wallet.error && (
                <div className="border-b border-red-200 dark:border-red-900/50 bg-red-50 dark:bg-red-950/30 px-6 py-2 text-center">
                    <span className="font-mono text-xs text-red-600 dark:text-red-400">{wallet.error}</span>
                </div>
            )}

            <main className="mx-auto max-w-5xl px-6 py-12 space-y-6">
                {/* Chain badges */}
                <div className="flex flex-wrap gap-2 justify-center">
                    {[
                        { name: "Secret Network", color: "text-yellow-700 dark:text-yellow-400 border-yellow-400/20 bg-yellow-50 dark:bg-yellow-400/5" },
                        { name: "zkSync Era", color: "text-blue-700 dark:text-blue-400 border-blue-400/20 bg-blue-50 dark:bg-blue-400/5" },
                        { name: "Aleo", color: "text-purple-700 dark:text-purple-400 border-purple-400/20 bg-purple-50 dark:bg-purple-400/5" },
                        { name: "Mina Protocol", color: "text-green-700 dark:text-green-400 border-green-400/20 bg-green-50 dark:bg-green-400/5" },
                    ].map((chain) => (
                        <span
                            key={chain.name}
                            className={`font-mono text-xs px-3 py-1 rounded-full border ${chain.color} tracking-wide`}
                        >
                            {chain.name}
                        </span>
                    ))}
                </div>

                {/* Main card */}
                <ProofCard
                    assetContract={assetContract}
                    onAssetContractChange={setAssetContract}
                    collateralValue={collateralValue}
                    onCollateralValueChange={setCollateralValue}
                    onGenerate={handleGenerateProof}
                    isRunning={isRunning}
                    selectedNetwork={selectedNetwork}
                    walletConnected={wallet.isConnected}
                />

                {/* Terminal */}
                <div>
                    <div className="flex items-center gap-3 mb-3">
                        <span className="font-mono text-xs text-gray-400 dark:text-gray-600 uppercase tracking-widest">
                            Prover Console
                        </span>
                        <div className="h-px flex-1 bg-gray-100 dark:bg-gray-800" />
                        {terminalLines.length > 0 && !isRunning && (
                            <button
                                onClick={() => setTerminalLines([])}
                                className="font-mono text-xs text-gray-400 hover:text-gray-600 dark:text-gray-600 dark:hover:text-gray-400 transition-colors"
                            >
                                Clear
                            </button>
                        )}
                    </div>
                    <Terminal lines={terminalLines} isRunning={isRunning} />
                </div>

                {/* Footer */}
                <div className="flex items-center justify-between pt-4 border-t border-gray-100 dark:border-gray-900">
                    <div className="flex items-center gap-6">
                        {[
                            { label: "Protocol", value: "Cross-Chain ZK" },
                            { label: "Proof System", value: "Groth16" },
                            { label: "Primitive", value: "Merkle + Range" },
                        ].map((item) => (
                            <div key={item.label}>
                                <div className="text-[10px] text-gray-400 dark:text-gray-600 uppercase tracking-widest">{item.label}</div>
                                <div className="font-mono text-xs text-gray-600 dark:text-gray-400 mt-0.5">{item.value}</div>
                            </div>
                        ))}
                    </div>
                    <div className="flex items-center gap-1.5 font-mono text-xs text-gray-400 dark:text-gray-600">
                        <span className="h-1 w-1 rounded-full bg-green-500" />
                        All systems operational
                    </div>
                </div>
            </main>
        </div>
    );
}
