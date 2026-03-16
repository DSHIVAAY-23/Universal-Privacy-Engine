import { NextResponse } from "next/server";
import { exec } from "child_process";
import { promisify } from "util";
import path from "path";

const execAsync = promisify(exec);

export async function POST(req: Request) {
    try {
        const body = await req.json().catch(() => ({}));
        const assetContract: string = body.assetContract ?? "0x0000000000000000000000000000000000000000";
        const minRequiredValue: string = body.minRequiredValue ?? "500000";

        // Call the real snarkjs generation script headlessly!
        const scriptPath = path.resolve(process.cwd(), "../../packages/upe-core-circuits/scripts/generate_proof.js");

        const t0 = performance.now();
        const { stdout, stderr } = await execAsync(`node ${scriptPath} ${minRequiredValue} ${assetContract}`);
        const t1 = performance.now();

        if (stderr && !stderr.includes("Warning") && !stderr.includes("ExperimentalWarning")) {
            console.error("Prover Script Error:", stderr);
        }

        const proofData = JSON.parse(stdout);

        return NextResponse.json({
            source: "rust-backend-with-snarkjs",
            proof: proofData.proof,
            flatProof: proofData.flatProof,
            publicSignals: proofData.publicSignals,
            nullifierHash: proofData.nullifierHash,
            stateRoot: proofData.stateRoot,
            assetContract: assetContract,
            minRequiredValue: minRequiredValue,
            proverTimeMs: (t1 - t0).toFixed(2),
            timestamp: Math.floor(Date.now() / 1000),
        });

    } catch (error: unknown) {
        console.error("Prover Route Error:", error);
        return NextResponse.json({ error: "Failed to generate real ZK proof.", details: (error as Error).message }, { status: 500 });
    }
}
