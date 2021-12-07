import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CrowdFunding } from "../target/types/crowd_funding";

describe("CrowdFunding", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

//   const program = anchor.workspace.CrowdFunding as Program<CrowdFunding>;
  const program = anchor.workspace.CrowdFunding

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
