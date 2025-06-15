import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SelfCustodialTwitterClone } from "../target/types/self_custodial_twitter_clone";
import { expect } from "chai";

describe("self-custodial-twitter-clone", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.selfCustodialTwitterClone as Program<SelfCustodialTwitterClone>;

  const signer = provider.wallet;
  let twitterAccountPDA: anchor.web3.PublicKey;

  // Initialize the account once before all tests
  before(async () => {
    [twitterAccountPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("self-custodial-twitter-clone"),
        signer.publicKey.toBuffer(),
      ],
      program.programId
    );

    const ix = await program.methods.createTwitterAccount(
      "timus999",
      "I am a hacker",
      "hacking is fun!!!"
    );

    const userTwitterAddress = (await ix.pubkeys()).twitterAccount;
    console.log("User Twitter Account Address", userTwitterAddress.toString());

    // Send transaction
    const tx = await ix.rpc();
    console.log("Your transaction signature", tx);

    // Confirm creation
    const userDetails = await program.account.twitterAccount.fetch(userTwitterAddress);
    console.log("Created Account:", userDetails);
  });

  it("updates bio and latest tweet", async () => {
    const newBio = "Updated via test 🚀";
    const newTweet = "Second tweet!";

    await program.methods
      .updateTwitterAccount(newBio, newTweet)
      .accounts({
        signer: signer.publicKey,
        twitterAccount: twitterAccountPDA,
      })
      .rpc();

    const account = await program.account.twitterAccount.fetch(twitterAccountPDA);
    console.log("Updated Account:", account);

    expect(account.bio).to.equal(newBio);
    expect(account.latestTweet).to.equal(newTweet);
    expect(account.authority.toBase58()).to.equal(signer.publicKey.toBase58());
  });

  
});
