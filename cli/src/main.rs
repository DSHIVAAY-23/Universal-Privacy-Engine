//! # Universal Privacy Engine CLI
//!
//! Command-line interface for generating, verifying, and exporting ZK proofs.
//!
//! ## Usage Examples
//!
//! ```bash
//! # Generate a proof from hex-encoded input
//! upe prove --input "48656c6c6f20576f726c64" --output proof.bin
//!
//! # Verify a proof receipt
//! upe verify --receipt proof.bin
//!
//! # Export a verifier for Solana
//! upe export-verifier --chain solana --output verifier.so
//! ```

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use universal_privacy_engine_core::{ChainType, PrivacyEngine, ProofReceipt};
use universal_privacy_engine_sp1::Sp1Backend;

mod mcp;
use mcp::VeriVaultMcpServer;

/// Universal Privacy Engine - Chain-agnostic ZK proving CLI
#[derive(Parser)]
#[command(name = "upe")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a zero-knowledge proof from input data
    Prove {
        /// Hex-encoded input data (private witness)
        #[arg(short, long)]
        input: String,

        /// Path to the guest ELF binary
        #[arg(short, long, default_value = "guest.elf")]
        elf: PathBuf,

        /// Output file for the proof receipt
        #[arg(short, long, default_value = "proof.bin")]
        output: PathBuf,
    },

    /// Verify a proof receipt
    Verify {
        /// Path to the proof receipt file
        #[arg(short, long)]
        receipt: PathBuf,

        /// Path to the guest ELF binary
        #[arg(short, long, default_value = "guest.elf")]
        elf: PathBuf,
    },

    /// Export a verifier for a specific blockchain
    ExportVerifier {
        /// Target blockchain platform
        #[arg(short, long, value_enum)]
        chain: ChainTypeArg,

        /// Path to the guest ELF binary
        #[arg(short, long, default_value = "guest.elf")]
        elf: PathBuf,

        /// Output file for the verifier bytecode
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Start MCP server for Cursor/Claude integration
    McpServer,
}

/// CLI-friendly version of ChainType with clap integration
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum ChainTypeArg {
    Solana,
    Stellar,
    Evm,
}

