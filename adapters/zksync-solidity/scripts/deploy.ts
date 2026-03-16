import { Wallet, utils } from "zksync-ethers";
import * as ethers from "ethers";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { Deployer } from "@matterlabs/hardhat-zksync-deploy";

// load env file
import dotenv from "dotenv";
dotenv.config();

// load wallet private key from env file
const PRIVATE_KEY = process.env.DEPLOYER_PRIVATE_KEY || "";

if (!PRIVATE_KEY)
  throw "Please set DEPLOYER_PRIVATE_KEY in your .env file!";

export default async function (hre: HardhatRuntimeEnvironment) {
  console.log(`Running deploy script for RWAOracle`);

  // Initialize the wallet.
  const wallet = new Wallet(PRIVATE_KEY);

  // Create deployer object and load the artifact of the contract you want to deploy.
  const deployer = new Deployer(hre, wallet);
  const artifact = await deployer.loadArtifact("RWAOracle");

  // Estimate contract deployment fee
  const deploymentFee = await deployer.estimateDeployFee(artifact, [wallet.address]);

  // OPTIONAL: Deposit funds from L1 to L2 if funder's balance is low
  // const depositHandle = await deployer.zkWallet.deposit({
  //   to: deployer.zkWallet.address,
  //   token: utils.ETH_ADDRESS,
  //   amount: deploymentFee.mul(2),
  // });
  // await depositHandle.wait();

  // Deploy this contract. The returned object will be of a `Contract` type, similarly to ones in `ethers`.
  // `wallet.address` is the attester address passed to the constructor.
  const parsedFee = ethers.formatEther(deploymentFee.toString());
  console.log(`The deployment is estimated to cost ${parsedFee} ETH`);

  const oracleContract = await deployer.deploy(artifact, [wallet.address]);

  //obtain the Constructor Arguments
  console.log("Constructor args: " + oracleContract.interface.encodeDeploy([wallet.address]));

  // Show the contract info.
  const contractAddress = await oracleContract.getAddress();
  console.log(`${artifact.contractName} was deployed to ${contractAddress}`);
}
