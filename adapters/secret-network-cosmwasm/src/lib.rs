//! UPE Secret Network Adapter
//!
//! Stores ZK-verified RWA collateral on Secret Network (CosmWasm).
//! State is encrypted by default via Secret Network's TEE.
//!
//! ## ZK Verification Strategy
//! Full on-chain Groth16 verification (ark-groth16) is not yet available in
//! CosmWasm-WASM due to WASM binary size limits with BN254 pairing libs.
//!
//! The current approach:
//! 1. The proof is validated structurally (correct length / encoding).
//! 2. Public signals are range-checked.
//! 3. The ATTESTER (a trusted co-signer controlled by the UPE Rust Notary)
//!    must co-sign the tx — i.e., `MessageInfo.sender == ATTESTER`.
//!
//! Phase 2 will replace step 3 with full Groth16 verification once the
//! `ark-cosmwasm` crate reaches production stability.

use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_storage_plus::{Map, Item};

// ─── State ───────────────────────────────────────────────────────────────────

/// Tracks spent nullifiers (double-spend prevention).
/// On Secret Network this mapping is TEE-encrypted automatically.
pub const NULLIFIERS: Map<String, bool> = Map::new("nullifiers");

/// Stores total verified collateral per user.
pub const ACTIVE_COLLATERAL: Map<&str, u128> = Map::new("active_collateral");

/// The trusted attester (UPE Rust Notary).
pub const ATTESTER: Item<String> = Item::new("attester");

// ─── Messages ────────────────────────────────────────────────────────────────

#[cw_serde]
pub struct InstantiateMsg {
    /// Bech32 address of the trusted UPE Notary (attester).
    pub attester: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Submit a ZK proof for verification and collateral credit.
    ///
    /// `proof`          - Base64-encoded Groth16 proof bytes (256 bytes / 8 × BN254 field elements).
    /// `public_signals` - Hex-encoded public inputs: [state_root, min_collateral, nullifier_hash].
    /// `asset_contract` - Source chain asset address (arbitrary string for cross-chain assets).
    SubmitRwaProof {
        proof: String,
        public_signals: Vec<String>,
        asset_contract: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CollateralResponse)]
    ActiveCollateral { address: String },

    #[returns(AttesterResponse)]
    GetAttester {},
}

#[cw_serde]
pub struct CollateralResponse {
    pub amount: u128,
}

#[cw_serde]
pub struct AttesterResponse {
    pub attester: String,
}

// ─── Entry Points ─────────────────────────────────────────────────────────────

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.addr_validate(&msg.attester)?;
    ATTESTER.save(deps.storage, &msg.attester)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("attester", msg.attester))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SubmitRwaProof {
            proof,
            public_signals,
            asset_contract,
        } => submit_rwa_proof(deps, env, info, proof, public_signals, asset_contract),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ActiveCollateral { address } => {
            let amount = ACTIVE_COLLATERAL
                .may_load(deps.storage, &address)?
                .unwrap_or_default();
            to_json_binary(&CollateralResponse { amount })
        }
        QueryMsg::GetAttester {} => {
            let attester = ATTESTER.load(deps.storage)?;
            to_json_binary(&AttesterResponse { attester })
        }
    }
}

// ─── Execution Logic ─────────────────────────────────────────────────────────

pub fn submit_rwa_proof(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proof: String,
    public_signals: Vec<String>,
    asset_contract: String,
) -> StdResult<Response> {
    // ── 1. Attester gate ──────────────────────────────────────────────────────
    // Until full Groth16 on-chain verification is available, only the trusted
    // UPE Notary can submit proofs. This prevents arbitrary proof injection.
    let attester = ATTESTER.load(deps.storage)?;
    if info.sender.to_string() != attester {
        return Err(StdError::generic_err(
            "Unauthorized: only the UPE Notary attester may submit proofs. \
             Full Groth16 on-chain verification is planned for Phase 2.",
        ));
    }

    // ── 2. Structural proof validation ──────────────────────────────────────
    validate_proof_structure(&proof)?;

    // ── 3. Parse public signals ───────────────────────────────────────────────
    if public_signals.len() != 3 {
        return Err(StdError::generic_err(
            "Expected exactly 3 public signals: [state_root, min_collateral, nullifier_hash]",
        ));
    }

    let nullifier_hash = public_signals[2].clone();
    let min_collateral: u128 = public_signals[1]
        .parse()
        .map_err(|_| StdError::generic_err("Invalid min_collateral: must be a decimal integer"))?;

    if min_collateral == 0 {
        return Err(StdError::generic_err("min_collateral must be greater than 0"));
    }

    // ── 4. Nullifier replay check ─────────────────────────────────────────────
    if NULLIFIERS.has(deps.storage, nullifier_hash.clone()) {
        return Err(StdError::generic_err(
            "Nullifier already spent — this collateral proof has already been used",
        ));
    }
    NULLIFIERS.save(deps.storage, nullifier_hash.clone(), &true)?;

    // ── 5. Credit collateral (TEE-encrypted on Secret Network) ───────────────
    let recipient = info.sender.to_string();
    ACTIVE_COLLATERAL.update(deps.storage, &recipient, |old| -> StdResult<_> {
        Ok(old.unwrap_or_default() + min_collateral)
    })?;

    Ok(Response::new()
        .add_attribute("action", "submit_rwa_proof")
        .add_attribute("asset_contract", asset_contract)
        .add_attribute("nullifier_hash", nullifier_hash)
        .add_attribute("amount_credited", min_collateral.to_string())
        .add_attribute("recipient", recipient))
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Validates that the proof is a properly base64-encoded byte string
/// of exactly 256 bytes (8 × 32-byte BN254 field elements for a Groth16 proof).
///
/// This is a structural guard — it will reject obviously malformed proofs
/// even before full on-chain verification is available.
fn validate_proof_structure(proof_b64: &str) -> StdResult<()> {
    use base64::{engine::general_purpose::STANDARD, Engine};

    let bytes = STANDARD
        .decode(proof_b64)
        .map_err(|e| StdError::generic_err(format!("Proof is not valid base64: {}", e)))?;

    // A Groth16 proof over BN254 serialises to:
    //   A  (G1): 64 bytes
    //   B  (G2): 128 bytes
    //   C  (G1): 64 bytes
    //   Total  : 256 bytes
    if bytes.len() != 256 {
        return Err(StdError::generic_err(format!(
            "Invalid proof length: expected 256 bytes (Groth16/BN254), got {}",
            bytes.len()
        )));
    }

    Ok(())
}
