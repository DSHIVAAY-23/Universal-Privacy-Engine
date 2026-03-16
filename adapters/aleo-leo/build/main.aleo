program upe_rwa_oracle.aleo;

mapping nullifiers:
    key as field.public;
    value as boolean.public;

mapping active_collateral:
    key as address.public;
    value as u128.public;

function submit_rwa_proof:
    input r0 as field.public;
    input r1 as field.public;
    input r2 as u128.public;
    gt r2 0u128 into r3;
    assert.eq r3 true;
    async submit_rwa_proof self.caller r1 r2 into r4;
    output r4 as upe_rwa_oracle.aleo/submit_rwa_proof.future;

finalize submit_rwa_proof:
    input r0 as address.public;
    input r1 as field.public;
    input r2 as u128.public;
    get.or_use nullifiers[r1] false into r3;
    not r3 into r4;
    assert.eq r4 true;
    set true into nullifiers[r1];
    get.or_use active_collateral[r0] 0u128 into r5;
    add r5 r2 into r6;
    set r6 into active_collateral[r0];
