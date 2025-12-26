//! # Verifier Generator
//!
//! Auto-generates Solidity smart contract verifiers for SP1 zkVM programs.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --bin generate_verifier -- \
//!   --program-vkey 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
//!   --out-dir contracts/generated
//! ```

use anyhow::{bail, Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context as TeraContext, Tera};

/// CLI arguments for verifier generator
#[derive(Parser, Debug)]
#[command(name = "generate_verifier")]
#[command(about = "Generate Solidity verifier contracts for SP1 zkVM programs")]
struct Args {
    /// Program verification key (hex string, 32 bytes)
    ///
    /// This is the hash of the compiled guest program ELF.
    /// Can be provided with or without '0x' prefix.
    ///
    /// Example: 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
    #[arg(long, value_name = "VKEY")]
    program_vkey: String,
    
    /// Output directory for generated contract
    ///
    /// The generated UniversalVerifier.sol will be written to this directory.
    #[arg(long, default_value = "contracts/generated", value_name = "DIR")]
    out_dir: PathBuf,
    
    /// Overwrite existing files without prompting
    #[arg(long, short = 'f')]
    force: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("═══════════════════════════════════════════════════════════");
    println!("         VeriVault Verifier Generator");
    println!("═══════════════════════════════════════════════════════════\n");
    
    // Validate and format VKey
    println!("Validating program verification key...");
    let formatted_vkey = validate_and_format_vkey(&args.program_vkey)?;
    println!("  ✓ VKey: {}", formatted_vkey);
    
    // Generate verifier
    println!("\nGenerating verifier contract...");
    let output_path = generate_verifier(&formatted_vkey, &args.out_dir, args.force)?;
    
    println!("\n═══════════════════════════════════════════════════════════");
    println!("                   GENERATION COMPLETE");
    println!("═══════════════════════════════════════════════════════════\n");
    println!("✓ Generated: {}", output_path.display());
    println!("\nNext steps:");
    println!("  1. Review the generated contract");
    println!("  2. Deploy MockSP1Verifier (for testing)");
    println!("  3. Deploy UniversalVerifier with SP1 verifier address");
    println!("  4. Verify proofs using verifyProof() function\n");
    
    Ok(())
}

/// Validate and format verification key
///
/// # Arguments
///
/// * `vkey` - Raw VKey string (with or without 0x prefix)
///
/// # Returns
///
/// Formatted VKey as `bytes32` literal (e.g., "0x1234...abcd")
///
/// # Errors
///
/// Returns error if:
/// - VKey is not 32 bytes (64 hex characters)
/// - VKey contains invalid hex characters
fn validate_and_format_vkey(vkey: &str) -> Result<String> {
    // Remove 0x prefix if present
    let vkey_clean = vkey.strip_prefix("0x").unwrap_or(vkey);
    
    // Validate length (32 bytes = 64 hex chars)
    if vkey_clean.len() != 64 {
        bail!(
            "Invalid VKey length: expected 64 hex characters (32 bytes), got {}",
            vkey_clean.len()
        );
    }
    
    // Validate hex characters
    hex::decode(vkey_clean)
        .context("VKey contains invalid hex characters")?;
    
    // Return formatted as bytes32 literal
    Ok(format!("0x{}", vkey_clean))
}

/// Generate Solidity verifier contract
///
/// # Arguments
///
/// * `vkey` - Formatted verification key (0x-prefixed hex)
/// * `out_dir` - Output directory path
/// * `force` - Overwrite existing files without prompting
///
/// # Returns
///
/// Path to the generated contract file
fn generate_verifier(vkey: &str, out_dir: &Path, force: bool) -> Result<PathBuf> {
    // Create output directory
    fs::create_dir_all(out_dir)
        .context(format!("Failed to create output directory: {}", out_dir.display()))?;
    
    // Check if output file exists
    let output_path = out_dir.join("UniversalVerifier.sol");
    if output_path.exists() && !force {
        bail!(
            "Output file already exists: {}\nUse --force to overwrite",
            output_path.display()
        );
    }
    
    // Load template
    let template_content = include_str!("../../../contracts/templates/UniversalVerifier.sol.tera");
    
    // Create Tera instance
    let mut tera = Tera::default();
    tera.add_raw_template("verifier", template_content)
        .context("Failed to parse template")?;
    
    // Create template context
    let mut context = TeraContext::new();
    context.insert("program_vkey", vkey);
    
    // Render template
    let rendered = tera.render("verifier", &context)
        .context("Failed to render template")?;
    
    // Write to file
    fs::write(&output_path, rendered)
        .context(format!("Failed to write to {}", output_path.display()))?;
    
    println!("  ✓ Contract written to: {}", output_path.display());
    
    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_vkey_valid() {
        let vkey = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let result = validate_and_format_vkey(vkey);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vkey);
    }
    
    #[test]
    fn test_validate_vkey_without_prefix() {
        let vkey = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let result = validate_and_format_vkey(vkey);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("0x{}", vkey));
    }
    
    #[test]
    fn test_validate_vkey_invalid_length() {
        let vkey = "0x1234"; // Too short
        let result = validate_and_format_vkey(vkey);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_vkey_invalid_hex() {
        let vkey = "0xGGGG567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let result = validate_and_format_vkey(vkey);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_generate_verifier() {
        let vkey = "0x0000000000000000000000000000000000000000000000000000000000000001";
        let temp_dir = std::env::temp_dir().join("verifier_test");
        
        let result = generate_verifier(vkey, &temp_dir, true);
        assert!(result.is_ok());
        
        let output_path = result.unwrap();
        assert!(output_path.exists());
        
        // Read generated file
        let content = fs::read_to_string(&output_path).unwrap();
        
        // Verify VKey is in the file
        assert!(content.contains(vkey));
        assert!(content.contains("UniversalVerifier"));
        assert!(content.contains("verifyProof"));
        
        // Cleanup
        fs::remove_dir_all(temp_dir).ok();
    }
}
