import { ethers } from "ethers";

export interface Env {
  NOTARY_PRIVATE_KEY: string;
}

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "GET,HEAD,POST,OPTIONS",
  "Access-Control-Allow-Headers": "*",
};

export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    if (request.method === "OPTIONS") {
      return new Response(null, { headers: corsHeaders });
    }

    const url = new URL(request.url);

    if (request.method === "GET" && (url.pathname === "/api/health" || url.pathname === "/healthz")) {
      if (!env.NOTARY_PRIVATE_KEY) {
        return Response.json({ status: "error", message: "Missing NOTARY_PRIVATE_KEY" }, { status: 500, headers: corsHeaders });
      }
      try {
        const wallet = new ethers.Wallet(env.NOTARY_PRIVATE_KEY);
        return Response.json({ status: "ok", notary_address: wallet.address }, { headers: corsHeaders });
      } catch (e: any) {
        return Response.json({ status: "error", message: e.message }, { status: 500, headers: corsHeaders });
      }
    }

    if (request.method === "POST" && url.pathname === "/api/generate-proof") {
      try {
        if (!env.NOTARY_PRIVATE_KEY) {
          throw new Error("Missing NOTARY_PRIVATE_KEY environment variable");
        }

        const wallet = new ethers.Wallet(env.NOTARY_PRIVATE_KEY);
        const body = (await request.json()) as { employee_address: string };
        const employeeAddress = body.employee_address;

        if (!employeeAddress || !ethers.isAddress(employeeAddress)) {
          return Response.json(
            { error: "Invalid employee address format. Expected 0x-prefixed 40-character hex string" },
            { status: 400, headers: corsHeaders }
          );
        }

        const salaryRaw = 75000;
        const timestampSecs = Math.floor(Date.now() / 1000);

        // Replicate Rust behavior: Keccak256(abi.encodePacked(address, uint256, uint256))
        const messageHash = ethers.solidityPackedKeccak256(
          ["address", "uint256", "uint256"],
          [employeeAddress, salaryRaw, timestampSecs]
        );

        // The Rust code calls sign_message on the hash bytes: `self.wallet.sign_message(message_hash.as_bytes())`
        // In ethers.js v6, wallet.signMessage with a Uint8Array adds the EIP-191 prefix correctly.
        const signature = await wallet.signMessage(ethers.getBytes(messageHash));

        const proof = {
          salary: salaryRaw.toString(),
          timestamp: timestampSecs,
          signature: signature,
          notary_pubkey: wallet.address,
        };

        return Response.json(proof, { headers: corsHeaders });
      } catch (error: any) {
        return Response.json(
          { error: `Failed to generate proof: ${error.message}` },
          { status: 500, headers: corsHeaders }
        );
      }
    }

    return new Response("Not Found", { status: 404, headers: corsHeaders });
  },
};
