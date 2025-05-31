import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import { assert, expect } from "chai";
import { Keypair, SystemProgram, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { DecentralizedCrowdFund } from "../target/types/decentralized_crowd_fund";

describe("decentralized_crowd_fund", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DecentralizedCrowdFund as Program<DecentralizedCrowdFund>;

  // Create donor keypair
  const donor = Keypair.generate();
  const donationAmount = 0.5 * LAMPORTS_PER_SOL;

  // Create separate campaigns for each test case
  let successCampaignPda: PublicKey;
  let successVaultPda: PublicKey;
  
  let failureCampaignPda: PublicKey;
  let failureVaultPda: PublicKey;

  it("funds the donor account", async () => {
    // Fund the donor with some SOL
    const tx = await provider.connection.requestAirdrop(donor.publicKey, 2 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(tx);
    const donorBalance = await provider.connection.getBalance(donor.publicKey);
    assert.ok(donorBalance > donationAmount, "Donor account should have enough balance");
  });

  it("✅ Creates campaigns for testing", async () => {
    // Create campaign for successful withdrawal
    const successName = "Success Campaign";
    const successGoal = new anchor.BN(1 * LAMPORTS_PER_SOL);
    const successDeadline = new anchor.BN(Math.floor(Date.now() / 1000) + 60);
    
    [successCampaignPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("campaign"), provider.wallet.publicKey.toBuffer(), Buffer.from(successName)],
      program.programId
    );
    
    [successVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), successCampaignPda.toBuffer()],
      program.programId
    );

    await program.methods.createCampaign(
      successName, 
      "Should succeed", 
      successGoal, 
      successDeadline
    ).accounts({
      campaign: successCampaignPda,
      vault: successVaultPda,
      owner: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();

    // Create campaign for failure cases
    const failureName = "Failure Campaign";
    const failureGoal = new anchor.BN(2 * LAMPORTS_PER_SOL);
    const failureDeadline = new anchor.BN(Math.floor(Date.now() / 1000) + 60);
    
    [failureCampaignPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("campaign"), provider.wallet.publicKey.toBuffer(), Buffer.from(failureName)],
      program.programId
    );
    
    [failureVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), failureCampaignPda.toBuffer()],
      program.programId
    );

    await program.methods.createCampaign(
      failureName, 
      "Should fail", 
      failureGoal, 
      failureDeadline
    ).accounts({
      campaign: failureCampaignPda,
      vault: failureVaultPda,
      owner: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("✅ Created test campaigns");
  });

  it("donates to both campaigns", async () => {
    // Fund success campaign
    await program.methods.donate(new anchor.BN(0.6 * LAMPORTS_PER_SOL))
      .accounts({
        campaign: successCampaignPda,
        vault: successVaultPda,
        donor: donor.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([donor])
      .rpc();

    // Fund failure campaign
    await program.methods.donate(new anchor.BN(0.5 * LAMPORTS_PER_SOL))
      .accounts({
        campaign: failureCampaignPda,
        vault: failureVaultPda,
        donor: donor.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([donor])
      .rpc();

    console.log("✅ Funded both campaigns");
  });

  it('Prevents withdrawal before deadline', async () => {
    try {
      await program.methods.withdraw()
        .accounts({
          campaign: successCampaignPda,
          vault: successVaultPda,
          owner: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      assert.fail("Withdrawal should fail before deadline");
    } catch (error) {
      expect(error instanceof AnchorError && error.error.errorCode.code === "DeadlineNotPassed").to.be.true;
    }
  });

  it('Prevents withdrawal if goal not reached', async () => {
    // Wait for deadline to pass
    await new Promise(resolve => setTimeout(resolve, 61000));
    
    try {
      await program.methods.withdraw()
        .accounts({
          campaign: failureCampaignPda,
          vault: failureVaultPda,
          owner: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      assert.fail('Withdrawal should fail when goal not reached');
    } catch (error) {
      expect(error instanceof AnchorError && error.error.errorCode.code === "GoalNotReached").to.be.true;
    }
  });

  it('Allows successful withdrawal', async () => {
    // Add additional donation to meet the goal
    await program.methods.donate(new anchor.BN(0.4 * LAMPORTS_PER_SOL))
      .accounts({
        campaign: successCampaignPda,
        vault: successVaultPda,
        donor: donor.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([donor])
      .rpc();

    // Get initial balance
    const initialBalance = await provider.connection.getBalance(provider.wallet.publicKey);

    // Withdraw funds from success campaign
    await program.methods.withdraw()
      .accounts({
        campaign: successCampaignPda,
        vault: successVaultPda,
        owner: provider.wallet.publicKey,
        // REMOVED systemProgram: SystemProgram.programId,
      })
      .rpc();

    // Check balance increased
    const finalBalance = await provider.connection.getBalance(provider.wallet.publicKey);
    expect(finalBalance).to.be.greaterThan(initialBalance);
    
    // Check campaign vault is empty
    const vaultBalance = await provider.connection.getBalance(successVaultPda);
    assert.equal(vaultBalance, 0);

    console.log("✅ Withdrawal successful, funds transferred to owner");
}); 
});