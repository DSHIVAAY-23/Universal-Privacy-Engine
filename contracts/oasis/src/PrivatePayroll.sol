// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title PrivatePayroll
 * @notice This contract leverages Sapphire's encrypted state to store salary data privately.
 * @dev Deployed on Oasis Sapphire (Confidential EVM).
 * This contract ingests UPE Signed Proofs and stores salary data in ENCRYPTED state.
 * On a normal chain, "mapping(address => uint256)" is public.
 * On Sapphire, it is PRIVATE by default.
 */
contract PrivatePayroll {
    // TRUSTED NOTARY (Placeholder - Replace with actual from UPE)
    address public constant TRUSTED_NOTARY = 0x1234567890123456789012345678901234567890; 

    // PRIVATE STATE: Only the employee can see their own salary
    // The Sapphire ParaTime encrypts this automatically.
    mapping(address => uint256) private salaries;
    mapping(address => bool) private hasProof;

    event SalaryVerified(address indexed employee, uint256 timestamp);

    // 1. Ingest Signed Proof from UPE (STLOP)
    function verifyAndStoreSalary(
        uint256 salary,
        uint256 timestamp,
        bytes memory signature
    ) external {
        // A. Reconstruct the message hash (Standard EIP-191)
        bytes32 messageHash = keccak256(abi.encodePacked(msg.sender, salary, timestamp));
        bytes32 ethSignedMessageHash = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)
        );

        // B. Verify the Notary signed this specific data
        address recoveredSigner = recoverSigner(ethSignedMessageHash, signature);
        require(recoveredSigner == TRUSTED_NOTARY, "Invalid Notary Signature");

        // C. Store in PRIVATE state (The Oasis Magic)
        salaries[msg.sender] = salary;
        hasProof[msg.sender] = true;

        emit SalaryVerified(msg.sender, timestamp);
    }

    // 2. View Function (Only callable by the employee)
    function getMySalary() external view returns (uint256) {
        require(hasProof[msg.sender], "No salary record");
        return salaries[msg.sender];
    }

    // Helper: Signature Recovery
    function recoverSigner(bytes32 _ethSignedMessageHash, bytes memory _signature)
        internal
        pure
        returns (address)
    {
        (bytes32 r, bytes32 s, uint8 v) = splitSignature(_signature);
        return ecrecover(_ethSignedMessageHash, v, r, s);
    }

    function splitSignature(bytes memory sig)
        internal
        pure
        returns (bytes32 r, bytes32 s, uint8 v)
    {
        require(sig.length == 65, "Invalid signature length");
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }
    }
}
