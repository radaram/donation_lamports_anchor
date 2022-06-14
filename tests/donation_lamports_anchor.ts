import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DonationLamportsAnchor } from "../target/types/donation_lamports_anchor";

describe("donation_lamports_anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DonationLamportsAnchor as Program<DonationLamportsAnchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