impl From<ChainTypeArg> for ChainType {
    fn from(arg: ChainTypeArg) -> Self {
        match arg {
            ChainTypeArg::Solana => ChainType::Solana,
            ChainTypeArg::Stellar => ChainType::Stellar,
            ChainTypeArg::Evm => ChainType::Evm,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Prove { input, elf, output } => {
            prove_command(input, elf, output).await?;
        }
        Commands::Verify { receipt, elf } => {
            verify_command(receipt, elf).await?;
        }
        Commands::ExportVerifier { chain, elf, output } => {
            export_verifier_command(chain, elf, output).await?;
        }
        Commands::McpServer => {
            mcp_server_command()?;
        }
    }

    Ok(())
}

/// Execute the prove command
async fn prove_command(input_hex: String, elf_path: PathBuf, output_path: PathBuf) -> Result<()> {
    println!("🔐 Universal Privacy Engine - Proof Generation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Decode hex input
    let input_bytes = hex::decode(&input_hex)
        .context("Failed to decode hex input. Ensure input is valid hexadecimal.")?;
    
    println!("✓ Decoded input: {} bytes", input_bytes.len());

    // Load the guest ELF
    let elf_bytes = std::fs::read(&elf_path)
        .with_context(|| format!("Failed to read ELF file: {}", elf_path.display()))?;
    
    println!("✓ Loaded guest ELF: {} bytes", elf_bytes.len());

    // Initialize the SP1 backend
    let backend = Sp1Backend::new(elf_bytes);
    println!("✓ Initialized SP1 backend");

    // Generate the proof
    println!("\n⏳ Generating proof... (this may take a while)");
    let receipt = backend
        .prove(&input_bytes)
        .context("Proof generation failed")?;
    
    println!("✓ Proof generated successfully!");
    println!("  - Proof size: {} bytes", receipt.proof.len());
    println!("  - Public values: {} bytes", receipt.public_values.len());

    // Serialize and save the receipt
    let receipt_bytes = bincode::serialize(&receipt)
        .context("Failed to serialize proof receipt")?;
    
    std::fs::write(&output_path, receipt_bytes)
        .with_context(|| format!("Failed to write receipt to {}", output_path.display()))?;
    
    println!("\n✅ Proof receipt saved to: {}", output_path.display());
    
    Ok(())
}

/// Execute the verify command
async fn verify_command(receipt_path: PathBuf, elf_path: PathBuf) -> Result<()> {
    println!("🔍 Universal Privacy Engine - Proof Verification");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Load the receipt
    let receipt_bytes = std::fs::read(&receipt_path)
        .with_context(|| format!("Failed to read receipt file: {}", receipt_path.display()))?;
    
    let receipt: ProofReceipt = bincode::deserialize(&receipt_bytes)
        .context("Failed to deserialize proof receipt")?;
    
    println!("✓ Loaded proof receipt: {} bytes", receipt_bytes.len());

    // Load the guest ELF
    let elf_bytes = std::fs::read(&elf_path)
        .with_context(|| format!("Failed to read ELF file: {}", elf_path.display()))?;
    
    println!("✓ Loaded guest ELF: {} bytes", elf_bytes.len());

    // Initialize the SP1 backend
    let backend = Sp1Backend::new(elf_bytes);
    println!("✓ Initialized SP1 backend");

    // Verify the proof
    println!("\n⏳ Verifying proof...");
    let is_valid = backend
        .verify(&receipt)
        .context("Proof verification failed")?;

    if is_valid {
        println!("\n✅ PROOF VALID - Verification successful!");
        println!("  - Public values: {} bytes", receipt.public_values.len());
        if !receipt.public_values.is_empty() {
            println!("  - Public values (hex): {}", hex::encode(&receipt.public_values));
        }
    } else {
        println!("\n❌ PROOF INVALID - Verification failed!");
        anyhow::bail!("Proof verification returned false");
    }

    Ok(())
}

/// Execute the export-verifier command
async fn export_verifier_command(
    chain: ChainTypeArg,
    elf_path: PathBuf,
    output_path: PathBuf,
) -> Result<()> {
    println!("📦 Universal Privacy Engine - Verifier Export");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let chain_type: ChainType = chain.into();
    println!("Target chain: {:?}", chain_type);

    // Load the guest ELF
    let elf_bytes = std::fs::read(&elf_path)
        .with_context(|| format!("Failed to read ELF file: {}", elf_path.display()))?;
    
    println!("✓ Loaded guest ELF: {} bytes", elf_bytes.len());

    // Initialize the SP1 backend
    let backend = Sp1Backend::new(elf_bytes);
    println!("✓ Initialized SP1 backend");

    // Export the verifier
    println!("\n⏳ Generating verifier bytecode...");
    let verifier_bytes = backend
        .export_verifier(chain_type)
        .context("Verifier export failed")?;
    
    println!("✓ Verifier generated: {} bytes", verifier_bytes.len());

    // Save the verifier
    std::fs::write(&output_path, verifier_bytes)
        .with_context(|| format!("Failed to write verifier to {}", output_path.display()))?;
    
    println!("\n✅ Verifier saved to: {}", output_path.display());
    println!("\nNext steps:");
    match chain_type {
        ChainType::Solana => {
            println!("  1. Deploy to Solana: solana program deploy {}", output_path.display());
            println!("  2. Initialize the program with your verification key");
        }
        ChainType::Stellar => {
            println!("  1. Deploy to Stellar: stellar contract deploy --wasm {}", output_path.display());
            println!("  2. Initialize the contract");
        }
        ChainType::Evm => {
            println!("  1. Deploy to EVM chain using your preferred tool (Hardhat, Foundry, etc.)");
            println!("  2. Call the verify() function with your proof");
        }
    }

    Ok(())
}

/// Execute the MCP server command
fn mcp_server_command() -> Result<()> {
    println!("🚀 VeriVault MCP Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📡 Starting MCP server for Cursor/Claude integration...");
    println!();
    
    let mut server = VeriVaultMcpServer::new();
    server.run().context("MCP server failed")?;
    
    Ok(())
}
