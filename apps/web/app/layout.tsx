import type { Metadata } from "next";
import "./globals.css";
import { Providers } from "./providers";

export const metadata: Metadata = {
    title: "UPE Labs — Universal Privacy Engine",
    description:
        "Enterprise-grade multi-chain ZK-TLS credential oracle infrastructure. Privacy-preserving proof generation across Secret Network, zkSync Era, Aleo, and Mina Protocol.",
    keywords: ["ZK-TLS", "Zero Knowledge", "Secret Network", "zkSync", "Privacy Engine"],
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en" suppressHydrationWarning>
            <body className="min-h-screen bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 antialiased transition-colors duration-200">
                <Providers>{children}</Providers>
            </body>
        </html>
    );
}
