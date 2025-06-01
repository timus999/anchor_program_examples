import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SelfCustodialFacebook } from "../target/types/self_custodial_facebook";

describe("self-custodial-facebook", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.selfCustodialFacebook as Program<SelfCustodialFacebook>;

  it("Creating a new account for user", async () => {
    const ix = await program.methods.createFacebook("Deep", "always tinkring", "0xdeep")
    const userFacebookAddress = (await ix.pubkeys()).facebookAccount;
    console.log("User facebook account address: ", userFacebookAddress.toString() );
    //create user's facebook address
    const tx = await ix.rpc();
    console.log("Your transaction signature: ", tx);
    // user details
    let userDetails = await program.account.facebookAccount.fetch(userFacebookAddress);
    console.log(`Created a new account with following details \n Name :: ${userDetails.name} \n Status :: ${userDetails.status} \n Twitter :: ${userDetails.twitter}`);
    
  });



  it("Find Someone's Facebook", async () => {
    const userAddress = new anchor.web3.PublicKey(provider.publicKey);
    const [userFacebookAddress, _] = await anchor.web3.PublicKey.findProgramAddressSync([
      anchor.utils.bytes.utf8.encode('self-custodial-facebook'),
      userAddress.toBuffer(),
    ], program.programId);

    try {
      let userDetails = await program.account.facebookAccount.fetch(userFacebookAddress);
      console.log(`Found user's account with following details \n Name :: ${userDetails.name} \n Status :: ${userDetails.status} \n Twitter :: ${userDetails.twitter}`);
    }catch (error) {
      console.log("Users account does not exist :: ", error);
    }
  });


  it("Close My Facebook Account", async () => {
    const ix = await program.methods.deleteAccount()
    const userFacebookAddress = (await ix.pubkeys()).facebookAccount
    console.log("user facebook address :: ", userFacebookAddress.toString());  
    // Create user's facebook address
    const tx = await ix.rpc()
    console.log("Your transaction signature", tx);
    // User Details Not found, 'cuz we closed the account
    try {
      let userDetails = await program.account.facebookAccount.fetch(userFacebookAddress);
      console.log(`Created a new account with following details \n Name :: ${userDetails.name} \n Status :: ${userDetails.status} \n Twitter :: ${userDetails.twitter}`)
    } catch {
      console.log("User Details Not found, 'cuz we close the account");
    }
  });
});
