//! # VeriVault Performance Benchmarking Suite
//!
//! This binary measures the performance of `PrivacyEngine` implementations across
//! different workload scenarios to generate concrete data for grant proposals.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --release --bin benchmark
//! ```
//!
//! ## Output
//!
//! Results are saved to `benchmarks.json` in the workspace root.

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Instant;
use sysinfo::System;
use universal_privacy_engine_core::{PrivacyEngine, ProofReceipt};
use universal_privacy_engine_tee::TeeProverStub;

/// Benchmark result for a single scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchmarkResult {
    /// Scenario name (Small, Medium, Large)
    scenario: String,
    
    /// Prover backend (SP1, TEE)
    prover: String,
    
    /// Proving mode (Mock, Stark, Groth16 for SP1; Mock for TEE)
    mode: String,
    
    /// Wall clock time in milliseconds
    time_ms: u64,
    
    /// CPU cycles (SP1 only, null for TEE)
    cycles: Option<u64>,
    
    /// RAM usage in megabytes
    ram_mb: u64,
    
    /// Timestamp of benchmark execution
    timestamp: String,
    
    /// Input size in bytes
    input_size_bytes: usize,
    
    /// Whether the output was verified successfully
    output_verified: bool,
}

/// Benchmark scenario configuration
struct Scenario {
    name: String,
    input_data: Vec<u8>,
    description: String,
}

impl Scenario {
    /// Create a small workload scenario (256 bytes)
    fn small() -> Self {
        Self {
            name: "Small".to_string(),
            input_data: vec![0x42; 256],
            description: "Simple 256-byte input (baseline)".to_string(),
        }
    }
    
    /// Create a medium workload scenario (1 KB)
    fn medium() -> Self {
        Self {
            name: "Medium".to_string(),
            input_data: vec![0x42; 1024],
            description: "1KB input (realistic data structure)".to_string(),
        }
    }
    
    /// Create a large workload scenario (5 KB)
    fn large() -> Self {
        Self {
            name: "Large".to_string(),
            input_data: vec![0x42; 5120],
            description: "5KB input (high-throughput workload)".to_string(),
        }
    }
}

/// Benchmark a single scenario with a given backend
fn benchmark_scenario(
    scenario: &Scenario,
    backend: &dyn PrivacyEngine,
    backend_name: &str,
    mode: &str,
) -> Result<BenchmarkResult> {
    println!(
        "  Running {} scenario with {} ({})...",
        scenario.name, backend_name, mode
    );
    
    // Get initial memory usage
    let mut sys = System::new_all();
    sys.refresh_all();
    let initial_memory = sys.used_memory();
    
    // Measure wall time
    let start = Instant::now();
    
    // Generate proof
    let receipt = backend
        .prove(&scenario.input_data)
        .context("Proof generation failed")?;
    
    let elapsed = start.elapsed();
    
    // Verify proof
    let is_valid = backend
        .verify(&receipt)
        .context("Proof verification failed")?;
    
    // Get final memory usage
    sys.refresh_all();
    let final_memory = sys.used_memory();
    let memory_used = (final_memory.saturating_sub(initial_memory)) / 1024 / 1024; // Convert to MB
    
    // Extract cycles from metadata (SP1 only)
    let cycles = extract_cycles_from_receipt(&receipt);
    
    Ok(BenchmarkResult {
        scenario: scenario.name.clone(),
        prover: backend_name.to_string(),
        mode: mode.to_string(),
        time_ms: elapsed.as_millis() as u64,
        cycles,
        ram_mb: memory_used,
        timestamp: Utc::now().to_rfc3339(),
        input_size_bytes: scenario.input_data.len(),
        output_verified: is_valid,
    })
}

/// Extract cycle count from proof receipt metadata (SP1 only)
fn extract_cycles_from_receipt(receipt: &ProofReceipt) -> Option<u64> {
    // Try to parse metadata as JSON and extract cycles
    if let Ok(metadata_str) = String::from_utf8(receipt.metadata.clone()) {
        if let Ok(metadata) = serde_json::from_str::<serde_json::Value>(&metadata_str) {
            if let Some(cycles) = metadata.get("cycles") {
                return cycles.as_u64();
            }
        }
    }
    None
}

/// Run all scenarios with a given backend
fn run_all_scenarios(
    backend: &dyn PrivacyEngine,
    backend_name: &str,
    mode: &str,
) -> Result<Vec<BenchmarkResult>> {
    let scenarios = vec![
        Scenario::small(),
        Scenario::medium(),
        Scenario::large(),
    ];
    
    let mut results = Vec::new();
    
    for scenario in scenarios {
        match benchmark_scenario(&scenario, backend, backend_name, mode) {
            Ok(result) => {
                println!(
                    "    ✓ Completed in {}ms (RAM: {}MB, Verified: {})",
                    result.time_ms, result.ram_mb, result.output_verified
                );
                results.push(result);
            }
            Err(e) => {
                eprintln!("    ✗ Failed: {}", e);
                // Continue with other scenarios
            }
        }
    }
    
    Ok(results)
}

