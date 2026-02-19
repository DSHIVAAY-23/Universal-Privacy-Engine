// NOTE: These tests covered the legacy Ed25519 zkTLS verifier (removed in Phase 1 cleanup).
// They are kept as a placeholder for the Phase 2 TLSNotary MPC integration.
// See docs/ARCHITECTURE.md for the Phase 2 implementation plan.
//
// To re-enable: implement TLSNotary transcript verification in
// core/src/data_source/zktls.rs and restore the test logic here.

#[cfg(feature = "zktls-legacy-tests")]  // disabled — feature flag not active
mod legacy {
    #[test]
    fn placeholder_zkts_tests_disabled() {
        // Phase 2 tests will live here once TLSNotary MPC is integrated.
    }
}
