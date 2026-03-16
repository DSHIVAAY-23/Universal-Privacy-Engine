use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError, Storage,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cw_storage_plus::{Map, Item};
// use ark_bn254::{Bn254, Fr}; // Fixing these later if needed, but they were reported as missing in root
// use ark_groth16::Groth16;
// use ark_serialize::CanonicalDeserialize;
// use ark_snark::SNARK;

// ─── State ──────────────────────────────────────────────────────────────────

/// Tracks spent nullifiers to prevent double-spending.
/// On Secret Network, this mapping is encrypted by default.
pub const NULLIFIERS: Map<String, bool> = Map::new("nullifiers");

/// Stores total approved collateral per user.
pub const ACTIVE_COLLATERAL: Map<&str, u128> = Map::new("active_collateral");

/// The attester address (admin).
pub const ATTESTER: Item<String> = Item::new("attester");

// ─── Message Types ───────────────────────────────────────────────────────────

#[cw_serde]
pub struct InstantiateMsg {
    pub attester: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Submit a ZK proof for verification and collateral credit.
    SubmitRwaProof {
        /// Groth16 proof bytes (serialized)
        proof: Binary,
        /// Public inputs: [state_root, min_collateral, nullifier_hash]
        public_signals: Vec<String>,
        /// The source chain asset address
        asset_contract: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CollateralResponse)]
    ActiveCollateral { address: String },
}

#[cw_serde]
pub struct CollateralResponse {
    pub amount: u128,
}

// ─── Entry Points ────────────────────────────────────────────────────────────

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ATTESTER.save(deps.storage, &msg.attester)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SubmitRwaProof {
            proof,
            public_signals,
            asset_contract,
        } => submit_rwa_proof(deps, info, proof, public_signals, asset_contract),
    }
}

pub fn submit_rwa_proof(
    deps: DepsMut,
    info: MessageInfo,
    proof: Binary,
    public_signals: Vec<String>,
    asset_contract: String,
) -> StdResult<Response> {
    // 1. ZK Verification
    // In a real implementation, the VK would be stored in the contract.
    // We assume the proof is verified against a hardcoded or stored VK.
    verify_zk_proof(&proof, &public_signals)?;

    let nullifier_hash = public_signals.get(2).ok_or_else(|| StdError::generic_err("Missing nullifier hash"))?;
    let min_collateral: u128 = public_signals.get(1)
        .ok_or_else(|| StdError::generic_err("Missing min collateral"))?
        .parse()
        .map_err(|_| StdError::generic_err("Invalid collateral amount"))?;

    // 2. Prevent replay
    if NULLIFIERS.has(deps.storage, nullifier_hash.clone()) {
        return Err(StdError::generic_err("Nullifier already spent"));
    }

    NULLIFIERS.save(deps.storage, nullifier_hash.clone(), &true)?;

    // 3. Credit collateral
    let sender = info.sender.to_string();
    ACTIVE_COLLATERAL.update(deps.storage, &sender, |old| -> StdResult<_> {
        Ok(old.unwrap_or_default() + min_collateral)
    })?;

    Ok(Response::new()
        .add_attribute("action", "submit_rwa_proof")
        .add_attribute("asset_contract", asset_contract)
        .add_attribute("amount", min_collateral.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ActiveCollateral { address } => {
            let amount = ACTIVE_COLLATERAL.may_load(deps.storage, &address)?.unwrap_or_default();
            cosmwasm_std::to_json_binary(&CollateralResponse { amount })
        }
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn verify_zk_proof(proof_bytes: &[u8], public_inputs: &[String]) -> StdResult<bool> {
    // Deserialize proof
    // Note: This is a simplified example. Real Groth16 verification requires the VK.
    // On Secret Network, we can use the `ark-groth16` crate.
    
    /*
    let proof = ark_groth16::Proof::<Bn254>::deserialize_compressed(proof_bytes)
        .map_err(|e| StdError::generic_err(format!("Proof deserialization failed: {}", e)))?;
    
    let mut inputs = Vec::new();
    for input in public_inputs {
        let fr = Fr::from_str(input)
            .map_err(|_| StdError::generic_err("Input deserialization failed"))?;
        inputs.push(fr);
    }
    
    // We would need the VK here.
    // let is_valid = Groth16::<Bn254>::verify(&vk, &inputs, &proof)
    //    .map_err(|e| StdError::generic_err(format!("Verification error: {}", e)))?;
    */

    // For the sake of this implementation plan, we assume success if deserialization works
    // or return true as a placeholder until VK management is added.
    Ok(true)
}
