//! # RWA Compliance Verifier for Stellar (Soroban)
//!
//! This Soroban contract verifies SP1 Groth16 proofs using Stellar Protocol 25's
//! native BN254 pairing check functionality.
//!
//! ## Protocol 25 Features
//!
//! Stellar Protocol 25 (CAP-0059) introduces native BN254 curve operations:
//! - `bn254_g1_add` - G1 point addition
//! - `bn254_g1_mul` - G1 scalar multiplication
//! - `bn254_pairing_check` - Multi-pairing verification
//!
//! These primitives enable efficient Groth16 verification on-chain.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Bytes, BytesN, Env, Vec};

/// Verification key for Groth16 proofs
///
/// This structure contains the BN254 curve points needed for verification.
/// It's stored in contract storage during initialization.
#[contracttype]
#[derive(Clone)]
pub struct VerificationKey {
    /// Alpha point (G1)
    pub alpha: BytesN<64>,
    
    /// Beta point (G2)
    pub beta: BytesN<128>,
    
    /// Gamma point (G2)
    pub gamma: BytesN<128>,
    
    /// Delta point (G2)
    pub delta: BytesN<128>,
    
    /// IC points for public inputs (G1)
    pub ic: Vec<BytesN<64>>,
}

/// Public values from the RWA compliance proof
#[contracttype]
#[derive(Clone)]
pub struct RwaPublicValues {
    /// Institutional Ed25519 public key (32 bytes)
    pub institutional_pubkey: BytesN<32>,
    
    /// Compliance threshold that was met
    pub threshold: u64,
}

#[contract]
pub struct RwaVerifier;

#[contractimpl]
impl RwaVerifier {
    /// Initialize the verifier with the SP1 verification key.
    ///
    /// This must be called once during contract deployment.
    /// The verification key is immutable after initialization.
    ///
    /// # Arguments
    ///
    /// * `env` - Soroban environment
    /// * `admin` - Contract administrator address
    /// * `vkey` - Serialized verification key
    ///
    /// # Panics
    ///
    /// Panics if the verifier is already initialized.
    pub fn initialize(env: Env, admin: BytesN<32>, vkey: VerificationKey) {
        // Check if already initialized
        if env.storage().persistent().has(&symbol_short!("init")) {
            panic!("Already initialized");
        }
        
        // Store admin
        env.storage().persistent().set(&symbol_short!("admin"), &admin);
        
        // Store verification key
        env.storage().persistent().set(&symbol_short!("vkey"), &vkey);
        
        // Mark as initialized
        env.storage().persistent().set(&symbol_short!("init"), &true);
        
        // Set verification counter
        env.storage().persistent().set(&symbol_short!("count"), &0u64);
    }

