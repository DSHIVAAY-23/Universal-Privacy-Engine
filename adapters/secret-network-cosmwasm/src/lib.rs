use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cosmwasm_schema::{cw_serde, QueryResponses};

// ─── Message Types ───────────────────────────────────────────────────────────

#[cw_serde]
pub struct InstantiateMsg {
    /// Committed hash of the Groth16 verification key
    pub verification_key_hash: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Submit a ZK proof for on-chain verification
    Verify {
        /// Groth16 proof bytes (base64-encoded)
        proof: Binary,
        /// Public signals / public inputs
        pub_signals: Vec<String>,
        /// Optional nullifier to prevent replay attacks
        nullifier: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns the stored verification key hash
    #[returns(VerificationKeyResponse)]
    VerificationKeyHash {},
}

#[cw_serde]
pub struct VerificationKeyResponse {
    pub hash: String,
}

// ─── Entry Points ────────────────────────────────────────────────────────────

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Verify {
            proof,
            pub_signals,
            nullifier,
        } => verify(_deps, _env, _info, proof, pub_signals, nullifier),
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VerificationKeyHash {} => {
            // TODO: load from storage
            cosmwasm_std::to_json_binary(&VerificationKeyResponse {
                hash: "placeholder_vk_hash".to_string(),
            })
        }
    }
}

// ─── Business Logic ──────────────────────────────────────────────────────────

/// Placeholder verify function.
/// In production this will call a native precompile or a Groth16 verification library.
fn verify(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    proof: Binary,
    pub_signals: Vec<String>,
    _nullifier: Option<String>,
) -> StdResult<Response> {
    // ⚠️  TODO: replace with real Groth16 / SNARK verification logic
    let _ = (proof, pub_signals);

    Ok(Response::new()
        .add_attribute("action", "verify")
        .add_attribute("status", "placeholder — not yet implemented"))
}
