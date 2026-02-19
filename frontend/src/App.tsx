import { Header } from './components/layout/Header';
import { Layout } from './components/layout/Layout';
import { VerifyIncomeCard } from './components/verify/VerifyIncomeCard';
import { SalaryDisplay } from './components/verify/SalaryDisplay';
import { useAccount } from 'wagmi';

function App() {
    const { isConnected } = useAccount();

    return (
        <Layout>
            <Header />

            {/* Hero Section */}
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div className="text-center mb-12">
                    <h1 className="text-5xl md:text-6xl font-bold text-white mb-4">
                        Verify Your Income
                        <span className="block text-transparent bg-clip-text bg-gradient-to-r from-sapphire-400 to-primary-400 mt-2">
                            Privately on Blockchain
                        </span>
                    </h1>
                    <p className="text-xl text-gray-400 max-w-2xl mx-auto">
                        Powered by Oasis Sapphire Confidential EVM — Your salary data is encrypted on-chain using TEE technology
                    </p>
                </div>

                {/* Main Content */}
                <div className="space-y-8">
                    <VerifyIncomeCard />
                    {isConnected && <SalaryDisplay />}
                </div>

                {/* Features Section */}
                <div className="mt-20 grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto">
                    <div className="text-center p-6">
                        <div className="inline-flex items-center justify-center w-16 h-16 bg-sapphire-500/10 rounded-full mb-4">
                            <svg className="w-8 h-8 text-sapphire-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                            </svg>
                        </div>
                        <h3 className="text-lg font-semibold text-white mb-2">Encrypted State</h3>
                        <p className="text-gray-400 text-sm">
                            All data is encrypted at the ParaTime level using Trusted Execution Environments (TEE)
                        </p>
                    </div>

                    <div className="text-center p-6">
                        <div className="inline-flex items-center justify-center w-16 h-16 bg-primary-500/10 rounded-full mb-4">
                            <svg className="w-8 h-8 text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                            </svg>
                        </div>
                        <h3 className="text-lg font-semibold text-white mb-2">Cryptographic Proofs</h3>
                        <p className="text-gray-400 text-sm">
                            STLOP (Signed TLS Off-chain Proofs) ensure data authenticity with notary signatures
                        </p>
                    </div>

                    <div className="text-center p-6">
                        <div className="inline-flex items-center justify-center w-16 h-16 bg-green-500/10 rounded-full mb-4">
                            <svg className="w-8 h-8 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                            </svg>
                        </div>
                        <h3 className="text-lg font-semibold text-white mb-2">Private Access</h3>
                        <p className="text-gray-400 text-sm">
                            Only you can decrypt and view your salary. Even validators cannot read your data
                        </p>
                    </div>
                </div>

                {/* Info Banner */}
                <div className="mt-12 max-w-3xl mx-auto p-6 bg-blue-500/10 border border-blue-500/20 rounded-xl">
                    <div className="flex items-start space-x-4">
                        <svg className="w-6 h-6 text-blue-400 flex-shrink-0 mt-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <div>
                            <h4 className="text-lg font-semibold text-blue-400 mb-2">Research Prototype</h4>
                            <p className="text-sm text-blue-400/80">
                                This is a demonstration of the Universal Privacy Engine built for the Oasis ROSE Bloom Grant.
                                The application showcases how institutional data can be verified and stored privately on the blockchain
                                using Oasis Sapphire's Confidential EVM technology.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </Layout>
    );
}

export default App;
