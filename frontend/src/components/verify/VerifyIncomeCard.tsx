import { useState } from 'react';
import { useAccount } from 'wagmi';
import { motion, AnimatePresence } from 'framer-motion';
import { Card } from '../ui/Card';
import { Button } from '../ui/Button';
import { Spinner } from '../ui/Spinner';
import { ProofPreview } from './ProofPreview';
import { TransactionStatus } from './TransactionStatus';
import { useNotaryAPI } from '../../hooks/useNotaryAPI';
import { useVerifySalary } from '../../hooks/usePrivatePayroll';
import { useAppStore } from '../../store/useAppStore';
import type { VerifyStep } from '../../types';

export function VerifyIncomeCard() {
    const { address, isConnected } = useAccount();
    const [step, setStep] = useState<VerifyStep>('idle');
    const { generateProof, error: proofError } = useNotaryAPI();
    const { verifySalary, txHash, isPending, isConfirming, isSuccess, error: txError } = useVerifySalary();
    const { currentProof, setProof } = useAppStore();

    const handleVerifyIncome = async () => {
        if (!address) return;

        try {
            setStep('fetching-proof');
            const proof = await generateProof(address);

            if (proof) {
                setProof(proof);
                setStep('preview');
            } else {
                // Proof generation failed, revert to idle
                setStep('idle');
            }
        } catch (error) {
            console.error('Error in handleVerifyIncome:', error);
            setStep('idle');
        }
    };

    const handleSubmitToBlockchain = async () => {
        if (!currentProof) return;

        try {
            setStep('submitting');
            await verifySalary(currentProof);
        } catch (error) {
            console.error('Error submitting to blockchain:', error);
            setStep('preview');
        }
    };

    const handleCancel = () => {
        setProof(null);
        setStep('idle');
    };

    const handleReset = () => {
        setProof(null);
        setStep('idle');
    };

    // Update step based on transaction status
    if (isSuccess && step === 'submitting') {
        setStep('success');
    }

    if (!isConnected) {
        return (
            <Card variant="gradient" className="max-w-2xl mx-auto text-center">
                <div className="py-12">
                    <div className="inline-flex items-center justify-center w-20 h-20 bg-sapphire-500/10 rounded-full mb-6">
                        <svg className="w-10 h-10 text-sapphire-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                        </svg>
                    </div>
                    <h2 className="text-3xl font-bold text-white mb-4">Connect Your Wallet</h2>
                    <p className="text-gray-400 mb-8 max-w-md mx-auto">
                        Connect your wallet to verify your income privately on the Oasis Sapphire blockchain
                    </p>
                    <div className="inline-flex items-center space-x-2 text-sm text-gray-500">
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                        </svg>
                        <span>Encrypted by Oasis Sapphire TEE</span>
                    </div>
                </div>
            </Card>
        );
    }

    return (
        <div className="space-y-6">
            <AnimatePresence mode="wait">
                {step === 'idle' && (
                    <motion.div
                        key="idle"
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -20 }}
                    >
                        <Card variant="gradient" className="max-w-2xl mx-auto text-center">
                            <div className="py-12">
                                <div className="inline-flex items-center justify-center w-20 h-20 bg-gradient-to-br from-sapphire-500/20 to-primary-500/20 rounded-full mb-6">
                                    <svg className="w-10 h-10 text-sapphire-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                </div>
                                <h2 className="text-3xl font-bold text-white mb-4">Generate STLOP Proof</h2>
                                <p className="text-gray-400 mb-8 max-w-md mx-auto">
                                    Initialize secure TLS proof generation from your payroll data source
                                </p>

                                {proofError && (
                                    <div className="mb-6 p-4 bg-red-500/10 border border-red-500/20 rounded-lg max-w-md mx-auto">
                                        <p className="text-sm text-red-400">{proofError}</p>
                                    </div>
                                )}

                                <Button
                                    size="lg"
                                    onClick={handleVerifyIncome}
                                    className="px-8 py-4 text-lg"
                                >
                                    <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                                    </svg>
                                    Start Verification
                                </Button>

                                <div className="mt-8 grid grid-cols-1 md:grid-cols-3 gap-4 max-w-2xl mx-auto">
                                    <div className="p-4 bg-gray-800/30 rounded-lg border border-gray-700/50">
                                        <div className="text-sapphire-400 mb-2">🔒</div>
                                        <h3 className="text-sm font-semibold text-white mb-1">Encrypted State</h3>
                                        <p className="text-xs text-gray-400">Data stored in Sapphire TEE</p>
                                    </div>
                                    <div className="p-4 bg-gray-800/30 rounded-lg border border-gray-700/50">
                                        <div className="text-sapphire-400 mb-2">✍️</div>
                                        <h3 className="text-sm font-semibold text-white mb-1">Cryptographic Proof</h3>
                                        <p className="text-xs text-gray-400">Notary-signed STLOP</p>
                                    </div>
                                    <div className="p-4 bg-gray-800/30 rounded-lg border border-gray-700/50">
                                        <div className="text-sapphire-400 mb-2">👤</div>
                                        <h3 className="text-sm font-semibold text-white mb-1">Private Access</h3>
                                        <p className="text-xs text-gray-400">Only you can decrypt</p>
                                    </div>
                                </div>
                            </div>
                        </Card>
                    </motion.div>
                )}

                {step === 'fetching-proof' && (
                    <motion.div
                        key="fetching"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                    >
                        <Card className="max-w-2xl mx-auto text-center py-12">
                            <Spinner size="lg" />
                            <h3 className="text-xl font-bold text-white mt-6 mb-2">Fetching Your Salary Data</h3>
                            <p className="text-gray-400">Generating cryptographic proof from payroll system...</p>
                        </Card>
                    </motion.div>
                )}

                {step === 'preview' && currentProof && (
                    <ProofPreview
                        key="preview"
                        proof={currentProof}
                        onSubmit={handleSubmitToBlockchain}
                        onCancel={handleCancel}
                        isSubmitting={isPending}
                    />
                )}

                {step === 'submitting' && (isPending || isConfirming) && (
                    <TransactionStatus key="submitting" status="pending" />
                )}

                {step === 'success' && txHash && (
                    <motion.div key="success">
                        <TransactionStatus status="success" txHash={txHash} />
                        <div className="text-center mt-6">
                            <Button onClick={handleReset} variant="outline">
                                Verify Another Salary
                            </Button>
                        </div>
                    </motion.div>
                )}

                {txError && step === 'submitting' && (
                    <motion.div key="error">
                        <TransactionStatus status="error" error={txError.message} />
                        <div className="text-center mt-6">
                            <Button onClick={handleReset} variant="outline">
                                Try Again
                            </Button>
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>
        </div>
    );
}
