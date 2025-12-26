//! Data provider trait definition

use async_trait::async_trait;
use super::error::DataError;

/// Trait for fetching data from external sources
///
/// This trait abstracts over different data sources (HTTP, IPFS, blockchain oracles, etc.)
/// and provides a unified interface for data ingestion into the Privacy Engine.
///
/// ## Future: zkTLS Integration
///
/// Eventually, implementations will be secured by zkTLS (TLSNotary/DECO) to provide
/// cryptographic proof of data authenticity without revealing the full TLS session.
///
/// ## Example
///
/// ```ignore
/// use universal_privacy_engine_core::data_source::{DataProvider, HttpProvider};
///
/// #[tokio::main]
/// async fn main() {
///     let provider = HttpProvider::new();
///     let balance = provider.fetch(
///         "https://api.bank.com/account/123",
///         "data.balance"
///     ).await.unwrap();
/// }
/// ```
#[async_trait]
pub trait DataProvider: Send + Sync {
    /// Fetch data from a source with an optional query
    ///
    /// # Arguments
    ///
    /// * `source` - URL or identifier for the data source (e.g., "https://api.example.com/data")
    /// * `query` - Query string to extract specific fields (e.g., "account.balance", "data[0].value")
    ///
    /// # Returns
    ///
    /// Raw bytes of the fetched and extracted data
    ///
    /// # Errors
    ///
    /// Returns `DataError` if:
    /// - HTTP request fails
    /// - JSON parsing fails
    /// - Requested field is not found
    /// - Network error occurs
    ///
    /// # Example Query Syntax
    ///
    /// - `"balance"` - Top-level field
    /// - `"account.balance"` - Nested field
    /// - `"items[0]"` - Array indexing
    /// - `"data.users[2].name"` - Complex path
    async fn fetch(&self, source: &str, query: &str) -> Result<Vec<u8>, DataError>;
    
    /// Verify a recorded TLS proof
    ///
    /// Validates that the provided proof matches the expected domain, certificate chain,
    /// and response integrity. This is the "Recorded zkTLS" replacement for the previous stub.
    ///
    /// # Arguments
    ///
    /// * `proof` - The recorded proof to verify
    ///
    /// # Returns
    ///
    /// `Ok(())` if verification succeeds, `Err(ZkTlsError)` otherwise
    fn verify_tls_proof(&self, proof: &super::zktls::RecordedTlsProof) -> Result<(), super::zktls::ZkTlsError>;
}