/// Save benchmark results to JSON file
fn save_results(results: &[BenchmarkResult], output_path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(results)
        .context("Failed to serialize results to JSON")?;
    
    fs::write(output_path, json)
        .context(format!("Failed to write results to {}", output_path))?;
    
    println!("\n✓ Results saved to {}", output_path);
    Ok(())
}

/// Print summary statistics
fn print_summary(results: &[BenchmarkResult]) {
    println!("\n═══════════════════════════════════════════════════════════");
    println!("                   BENCHMARK SUMMARY");
    println!("═══════════════════════════════════════════════════════════\n");
    
    // Group by prover
    let mut tee_results = Vec::new();
    let mut sp1_results = Vec::new();
    
    for result in results {
        if result.prover == "TEE" {
            tee_results.push(result);
        } else {
            sp1_results.push(result);
        }
    }
    
    // Print TEE results
    if !tee_results.is_empty() {
        println!("TEE Backend (Mock):");
        println!("┌─────────────┬──────────────┬──────────┬──────────────┐");
        println!("│ Scenario    │ Time (ms)    │ RAM (MB) │ Verified     │");
        println!("├─────────────┼──────────────┼──────────┼──────────────┤");
        for result in &tee_results {
            println!(
                "│ {:<11} │ {:>12} │ {:>8} │ {:<12} │",
                result.scenario,
                result.time_ms,
                result.ram_mb,
                if result.output_verified { "✓" } else { "✗" }
            );
        }
        println!("└─────────────┴──────────────┴──────────┴──────────────┘\n");
    }
    
    // Print SP1 results
    if !sp1_results.is_empty() {
        println!("SP1 Backend (Mock):");
        println!("┌─────────────┬──────────────┬──────────┬──────────────┐");
        println!("│ Scenario    │ Time (ms)    │ RAM (MB) │ Verified     │");
        println!("├─────────────┼──────────────┼──────────┼──────────────┤");
        for result in &sp1_results {
            println!(
                "│ {:<11} │ {:>12} │ {:>8} │ {:<12} │",
                result.scenario,
                result.time_ms,
                result.ram_mb,
                if result.output_verified { "✓" } else { "✗" }
            );
        }
        println!("└─────────────┴──────────────┴──────────┴──────────────┘\n");
    }
    
    // Print totals
    let total_time: u64 = results.iter().map(|r| r.time_ms).sum();
    let avg_time = if !results.is_empty() {
        total_time / results.len() as u64
    } else {
        0
    };
    
    println!("Total benchmarks run: {}", results.len());
    println!("Total time: {}ms", total_time);
    println!("Average time: {}ms", avg_time);
    println!("All verified: {}", results.iter().all(|r| r.output_verified));
}

fn main() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("         VeriVault Performance Benchmark Suite");
    println!("═══════════════════════════════════════════════════════════\n");
    
    let mut all_results = Vec::new();
    
    // ═══════════════════════════════════════════════════════════════
    // Benchmark TEE Backend
    // ═══════════════════════════════════════════════════════════════
    println!("Benchmarking TEE Backend (Mock)...");
    let tee_backend = TeeProverStub::new();
    let tee_results = run_all_scenarios(&tee_backend, "TEE", "Mock")?;
    all_results.extend(tee_results);
    
    // ═══════════════════════════════════════════════════════════════
    // Benchmark SP1 Backend (if guest ELF is available)
    // ═══════════════════════════════════════════════════════════════
    // Note: SP1 benchmarking requires a compiled guest program ELF.
    // For now, we skip SP1 benchmarking to avoid build dependencies.
    // Uncomment the following code when guest ELF is available:
    
    /*
    println!("\nBenchmarking SP1 Backend (Mock)...");
    
    // Load guest ELF
    let elf_path = "guest/rwa_compliance/elf/riscv32im-succinct-zkvm-elf";
    if std::path::Path::new(elf_path).exists() {
        let elf = std::fs::read(elf_path)
            .context("Failed to read guest ELF")?;
        
        let sp1_backend = universal_privacy_engine_sp1::Sp1Backend::new(elf);
        let sp1_results = run_all_scenarios(&sp1_backend, "SP1", "Mock")?;
        all_results.extend(sp1_results);
    } else {
        println!("  ⚠ Guest ELF not found at {}. Skipping SP1 benchmarks.", elf_path);
        println!("  Run 'cd guest/rwa_compliance && cargo prove build' to build the guest program.");
    }
    */
    
    println!("\n  ⚠ SP1 benchmarks skipped (requires compiled guest ELF)");
    println!("  To enable: cd guest/rwa_compliance && cargo prove build\n");
    
    // ═══════════════════════════════════════════════════════════════
    // Save Results
    // ═══════════════════════════════════════════════════════════════
    let output_path = "benchmarks.json";
    save_results(&all_results, output_path)?;
    
    // ═══════════════════════════════════════════════════════════════
    // Print Summary
    // ═══════════════════════════════════════════════════════════════
    print_summary(&all_results);
    
    println!("\n═══════════════════════════════════════════════════════════");
    println!("                    BENCHMARK COMPLETE");
    println!("═══════════════════════════════════════════════════════════\n");
    
    Ok(())
}
