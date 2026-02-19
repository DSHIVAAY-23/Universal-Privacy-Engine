import { ReactNode } from 'react';

interface LayoutProps {
    children: ReactNode;
}

export function Layout({ children }: LayoutProps) {
    return (
        <div className="min-h-screen bg-gradient-to-br from-gray-950 via-gray-900 to-gray-950">
            {/* Background Pattern */}
            <div className="fixed inset-0 bg-[url('/grid.svg')] bg-center opacity-5"></div>

            {/* Content */}
            <div className="relative">
                {children}
            </div>

            {/* Footer */}
            <footer className="border-t border-gray-800 mt-20">
                <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                    <div className="flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
                        <div className="text-sm text-gray-400">
                            <p>© 2026 Universal Privacy Engine. Research Prototype.</p>
                            <p className="text-xs mt-1">Built for Oasis ROSE Bloom Grant</p>
                        </div>
                        <div className="flex space-x-6 text-sm text-gray-400">
                            <a href="https://docs.oasis.io/dapp/sapphire/" target="_blank" rel="noopener noreferrer" className="hover:text-sapphire-400 transition-colors">
                                Sapphire Docs
                            </a>
                            <a href="https://testnet.explorer.sapphire.oasis.io" target="_blank" rel="noopener noreferrer" className="hover:text-sapphire-400 transition-colors">
                                Explorer
                            </a>
                            <a href="https://github.com" target="_blank" rel="noopener noreferrer" className="hover:text-sapphire-400 transition-colors">
                                GitHub
                            </a>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    );
}
