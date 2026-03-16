program upe_rwa_oracle.aleo {
    mapping nullifiers: field => bool;
    mapping active_collateral: address => u128;

    async transition submit_rwa_proof(
        public state_root: field,
        public nullifier_hash: field,
        public min_collateral: u128
    ) -> Future {
        assert(min_collateral > 0u128);
        return finalize_submit_rwa_proof(self.caller, nullifier_hash, min_collateral);
    }

    async function finalize_submit_rwa_proof(
        caller: address,
        nullifier_hash: field,
        min_collateral: u128
    ) {
        let is_spent: bool = Mapping::get_or_use(nullifiers, nullifier_hash, false);
        assert(!is_spent);
        Mapping::set(nullifiers, nullifier_hash, true);
        let current_collateral: u128 = Mapping::get_or_use(active_collateral, caller, 0u128);
        Mapping::set(active_collateral, caller, current_collateral + min_collateral);
    }
}
