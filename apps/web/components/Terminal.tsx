"use client";

import type { TerminalLine } from "@/app/page";

interface TerminalProps {
    lines: TerminalLine[];
    isRunning: boolean;
}

export default function Terminal({ lines, isRunning }: TerminalProps) {
    return (
        <div className="w-full rounded-xl border border-gray-200 dark:border-gray-800 overflow-hidden shadow-md dark:shadow-2xl">
            {/* Title bar */}
            <div className="flex items-center gap-2 bg-gray-100 dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800 px-4 py-3">
                <div className="flex gap-1.5">
                    <div className="h-3 w-3 rounded-full bg-red-400/70" />
                    <div className="h-3 w-3 rounded-full bg-yellow-400/70" />
                    <div className="h-3 w-3 rounded-full bg-green-400/70" />
                </div>
                <div className="flex-1 flex items-center justify-center gap-2">
                    <svg className="h-3.5 w-3.5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                    </svg>
                    <span className="font-mono text-xs text-gray-400 dark:text-gray-500 tracking-wide">
                        upe-prover — zk-tls-oracle — bash
                    </span>
                </div>
                {isRunning && (
                    <div className="flex items-center gap-1.5">
                        <div className="h-1.5 w-1.5 rounded-full bg-green-500 animate-pulse" />
                        <span className="font-mono text-[10px] text-green-600 dark:text-green-400 uppercase tracking-widest">Running</span>
                    </div>
                )}
            </div>

            {/* Body */}
            <div className="relative min-h-[280px] bg-gray-50 dark:bg-gray-950 p-5 scan-line">
                {/* Dark-mode only glow */}
                <div className="pointer-events-none absolute inset-0 rounded-b-xl opacity-0 dark:opacity-30"
                    style={{ background: "radial-gradient(ellipse at 50% 0%, rgba(0,255,136,0.06) 0%, transparent 70%)" }}
                />

                {lines.length === 0 && !isRunning ? (
                    <div className="flex items-center gap-2 font-mono text-sm text-gray-300 dark:text-gray-600">
                        <span className="text-green-500">$</span>
                        <span>Awaiting proof generation request</span>
                        <span className="inline-block h-4 w-2 bg-gray-200 dark:bg-gray-600 animate-blink ml-0.5" />
                    </div>
                ) : (
                    <div className="space-y-1.5 font-mono text-sm">
                        {lines.map((line, i) => (
                            <div
                                key={i}
                                className="flex items-start gap-2 leading-relaxed"
                                style={{ animation: "fadeSlideIn 0.3s ease-out forwards" }}
                            >
                                <span className="mt-0.5 shrink-0 text-green-500 dark:text-green-600 select-none">›</span>
                                <span
                                    className={
                                        line.isError
                                            ? "text-red-500 dark:text-red-400"
                                            : line.isSuccess
                                                ? "text-green-600 dark:text-green-400 glow-green-text font-semibold"
                                                : line.isBenchmark
                                                    ? "text-cyan-600 dark:text-cyan-400 font-semibold"
                                                    : "text-gray-500 dark:text-gray-400"
                                    }
                                >
                                    {line.text}
                                </span>
                            </div>
                        ))}
                        {isRunning && (
                            <div className="flex items-center gap-2 pt-1">
                                <span className="text-green-500 dark:text-green-600 select-none">›</span>
                                <div className="flex items-center gap-1">
                                    {[0, 1, 2].map((i) => (
                                        <div
                                            key={i}
                                            className="h-1.5 w-1.5 rounded-full bg-green-500"
                                            style={{ animation: `bounce 1.2s ease-in-out ${i * 0.15}s infinite` }}
                                        />
                                    ))}
                                </div>
                            </div>
                        )}
                    </div>
                )}
            </div>

            <style jsx>{`
                @keyframes fadeSlideIn {
                    from { opacity: 0; transform: translateY(4px); }
                    to   { opacity: 1; transform: translateY(0); }
                }
                @keyframes bounce {
                    0%, 80%, 100% { transform: scale(0.6); opacity: 0.4; }
                    40%           { transform: scale(1); opacity: 1; }
                }
            `}</style>
        </div>
    );
}
