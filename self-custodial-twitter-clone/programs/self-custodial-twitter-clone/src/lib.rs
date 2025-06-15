use anchor_lang::prelude::*;

declare_id!("Bs4BSjGy2fqcg1QvK2C2odGKnRRPMtuhT2pCeSZ9K2kB");

#[program]
pub mod self_custodial_twitter_clone {
    use super::*;

    pub fn create_twitter_account(ctx: Context<Initialize>, username: String, bio: String, latest_tweet: String) -> Result<()> {
        // Setting user data in the user's Twitter account
        let twitter_account = &mut ctx.accounts.twitter_account;
        twitter_account.bump = ctx.bumps.twitter_account;

       twitter_account.authority = *ctx.accounts.signer.key;
       twitter_account.username = username;
       twitter_account.bio = bio;
       twitter_account.latest_tweet = latest_tweet;


       //printing user info into program's on chain transaction log
       msg!(" Created a new Twitter account with following details:
       \nUsername: {}\nBio: {}\nLatest Tweet: {}\nAuthority: {}",
            twitter_account.username,
            twitter_account.bio,
            twitter_account.latest_tweet,
            twitter_account.authority
        ); 


        Ok(())
    }
    
    pub fn update_twitter_account(ctx: Context<UpdateTwitterAccount>,new_bio: String,  latest_tweet: String) -> Result<()> {
        // Updating the latest tweet in the user's Twitter account
        let twitter_account = &mut ctx.accounts.twitter_account;


        //check authority
        require_keys_eq!(twitter_account.authority, ctx.accounts.signer.key(), TwitterError::Unauthorized);

        twitter_account.bio = new_bio;
        twitter_account.latest_tweet = latest_tweet;

        //printing updated tweet info into program's on chain transaction log
        msg!(" Updated Twitter account for : {}", twitter_account.username);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
   // user's account
   #[account(mut)] 
   pub signer: Signer<'info>,

   //creating a new Twitter account for every user with seed of their wallet address
   // this constraint ensures that the account is created only once
   #[account(
        init,
        payer = signer,
        space = TwitterAccount::LEN,
        seeds = ["self-custodial-twitter-clone".as_bytes(), signer.key().as_ref()],
        bump
   )]
   pub twitter_account: Account<'info, TwitterAccount>,
   pub system_program: Program<'info, System>,
}

#[account]
pub struct TwitterAccount{
    pub authority: Pubkey, // authority of the account, usually the user's wallet address
    pub username: String,
    pub bio: String,
    pub latest_tweet: String,
    pub bump: u8, // bump seed for PDA
}

impl TwitterAccount {

    pub const LEN: usize = 8 + // discriminator
        32 + // authority (Pubkey)
        4 + 32 + // username length
        4 + 64 + // bio length
        4 + 100 +// latest_tweet length
        1; // bump seed

}

#[derive(Accounts)]
pub struct UpdateTwitterAccount<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = ["self-custodial-twitter-clone".as_bytes(), signer.key().as_ref()],
        bump = twitter_account.bump,
    )]
    pub twitter_account : Account<'info, TwitterAccount>,
}

#[error_code]
pub enum TwitterError {
    #[msg("You are not authorized to update this account.")]
    Unauthorized,
}