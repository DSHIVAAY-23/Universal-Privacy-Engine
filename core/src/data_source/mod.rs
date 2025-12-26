//! Data ingestion module for fetching external data
//!
//! This module provides abstractions for fetching data from external sources
//! (HTTP APIs, IPFS, blockchain oracles, etc.) and preparing it for zero-knowledge
//! proof generation.
//!
//! ## Architecture
//!
//! The module follows the **Ports and Adapters** pattern:
//! - `DataProvider` trait is the **Port** (interface)
//! - `HttpProvider` is an **Adapter** (implementation)
//!
//! ## Future: zkTLS Integration
//!
//! Eventually, data fetching will be secured by zkTLS (TLSNotary/DECO) to provide
//! cryptographic proof of data authenticity without revealing the full TLS session.
//!
//! ## Example Usage
//!
//! ```ignore
//! use universal_privacy_engine_core::data_source::{
//!     HttpProvider, DataProvider, ZkInputBuilder
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 1. Fetch data from API
//!     let provider = HttpProvider::new();
//!     let balance = provider.fetch(
//!         "https://api.bank.com/account/123",
//!         "data.balance"
//!     ).await?;
//!     
//!     // 2. Verify TLS (currently stub)
//!     assert!(provider.verify_tls_signature());
//!     
//!     // 3. Build ZK input
//!     let mut builder = ZkInputBuilder::new();
//!     builder.add_public_data(balance);
//!     builder.add_secret(user_private_key);
//!     
//!     let zk_input = builder.build();
//!     
//!     // 4. Generate proof
//!     let receipt = privacy_engine.prove(&zk_input)?;
//!     
//!     Ok(())
//! }
//! ```

mod error;
mod provider;
mod http;
mod builder;

pub use error::DataError;
pub use provider::DataProvider;
pub use http::HttpProvider;
pub use builder::ZkInputBuilder;
