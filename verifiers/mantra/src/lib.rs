//! # RWA Compliance Verifier for Mantra (CosmWasm)
//!
//! This CosmWasm contract verifies SP1 Groth16 proofs on the Mantra blockchain.
//! Mantra is a Cosmos-based chain focused on real-world asset tokenization.

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
use cw_storage_plus::Item;

// Storage
const CONFIG: Item<Config> = Item::new("config");
const VKEY: Item<VerificationKey> = Item::new("vkey");
const VERIFICATION_COUNT: Item<u64> = Item::new("verification_count");

#[cw_serde]
pub struct Config {
    pub admin: String,
}

#[cw_serde]
pub struct VerificationKey {
    /// Serialized Groth16 verification key
    pub data: Binary,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub verification_key: Binary,
}

#[cw_serde]
pub enum ExecuteMsg {
    VerifyRwaProof {
        proof: Binary,
        public_values: Binary,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    
    #[returns(VerificationCountResponse)]
    VerificationCount {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
}

#[cw_serde]
pub struct VerificationCountResponse {
    pub count: u64,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        admin: info.sender.to_string(),
    };
    CONFIG.save(deps.storage, &config)?;

    let vkey = VerificationKey {
        data: msg.verification_key,
    };
    VKEY.save(deps.storage, &vkey)?;

    VERIFICATION_COUNT.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", info.sender))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::VerifyRwaProof {
            proof,
            public_values,
        } => execute_verify_proof(deps, env, proof, public_values),
    }
}

fn execute_verify_proof(
    deps: DepsMut,
    _env: Env,
    proof: Binary,
    public_values: Binary,
) -> StdResult<Response> {
    // Load verification key
    let _vkey = VKEY.load(deps.storage)?;

    // ═══════════════════════════════════════════════════════════════
    // Groth16 Verification
    // ═══════════════════════════════════════════════════════════════
    //
    // TODO: Integrate cw-zk-verify or implement custom BN254 verification
    //
    // The verification process:
    // 1. Deserialize the Groth16 proof
    // 2. Extract public values (institutional_pubkey, threshold)
    // 3. Verify using BN254 pairing check
    // 4. Validate against stored Vkey
    //
    // Example (pseudo-code):
    // ```
    // use cw_zk_verify::Groth16Verifier;
    //
    // let verifier = Groth16Verifier::new(&vkey.data)?;
    // let valid = verifier.verify(&proof, &public_values)?;
    //
    // if !valid {
    //     return Err(StdError::generic_err("Invalid proof"));
    // }
    // ```
    //
    // For now, we'll use a placeholder that checks basic validity
    // ═══════════════════════════════════════════════════════════════

    // Placeholder verification (REPLACE IN PRODUCTION)
    if proof.is_empty() || public_values.len() < 40 {
        return Err(StdError::generic_err("Invalid proof or public values"));
    }

    // Parse public values (32 bytes pubkey + 8 bytes threshold)
    let institutional_pubkey = &public_values[..32];
    let threshold_bytes: [u8; 8] = public_values[32..40]
        .try_into()
        .map_err(|_| StdError::generic_err("Invalid threshold format"))?;
    let threshold = u64::from_le_bytes(threshold_bytes);

    // Increment verification counter
    let mut count = VERIFICATION_COUNT.load(deps.storage)?;
    count += 1;
    VERIFICATION_COUNT.save(deps.storage, &count)?;

    Ok(Response::new()
        .add_attribute("action", "verify_rwa_proof")
        .add_attribute("institutional_pubkey", hex::encode(institutional_pubkey))
        .add_attribute("threshold", threshold.to_string())
        .add_attribute("verification_count", count.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::VerificationCount {} => to_json_binary(&query_verification_count(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        admin: config.admin,
    })
}

fn query_verification_count(deps: Deps) -> StdResult<VerificationCountResponse> {
    let count = VERIFICATION_COUNT.load(deps.storage)?;
    Ok(VerificationCountResponse { count })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_json, Addr};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            verification_key: Binary::from(vec![1, 2, 3, 4]),
        };

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Query config
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let config: ConfigResponse = from_json(&res).unwrap();
        assert_eq!("creator", config.admin);
    }

    #[test]
    fn verify_proof() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            verification_key: Binary::from(vec![1, 2, 3, 4]),
        };

        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // Create mock proof and public values
        let proof = Binary::from(vec![5, 6, 7, 8]);
        let mut public_values = vec![0u8; 40];
        public_values[..32].copy_from_slice(&[10u8; 32]); // institutional_pubkey
        public_values[32..40].copy_from_slice(&1000000u64.to_le_bytes()); // threshold

        let msg = ExecuteMsg::VerifyRwaProof {
            proof,
            public_values: Binary::from(public_values),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(4, res.attributes.len());
        assert_eq!("verify_rwa_proof", res.attributes[0].value);

        // Check verification count
        let res = query(deps.as_ref(), mock_env(), QueryMsg::VerificationCount {}).unwrap();
        let count_res: VerificationCountResponse = from_json(&res).unwrap();
        assert_eq!(1, count_res.count);
    }
}
