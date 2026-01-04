# Benchmarks

This repository uses [Criterion.rs](https://bheisler.github.io/criterion.rs/) for benchmarking the privacy engine core.

## Running Benchmarks

To run the benchmarks and generate the standard Criterion report:

```bash
cargo bench -p universal-privacy-engine-core
```

## JSON Output

To generate JSON output (useful for CI/CD tracking):

```bash
cargo bench -p universal-privacy-engine-core -- --output-format bencher
# Note: Criterion's JSON output requires using a specific message-format or installing cargo-criterion
# Alternatively:
cargo bench -p universal-privacy-engine-core -- --noplot --save-baseline default
```

Or using `cargo-criterion` if installed:
```bash
cargo criterion --message-format=json
```

## Interpreting Results

The benchmarks currently measure the overhead of the `PrivacyEngine` trait interface using a mock backend.
This ensures the core abstraction doesn't introduce significant latency.

Input sizes tested:
- 1 KB
- 10 KB
- 100 KB
