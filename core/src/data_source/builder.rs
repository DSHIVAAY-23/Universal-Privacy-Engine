//! ZK input builder for combining data with secrets

use secrecy::{Secret, ExposeSecret};
use serde::{Serialize, Deserialize};

/// Builder for constructing Privacy Engine inputs
///
/// Combines fetched public data with user secrets to create the final
/// input for zero-knowledge proof generation.
///
/// ## Security
///
/// - Secrets are wrapped in `secrecy::Secret` to prevent accidental logging
/// - Secrets are only exposed during final serialization
/// - Memory is zeroed when secrets are dropped
///
/// ## Example
///
/// ```ignore
/// use universal_privacy_engine_core::data_source::ZkInputBuilder;
///
/// let mut builder = ZkInputBuilder::new();
///
/// // Add public data from API
/// builder.add_public_data(balance_data);
///
/// // Add user secrets
/// builder.add_secret(private_key);
/// builder.add_secret(password);
///
/// // Build final input
/// let zk_input = builder.build();
///
/// // Generate proof
/// let receipt = privacy_engine.prove(&zk_input)?;
/// ```
#[derive(Default)]
pub struct ZkInputBuilder {
    /// Public data fetched from external sources
    public_data: Vec<Vec<u8>>,
    
    /// User secrets (private keys, passwords, etc.)
    secrets: Vec<Secret<Vec<u8>>>,
}

/// Combined input structure for serialization
#[derive(Serialize, Deserialize)]
struct CombinedInput {
    /// Public data fields
    public_data: Vec<Vec<u8>>,
    
    /// Secret data fields (will be private in ZK proof)
    secrets: Vec<Vec<u8>>,
}

impl ZkInputBuilder {
    /// Create a new input builder
    pub fn new() -> Self {
        Self {
            public_data: Vec::new(),
            secrets: Vec::new(),
        }
    }
    
    /// Add public data from a data provider
    ///
    /// This data will be visible in the proof's public inputs.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw bytes from data provider
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.add_public_data(balance_data);
    /// ```
    pub fn add_public_data(&mut self, data: Vec<u8>) -> &mut Self {
        self.public_data.push(data);
        self
    }
    
    /// Add a secret value
    ///
    /// This data will be kept private in the ZK proof.
    /// The secret is wrapped in `secrecy::Secret` to prevent accidental exposure.
    ///
    /// # Arguments
    ///
    /// * `secret` - Raw bytes of secret data
    ///
    /// # Example
    ///
    /// ```ignore
    /// builder.add_secret(private_key.to_vec());
    /// ```
    pub fn add_secret(&mut self, secret: Vec<u8>) -> &mut Self {
        self.secrets.push(Secret::new(secret));
        self
    }
    
    /// Add multiple public data fields at once
    ///
    /// # Arguments
    ///
    /// * `data_vec` - Vector of data fields
    pub fn add_public_data_batch(&mut self, data_vec: Vec<Vec<u8>>) -> &mut Self {
        self.public_data.extend(data_vec);
        self
    }
    
    /// Add multiple secrets at once
    ///
    /// # Arguments
    ///
    /// * `secrets_vec` - Vector of secret values
    pub fn add_secrets_batch(&mut self, secrets_vec: Vec<Vec<u8>>) -> &mut Self {
        for secret in secrets_vec {
            self.secrets.push(Secret::new(secret));
        }
        self
    }
    
    /// Build the final input for Privacy Engine
    ///
    /// Combines public data and secrets into a single serialized byte vector.
    /// The structure is:
    ///
    /// ```text
    /// [public_data_count][public_data_1][public_data_2]...[secret_count][secret_1][secret_2]...
    /// ```
    ///
    /// # Returns
    ///
    /// Serialized bytes ready for `PrivacyEngine::prove()`
    ///
    /// # Example
    ///
    /// ```ignore
    /// let zk_input = builder.build();
    /// let receipt = privacy_engine.prove(&zk_input)?;
    /// ```
    pub fn build(&self) -> Vec<u8> {
        // Extract secrets (this is the only place they're exposed)
        let secret_bytes: Vec<Vec<u8>> = self.secrets
            .iter()
            .map(|s| s.expose_secret().clone())
            .collect();
        
        let combined = CombinedInput {
            public_data: self.public_data.clone(),
            secrets: secret_bytes,
        };
        
        // Serialize to bytes using bincode
        bincode::serialize(&combined)
            .expect("Failed to serialize combined input")
    }
    
    /// Get the number of public data fields
    pub fn public_data_count(&self) -> usize {
        self.public_data.len()
    }
    
    /// Get the number of secrets
    pub fn secrets_count(&self) -> usize {
        self.secrets.len()
    }
    
    /// Clear all data and secrets
    ///
    /// Useful for reusing the builder.
    pub fn clear(&mut self) -> &mut Self {
        self.public_data.clear();
        self.secrets.clear();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builder_creation() {
        let builder = ZkInputBuilder::new();
        assert_eq!(builder.public_data_count(), 0);
        assert_eq!(builder.secrets_count(), 0);
    }
    
    #[test]
    fn test_add_public_data() {
        let mut builder = ZkInputBuilder::new();
        builder.add_public_data(vec![1, 2, 3]);
        builder.add_public_data(vec![4, 5, 6]);
        
        assert_eq!(builder.public_data_count(), 2);
    }
    
    #[test]
    fn test_add_secret() {
        let mut builder = ZkInputBuilder::new();
        builder.add_secret(vec![7, 8, 9]);
        builder.add_secret(vec![10, 11, 12]);
        
        assert_eq!(builder.secrets_count(), 2);
    }
    
    #[test]
    fn test_build() {
        let mut builder = ZkInputBuilder::new();
        builder.add_public_data(vec![1, 2, 3]);
        builder.add_secret(vec![4, 5, 6]);
        
        let input = builder.build();
        assert!(!input.is_empty());
        
        // Deserialize to verify structure
        let combined: CombinedInput = bincode::deserialize(&input).unwrap();
        assert_eq!(combined.public_data.len(), 1);
        assert_eq!(combined.secrets.len(), 1);
        assert_eq!(combined.public_data[0], vec![1, 2, 3]);
        assert_eq!(combined.secrets[0], vec![4, 5, 6]);
    }
    
    #[test]
    fn test_batch_operations() {
        let mut builder = ZkInputBuilder::new();
        
        builder.add_public_data_batch(vec![
            vec![1, 2],
            vec![3, 4],
        ]);
        
        builder.add_secrets_batch(vec![
            vec![5, 6],
            vec![7, 8],
        ]);
        
        assert_eq!(builder.public_data_count(), 2);
        assert_eq!(builder.secrets_count(), 2);
    }
    
    #[test]
    fn test_clear() {
        let mut builder = ZkInputBuilder::new();
        builder.add_public_data(vec![1, 2, 3]);
        builder.add_secret(vec![4, 5, 6]);
        
        builder.clear();
        
        assert_eq!(builder.public_data_count(), 0);
        assert_eq!(builder.secrets_count(), 0);
    }
    
    #[test]
    fn test_chaining() {
        let input = ZkInputBuilder::new()
            .add_public_data(vec![1, 2, 3])
            .add_secret(vec![4, 5, 6])
            .build();
        
        assert!(!input.is_empty());
    }
}
