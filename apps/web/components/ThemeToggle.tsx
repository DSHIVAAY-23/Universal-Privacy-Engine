"use client";

import { useTheme } from "next-themes";
import { Sun, Moon } from "lucide-react";
import { useEffect, useState } from "react";

export default function ThemeToggle() {
    const { theme, setTheme } = useTheme();
    const [mounted, setMounted] = useState(false);

    // Avoid hydration mismatch — only render icon after mount
    useEffect(() => setMounted(true), []);
    if (!mounted) return <div className="h-9 w-9" />;

    const isDark = theme === "dark";

    return (
        <button
            onClick={() => setTheme(isDark ? "light" : "dark")}
            aria-label="Toggle theme"
            className="
                relative flex h-9 w-9 items-center justify-center rounded-lg
                border border-gray-700 dark:border-gray-700
                bg-gray-100 dark:bg-gray-900
                text-gray-600 dark:text-gray-400
                hover:border-green-500/50 hover:text-green-600 dark:hover:text-green-400
                hover:bg-gray-50 dark:hover:bg-gray-800
                transition-all duration-200 group overflow-hidden
            "
        >
            {/* Glow ring on hover */}
            <span className="pointer-events-none absolute inset-0 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity"
                style={{ boxShadow: "inset 0 0 8px rgba(0,255,136,0.15)" }}
            />
            {isDark ? (
                <Sun className="h-4 w-4 transition-transform duration-300 group-hover:rotate-12" />
            ) : (
                <Moon className="h-4 w-4 transition-transform duration-300 group-hover:-rotate-12" />
            )}
        </button>
    );
}
