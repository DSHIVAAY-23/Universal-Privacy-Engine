// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/PrivatePayroll.sol";

/**
 * @title PrivatePayrollTest
 * @notice Foundry tests for PrivatePayroll.sol
 *
 * Key addresses:
 *   notaryKey = 1  →  notary = vm.addr(1)
 *   Employee A = vm.addr(2)
 *   Employee B = vm.addr(3)
 *
 * IMPORTANT: vm.sign(key, hash) signs the hash RAW — no EIP-191 prefix.
 * The contract uses ECDSA.toEthSignedMessageHash which prepends
 * "\x19Ethereum Signed Message:\n32" before calling ecrecover.
 * Therefore we must pass the eth-prefixed hash to vm.sign so ecrecover
 * recovers the right address.
 */
contract PrivatePayrollTest is Test {
    PrivatePayroll payroll;

    uint256 constant NOTARY_KEY = 1;
    address notary;

    // Mirror of the contract event for vm.expectEmit
    event SalaryVerified(
        address indexed employee,
        uint256 salary,
        uint256 timestamp
    );

    function setUp() public {
        notary = vm.addr(NOTARY_KEY);
        payroll = new PrivatePayroll(notary);
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Build the raw message hash: keccak256(abi.encodePacked(who, salary, ts))
    function _rawHash(
        address who,
        uint256 salary,
        uint256 ts
    ) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(who, salary, ts));
    }

    /// Apply EIP-191 prefix: keccak256("\x19Ethereum Signed Message:\n32" || rawHash)
    function _ethHash(bytes32 rawHash) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encodePacked("\x19Ethereum Signed Message:\n32", rawHash)
            );
    }

    /// Sign the EIP-191-prefixed hash and pack into 65-byte signature (r ++ s ++ v).
    function _sign(
        uint256 key,
        address who,
        uint256 salary,
        uint256 ts
    ) internal pure returns (bytes memory) {
        bytes32 prefixed = _ethHash(_rawHash(who, salary, ts));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(key, prefixed);
        return abi.encodePacked(r, s, v);
    }

    // ── Positive tests ────────────────────────────────────────────────────────

    function testValidProofStoresSalary() public {
        address alice = vm.addr(2);
        uint256 salary = 75_000;
        uint256 ts = block.timestamp;

        bytes memory sig = _sign(NOTARY_KEY, alice, salary, ts);

        vm.prank(alice);
        vm.expectEmit(true, false, false, true);
        emit SalaryVerified(alice, salary, ts);

        payroll.verifyAndStoreSalary(salary, ts, sig);

        vm.prank(alice);
        assertEq(payroll.getMySalary(), salary);
    }

    function testLatestTimestampUpdated() public {
        address alice = vm.addr(2);
        uint256 salary = 75_000;
        uint256 ts = block.timestamp;

        vm.prank(alice);
        payroll.verifyAndStoreSalary(
            salary,
            ts,
            _sign(NOTARY_KEY, alice, salary, ts)
        );

        assertEq(payroll.latestTimestamp(alice), ts);
    }

    function testDifferentEmployeesIsolated() public {
        address alice = vm.addr(2);
        address bob = vm.addr(3);
        uint256 ts = block.timestamp;

        vm.prank(alice);
        payroll.verifyAndStoreSalary(
            75_000,
            ts,
            _sign(NOTARY_KEY, alice, 75_000, ts)
        );

        vm.prank(bob);
        payroll.verifyAndStoreSalary(
            120_000,
            ts,
            _sign(NOTARY_KEY, bob, 120_000, ts)
        );

        vm.prank(alice);
        assertEq(payroll.getMySalary(), 75_000);
        vm.prank(bob);
        assertEq(payroll.getMySalary(), 120_000);
    }

    // ── Negative tests ────────────────────────────────────────────────────────

    function testInvalidSignatureReverts() public {
        address alice = vm.addr(2);
        uint256 salary = 75_000;
        uint256 ts = block.timestamp;

        bytes memory sig = _sign(NOTARY_KEY, alice, salary, ts);
        sig[10] = bytes1(uint8(sig[10]) ^ 0xFF); // corrupt one byte

        vm.prank(alice);
        // OZ ECDSA.recover reverts with "ECDSA: invalid signature" when the
        // s value is pushed out of valid range by the corruption. Our custom
        // require is only reached when recover succeeds but returns wrong addr.
        vm.expectRevert("ECDSA: invalid signature");
        payroll.verifyAndStoreSalary(salary, ts, sig);
    }

    function testWrongSignerReverts() public {
        address alice = vm.addr(2);
        uint256 salary = 75_000;
        uint256 ts = block.timestamp;

        // Sign with a different key (not the notary)
        bytes memory sig = _sign(42, alice, salary, ts);

        vm.prank(alice);
        vm.expectRevert("Invalid Notary Signature");
        payroll.verifyAndStoreSalary(salary, ts, sig);
    }

    function testTamperedSalaryReverts() public {
        address alice = vm.addr(2);
        uint256 ts = block.timestamp;

        // Notary signs salary=75000 but attacker submits salary=999999
        bytes memory sig = _sign(NOTARY_KEY, alice, 75_000, ts);

        vm.prank(alice);
        vm.expectRevert("Invalid Notary Signature");
        payroll.verifyAndStoreSalary(999_999, ts, sig);
    }

    function testReplaySameTimestampReverts() public {
        address alice = vm.addr(2);
        uint256 salary = 75_000;
        uint256 ts = block.timestamp;

        bytes memory sig = _sign(NOTARY_KEY, alice, salary, ts);

        vm.prank(alice);
        payroll.verifyAndStoreSalary(salary, ts, sig);

        // Exact same proof again — must revert
        vm.prank(alice);
        vm.expectRevert("replay: timestamp not newer");
        payroll.verifyAndStoreSalary(salary, ts, sig);
    }

    function testReplayOlderTimestampReverts() public {
        address alice = vm.addr(2);
        uint256 salary = 75_000;
        uint256 ts1 = block.timestamp;
        uint256 ts2 = ts1 - 1; // older

        vm.prank(alice);
        payroll.verifyAndStoreSalary(
            salary,
            ts1,
            _sign(NOTARY_KEY, alice, salary, ts1)
        );

        vm.prank(alice);
        vm.expectRevert("replay: timestamp not newer");
        payroll.verifyAndStoreSalary(
            salary,
            ts2,
            _sign(NOTARY_KEY, alice, salary, ts2)
        );
    }

    function testCrossEmployeeReplayReverts() public {
        // Bob tries to reuse Alice's valid sig — msg.sender=bob ≠ alice → hash mismatch
        address alice = vm.addr(2);
        address bob = vm.addr(3);
        uint256 salary = 75_000;
        uint256 ts = block.timestamp;

        bytes memory aliceSig = _sign(NOTARY_KEY, alice, salary, ts);

        vm.prank(bob);
        vm.expectRevert("Invalid Notary Signature");
        payroll.verifyAndStoreSalary(salary, ts, aliceSig);
    }

    function testShortSignatureReverts() public {
        address alice = vm.addr(2);
        bytes memory badSig = new bytes(64); // must be 65

        vm.prank(alice);
        vm.expectRevert("invalid signature length");
        payroll.verifyAndStoreSalary(75_000, block.timestamp, badSig);
    }

    function testGetSalaryWithoutProofReverts() public {
        address alice = vm.addr(2);
        vm.prank(alice);
        vm.expectRevert("no salary record");
        payroll.getMySalary();
    }
}