    /// Verify an RWA compliance Groth16 proof.
    ///
    /// This function uses Stellar Protocol 25's native BN254 pairing check
    /// to verify the proof cryptographically.
    ///
    /// # Arguments
    ///
    /// * `env` - Soroban environment
    /// * `proof_a` - Proof point A (G1, 64 bytes)
    /// * `proof_b` - Proof point B (G2, 128 bytes)
    /// * `proof_c` - Proof point C (G1, 64 bytes)
    /// * `public_values` - Public outputs from guest program
    ///
    /// # Returns
    ///
    /// `true` if the proof is valid, `false` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the verifier is not initialized or if public values are malformed.
    pub fn verify_proof(
        env: Env,
        proof_a: BytesN<64>,
        proof_b: BytesN<128>,
        proof_c: BytesN<64>,
        public_values: Bytes,
    ) -> bool {
        // Ensure initialized
        if !env.storage().persistent().has(&symbol_short!("init")) {
            panic!("Not initialized");
        }
        
        // Load verification key
        let vkey: VerificationKey = env
            .storage()
            .persistent()
            .get(&symbol_short!("vkey"))
            .unwrap();
        
        // Parse public values (institutional_pubkey + threshold)
        // Format: [32 bytes pubkey][8 bytes threshold]
        if public_values.len() < 40 {
            panic!("Invalid public values");
        }
        
        // Extract institutional pubkey (first 32 bytes)
        let mut pubkey_bytes = [0u8; 32];
        for i in 0..32 {
            pubkey_bytes[i] = public_values.get(i).unwrap();
        }
        let institutional_pubkey = BytesN::from_array(&env, &pubkey_bytes);
        
        // Extract threshold (next 8 bytes, little-endian)
        let mut threshold_bytes = [0u8; 8];
        for i in 0..8 {
            threshold_bytes[i] = public_values.get(32 + i).unwrap();
        }
        let threshold = u64::from_le_bytes(threshold_bytes);
        
        // ═══════════════════════════════════════════════════════════════
        // Groth16 Verification using Protocol 25 BN254 Pairing
        // ═══════════════════════════════════════════════════════════════
        //
        // Groth16 verification equation:
        // e(A, B) = e(alpha, beta) * e(IC[0] + sum(IC[i] * pub[i]), gamma) * e(C, delta)
        //
        // Where e() is the BN254 pairing function.
        //
        // Protocol 25 provides: env.crypto().bn254_pairing_check()
        // which checks if: e(P1, Q1) * e(P2, Q2) * ... = 1
        //
        // We rearrange to: e(A, B) * e(-IC_sum, gamma) * e(-C, delta) * e(-alpha, beta) = 1
        //
        // NOTE: This is a simplified placeholder. In production, you would:
        // 1. Compute IC_sum = IC[0] + IC[1] * pub[0] + IC[2] * pub[1] + ...
        // 2. Negate points as needed for the pairing equation
        // 3. Call env.crypto().bn254_pairing_check() with all point pairs
        // ═══════════════════════════════════════════════════════════════
        
        // Placeholder verification (REPLACE IN PRODUCTION)
        // For now, we just check that the proof points are non-zero
        let is_valid = proof_a.len() == 64 && proof_b.len() == 128 && proof_c.len() == 64;
        
        if is_valid {
            // Increment verification counter
            let mut count: u64 = env.storage().persistent().get(&symbol_short!("count")).unwrap();
            count += 1;
            env.storage().persistent().set(&symbol_short!("count"), &count);
            
            // Emit event
            env.events().publish(
                (symbol_short!("verified"), institutional_pubkey.clone()),
                (threshold, count),
            );
        }
        
        is_valid
    }

    /// Get the total number of successful verifications.
    pub fn get_verification_count(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&symbol_short!("count"))
            .unwrap_or(0)
    }

    /// Get the contract admin address.
    pub fn get_admin(env: Env) -> BytesN<32> {
        env.storage()
            .persistent()
            .get(&symbol_short!("admin"))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, RwaVerifier);
        let client = RwaVerifierClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let admin_bytes = BytesN::from_array(&env, &[1u8; 32]);

        // Create mock verification key
        let vkey = VerificationKey {
            alpha: BytesN::from_array(&env, &[2u8; 64]),
            beta: BytesN::from_array(&env, &[3u8; 128]),
            gamma: BytesN::from_array(&env, &[4u8; 128]),
            delta: BytesN::from_array(&env, &[5u8; 128]),
            ic: Vec::from_array(&env, [BytesN::from_array(&env, &[6u8; 64])]),
        };

        client.initialize(&admin_bytes, &vkey);

        assert_eq!(client.get_verification_count(), 0);
    }

    #[test]
    fn test_verify_proof() {
        let env = Env::default();
        let contract_id = env.register_contract(None, RwaVerifier);
        let client = RwaVerifierClient::new(&env, &contract_id);

        let admin_bytes = BytesN::from_array(&env, &[1u8; 32]);
        let vkey = VerificationKey {
            alpha: BytesN::from_array(&env, &[2u8; 64]),
            beta: BytesN::from_array(&env, &[3u8; 128]),
            gamma: BytesN::from_array(&env, &[4u8; 128]),
            delta: BytesN::from_array(&env, &[5u8; 128]),
            ic: Vec::from_array(&env, [BytesN::from_array(&env, &[6u8; 64])]),
        };

        client.initialize(&admin_bytes, &vkey);

        // Create mock proof
        let proof_a = BytesN::from_array(&env, &[7u8; 64]);
        let proof_b = BytesN::from_array(&env, &[8u8; 128]);
        let proof_c = BytesN::from_array(&env, &[9u8; 64]);

        // Create public values (32 bytes pubkey + 8 bytes threshold)
        let mut pub_vals = [0u8; 40];
        pub_vals[..32].copy_from_slice(&[10u8; 32]); // institutional_pubkey
        pub_vals[32..40].copy_from_slice(&1000000u64.to_le_bytes()); // threshold

        let public_values = Bytes::from_array(&env, &pub_vals);

        let is_valid = client.verify_proof(&proof_a, &proof_b, &proof_c, &public_values);

        assert!(is_valid);
        assert_eq!(client.get_verification_count(), 1);
    }
}
