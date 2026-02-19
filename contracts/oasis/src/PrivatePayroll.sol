// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/**
 * @title PrivatePayroll
 * @notice Stores employee salary data in Oasis Sapphire's TEE-encrypted contract state.
 *
 * @dev TRUST MODEL: State variable confidentiality relies on Oasis Sapphire ParaTime.
 *      Deploy ONLY to Sapphire — on any standard EVM chain (Ethereum, Polygon, etc.)
 *      the `salaries` mapping is readable via `eth_getStorageAt`. The on-chain
 *      signature verification is meaningful on any EVM; the privacy guarantee is not.
 *
 * @dev SIGNATURE SCHEME: ECDSA over EIP-191 prefixed message.
 *      messageHash = keccak256(abi.encodePacked(employee: address, salary: uint256, timestamp: uint256))
 *      ethHash     = keccak256("\x19Ethereum Signed Message:\n32" || messageHash)  [OZ ECDSA]
 *      The Rust notary signs `messageHash` via `sign_message(messageHash.as_bytes())`,
 *      which produces the identical EIP-191 prefixed digest.
 *
 * @dev REPLAY PROTECTION: each proof carries a strictly-increasing timestamp.
 *      The contract rejects any proof whose timestamp is not greater than the
 *      last accepted timestamp for that employee.
 */
contract PrivatePayroll {
    using ECDSA for bytes32;

    // ── Immutable config ──────────────────────────────────────────────────────

    /// The only address allowed to sign salary proofs. Set once at deployment.
    address public immutable TRUSTED_NOTARY;

    // ── Encrypted state (Sapphire ParaTime encrypts these automatically) ──────

    /// Employee salary in USD. Private on Sapphire; readable on standard EVMs.
    mapping(address => uint256) private salaries;

    /// True once a valid proof has been stored for this employee.
    mapping(address => bool) private hasProof;

    /// Last accepted proof timestamp per employee (replay protection).
    mapping(address => uint256) public latestTimestamp;

    // ── Events ────────────────────────────────────────────────────────────────

    /**
     * @notice Emitted when a salary proof is successfully verified and stored.
     * @param employee  The employee whose salary was verified.
     * @param salary    The verified salary value.
     * @param timestamp The notary-attested timestamp of the proof.
     */
    event SalaryVerified(
        address indexed employee,
        uint256 salary,
        uint256 timestamp
    );

    // ── Constructor ───────────────────────────────────────────────────────────

    /**
     * @param _trustedNotary Ethereum address of the notary signer.
     *        Must match the address derived from the notary's secp256k1 private key.
     */
    constructor(address _trustedNotary) {
        require(_trustedNotary != address(0), "zero notary address");
        TRUSTED_NOTARY = _trustedNotary;
    }

    // ── External functions ────────────────────────────────────────────────────

    /**
     * @notice Verify a notary-signed STLOP proof and store the salary in private state.
     *
     * @dev Steps:
     *      1. Validate signature length (malleability guard).
     *      2. Enforce strictly-increasing timestamp (replay protection).
     *      3. Reconstruct the message hash using the same packed encoding as the notary.
     *      4. Apply EIP-191 prefix via OZ ECDSA (handles common malleability pitfalls).
     *      5. Recover signer and check against TRUSTED_NOTARY.
     *      6. Write salary to encrypted state and emit event.
     *
     * @param salary    The salary value the notary attested to.
     * @param timestamp Unix timestamp (seconds) embedded in the proof. Must be strictly
     *                  greater than the last accepted timestamp for `msg.sender`.
     * @param signature 65-byte ECDSA signature produced by the notary (r ++ s ++ v).
     */
    function verifyAndStoreSalary(
        uint256 salary,
        uint256 timestamp,
        bytes calldata signature
    ) external {
        // 1. Signature length check (catches obvious malformed inputs before any crypto)
        require(signature.length == 65, "invalid signature length");

        // 2. Replay protection — timestamp must strictly increase per employee
        require(
            timestamp > latestTimestamp[msg.sender],
            "replay: timestamp not newer"
        );

        // 3. Reconstruct the message hash.
        //    Must match: keccak256(abi.encodePacked(msg.sender, salary, timestamp))
        //    in the Rust notary's `create_message_hash`.
        bytes32 messageHash = keccak256(
            abi.encodePacked(msg.sender, salary, timestamp)
        );

        // 4. Apply EIP-191 prefix and recover signer (OZ handles s-value malleability)
        bytes32 ethSignedHash = ECDSA.toEthSignedMessageHash(messageHash);
        address recovered = ECDSA.recover(ethSignedHash, signature);

        // 5. Validate signer
        require(recovered == TRUSTED_NOTARY, "Invalid Notary Signature");

        // 6. Persist state
        latestTimestamp[msg.sender] = timestamp;
        salaries[msg.sender] = salary;
        hasProof[msg.sender] = true;

        emit SalaryVerified(msg.sender, salary, timestamp);
    }

    /**
     * @notice Read the caller's own verified salary.
     * @dev Only the employee (msg.sender) can decrypt this on Sapphire.
     *      On other EVM chains any caller address trivially reads their slot.
     * @return The employee's verified salary, or reverts if no proof exists.
     */
    function getMySalary() external view returns (uint256) {
        require(hasProof[msg.sender], "no salary record");
        return salaries[msg.sender];
    }
}
