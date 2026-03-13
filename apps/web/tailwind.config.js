/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: "class",
    content: [
        "./app/**/*.{js,ts,jsx,tsx,mdx}",
        "./components/**/*.{js,ts,jsx,tsx,mdx}",
    ],
    theme: {
        extend: {
            fontFamily: {
                sans: ["Inter", "system-ui", "sans-serif"],
                mono: ["JetBrains Mono", "Fira Code", "monospace"],
            },
            colors: {
                "neon-green": "#00ff88",
                "neon-cyan": "#00d4ff",
                "neon-purple": "#8b5cf6",
            },
            animation: {
                "pulse-slow": "pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite",
                "glow-pulse": "glow 2s ease-in-out infinite alternate",
                blink: "blink 1s step-end infinite",
            },
            keyframes: {
                glow: {
                    from: { boxShadow: "0 0 10px #00ff8844, 0 0 20px #00ff8822" },
                    to: { boxShadow: "0 0 20px #00ff8888, 0 0 40px #00ff8844, 0 0 60px #00ff8822" },
                },
                blink: {
                    "0%, 100%": { opacity: "1" },
                    "50%": { opacity: "0" },
                },
            },
            boxShadow: {
                "neon-green": "0 0 20px #00ff8855, 0 0 40px #00ff8833",
                "neon-cyan": "0 0 20px #00d4ff55, 0 0 40px #00d4ff33",
            },
        },
    },
    plugins: [],
};
