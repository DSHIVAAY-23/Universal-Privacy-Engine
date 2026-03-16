var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
var __metadata = (this && this.__metadata) || function (k, v) {
    if (typeof Reflect === "object" && typeof Reflect.metadata === "function") return Reflect.metadata(k, v);
};
import { Field, SmartContract, state, State, method, Permissions, PublicKey, UInt64, Struct, } from 'o1js';
export class RWAProof extends Struct({
    stateRoot: Field,
    minCollateral: UInt64,
    nullifierHash: Field,
}) {
}
/**
 * @title RWAOracle
 * @notice Mina smart contract to verify ZK-TLS Oracle proofs.
 */
export class RWAOracle extends SmartContract {
    // ─── State ──────────────────────────────────────────────────────────────────
    // On Mina, we store a commitment to the nullifier set (e.g., a Merkle Root).
    // For simplicity in this adapter, we track a single root.
    nullifierRoot = State();
    // Storage for active collateral would typically be handled via off-chain storage 
    // with an on-chain commitment or using Mina's actions/events.
    async deploy(args) {
        super.deploy(args);
        this.nullifierRoot.set(Field(0));
        this.account.permissions.set({
            ...Permissions.default(),
            editState: Permissions.proofOrSignature(),
        });
    }
    async submitRWAProof(proof) {
        // 1. Validate proof inputs
        proof.minCollateral.assertGreaterThan(UInt64.from(0));
        // 2. Prevent replay
        // In a real implementation, we would check and update a Merkle Tree of nullifiers.
        // For this adapter, we emit an event that off-chain indexers can track.
        this.emitEvent('rwa-verified', {
            account: this.sender,
            nullifier: proof.nullifierHash,
            amount: proof.minCollateral,
        });
        // 3. Logic to update on-chain state if necessary.
    }
    events = {
        'rwa-verified': Struct({
            account: PublicKey,
            nullifier: Field,
            amount: UInt64,
        }),
    };
}
__decorate([
    state(Field),
    __metadata("design:type", Object)
], RWAOracle.prototype, "nullifierRoot", void 0);
__decorate([
    method,
    __metadata("design:type", Function),
    __metadata("design:paramtypes", [RWAProof]),
    __metadata("design:returntype", Promise)
], RWAOracle.prototype, "submitRWAProof", null);
