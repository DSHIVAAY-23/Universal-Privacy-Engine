use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use universal_privacy_engine_core::{PrivacyEngine, ProofReceipt, ProofType, ChainType, PrivacyEngineError};

// Mock Backend for benchmarking
//
// In a production environment with a compiled guest ELF, you would pull in 
// the actual Sp1Backend or TeeProver here (likely in their own crates).
// Since `core` cannot depend on adapters (cyclic), and we lack a compiled guest ELF in CI,
// we benchmark the interface overhead using a mock.
struct MockBenchBackend;

impl PrivacyEngine for MockBenchBackend {
    fn prove(&self, input: &[u8]) -> Result<ProofReceipt, PrivacyEngineError> {
        // Simulate work proportional to input size (e.g. hashing)
        let _serialized_input = input.to_vec(); 
        
        Ok(ProofReceipt {
            proof_type: ProofType::ZkProof,
            proof: vec![0u8; 100], // Constant size proof for mock
            public_values: vec![],
            metadata: b"mock_bench".to_vec(),
        })
    }
    
    fn verify(&self, _receipt: &ProofReceipt) -> Result<bool, PrivacyEngineError> {
        Ok(true)
    }

    fn export_verifier(&self, _chain: ChainType) -> Result<Vec<u8>, PrivacyEngineError> {
        Ok(vec![])
    }
}

fn bench_proof_generation(c: &mut Criterion) {
    let backend = MockBenchBackend;
    let mut group = c.benchmark_group("proof_generation");

    // Benchmark 3 input sizes as requested
    for size in [1024, 10_240, 102_400].iter() {
        let input = vec![0u8; *size];
        
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("mock_sp1", size), size, |b, &s| {
            b.iter(|| backend.prove(black_box(&vec![0u8; s])))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_proof_generation);
criterion_main!(benches);
