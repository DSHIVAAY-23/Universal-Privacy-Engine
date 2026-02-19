import axios, { AxiosInstance } from 'axios';
import type { STLOPProof, GenerateProofRequest, HealthResponse } from '../types';

/**
 * Notary API Client
 * Communicates with the Rust Notary Service to generate STLOP proofs.
 *
 * Works with both local (http://localhost:3002) and Ngrok tunnel URLs.
 * The `ngrok-skip-browser-warning` header is injected on every request so
 * Ngrok's free-tier HTML interstitial page is bypassed and raw JSON is returned.
 */
class NotaryAPIClient {
    private client: AxiosInstance;
    private baseURL: string;

    constructor(baseURL?: string) {
        this.baseURL =
            baseURL ||
            import.meta.env.VITE_NOTARY_API_URL ||
            'http://localhost:3002';

        this.client = axios.create({
            baseURL: this.baseURL,
            // Slightly longer timeout to account for Ngrok tunnel latency
            timeout: 15000,
            headers: {
                'Content-Type': 'application/json',
                // Bypasses the Ngrok free-tier browser warning page.
                // Without this header, Ngrok returns an HTML page instead of JSON.
                'ngrok-skip-browser-warning': '69420',
            },
        });

        console.log('🔗 Notary API Client initialized:', this.baseURL);
    }

    /**
     * Generate STLOP proof for an employee.
     * @throws Error if the Notary API is unavailable or returns an error.
     */
    async generateProof(request: GenerateProofRequest): Promise<STLOPProof> {
        try {
            console.log('📤 Requesting proof from Rust Notary API...');
            const response = await this.client.post<STLOPProof>(
                '/api/generate-proof',
                request,
            );
            console.log('✅ Proof received from Rust Notary');
            return response.data;
        } catch (error) {
            if (axios.isAxiosError(error)) {
                const message =
                    error.response?.data?.error || error.message;
                console.error('❌ Notary API Error:', message);
                throw new Error(`Notary API Error: ${message}`);
            }
            throw error;
        }
    }

    /** Check Notary service health. */
    async checkHealth(): Promise<HealthResponse> {
        try {
            const response = await this.client.get<HealthResponse>(
                '/api/health',
            );
            return response.data;
        } catch (error) {
            if (axios.isAxiosError(error)) {
                throw new Error(
                    `Notary service unavailable: ${error.message}`,
                );
            }
            throw error;
        }
    }
}

// Export singleton instance
export const notaryAPI = new NotaryAPIClient();
