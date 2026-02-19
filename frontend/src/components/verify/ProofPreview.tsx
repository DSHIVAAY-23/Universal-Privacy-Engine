import { motion } from 'framer-motion';
import { Card } from '../ui/Card';
import type { STLOPProof } from '../../types';

interface ProofPreviewProps {
    proof: STLOPProof;
    onSubmit: () => void;
    onCancel: () => void;
    isSubmitting: boolean;
}

export function ProofPreview({ proof, onSubmit, onCancel, isSubmitting }: ProofPreviewProps) {
    const formatSalary = (salary: string) => {
        return new Intl.NumberFormat('en-US', {
            style: 'currency',
            currency: 'USD',
            minimumFractionDigits: 0,
        }).format(Number(salary));
    };

    const formatTimestamp = (timestamp: number) => {
        return new Date(timestamp * 1000).toLocaleString('en-US', {
            month: 'short',
            day: 'numeric',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit',
        });
    };

    const truncateAddress = (address: string) => {
        return `${address.slice(0, 6)}...${address.slice(-4)}`;
    };

    return (
        <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
        >
            <Card variant="gradient" className="max-w-2xl mx-auto">
                <div className="text-center mb-6">
                    <div className="inline-flex items-center justify-center w-16 h-16 bg-sapphire-500/10 rounded-full mb-4">
                        <svg className="w-8 h-8 text-sapphire-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>
                    <h2 className="text-2xl font-bold text-white mb-2">📄 Salary Proof Ready</h2>
                    <p className="text-gray-400">Review your proof before submitting to the blockchain</p>
                </div>

                <div className="space-y-4 mb-6">
                    {/* Salary Amount */}
                    <div className="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
                        <p className="text-sm text-gray-400 mb-1">Verified Salary</p>
                        <p className="text-3xl font-bold text-white">{formatSalary(proof.salary)}</p>
                    </div>

                    {/* Proof Details */}
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div className="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
                            <p className="text-sm text-gray-400 mb-1">Verified By</p>
                            <p className="text-sm font-mono text-white">{truncateAddress(proof.notary_pubkey)}</p>
                        </div>
                        <div className="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
                            <p className="text-sm text-gray-400 mb-1">Timestamp</p>
                            <p className="text-sm text-white">{formatTimestamp(proof.timestamp)}</p>
                        </div>
                    </div>

                    {/* Signature */}
                    <div className="bg-gray-800/50 rounded-lg p-4 border border-gray-700">
                        <p className="text-sm text-gray-400 mb-1">Cryptographic Signature</p>
                        <p className="text-xs font-mono text-gray-500 break-all">{proof.signature}</p>
                    </div>
                </div>

                {/* Actions */}
                <div className="flex space-x-4">
                    <button
                        onClick={onCancel}
                        disabled={isSubmitting}
                        className="flex-1 px-6 py-3 bg-gray-800 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors disabled:opacity-50"
                    >
                        Cancel
                    </button>
                    <button
                        onClick={onSubmit}
                        disabled={isSubmitting}
                        className="flex-1 px-6 py-3 bg-gradient-to-r from-sapphire-600 to-primary-600 hover:from-sapphire-700 hover:to-primary-700 text-white rounded-lg font-medium shadow-lg transition-all disabled:opacity-50"
                    >
                        {isSubmitting ? (
                            <span className="flex items-center justify-center">
                                <svg className="animate-spin -ml-1 mr-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                                Submitting...
                            </span>
                        ) : (
                            'Submit to Blockchain'
                        )}
                    </button>
                </div>

                {/* Info Banner */}
                <div className="mt-6 p-4 bg-blue-500/10 border border-blue-500/20 rounded-lg">
                    <p className="text-sm text-blue-400">
                        <span className="font-semibold">🔒 Privacy Guarantee:</span> Your salary will be encrypted on-chain using Oasis Sapphire's TEE technology. Only you can decrypt and view this value.
                    </p>
                </div>
            </Card>
        </motion.div>
    );
}
