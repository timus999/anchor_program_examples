use anchor_lang::prelude::*;

declare_id!("J1ws3NfvCUgBDbRhYLu4EZZgat8R3SfTyDtvxRfDoAPp");

#[program]
pub mod self_custodial_facebook {
    use super::*;

    pub fn create_facebook(ctx: Context<Initialize>, name: String, status: String, twitter: String) ->
        Result<()> {
            // setting user data in user's account
            let users_account_data = &mut ctx.accounts.facebook_account;
            users_account_data.bump = ctx.bumps.facebook_account;



            users_account_data.authority = *ctx.accounts.signer.key;
            users_account_data.name = name.to_owned();
            users_account_data.status = status.to_owned();
            users_account_data.twitter = twitter.to_owned();


            // printing user info into program's on-chain trancsaction log.
            msg!("Created a new account with following details
                Name :: {0}
                Status :: {1}
                Twitter :: {2}
                ", name, status, twitter);
        
        Ok(())
        }

    pub fn update_status(ctx: Context<Update>, new_status: String) -> Result<()> {
        // Update user status, much more like whatsapp 24 hour status, without self destruction
        msg!("Updating status from :: {0} -> to :: {1}", &ctx.accounts.facebook_account.status, &new_status);
        ctx.accounts.facebook_account.status = new_status;
        Ok(())
    }

    pub fn delete_account(_ctx: Context<Close>) -> Result<()> {
        msg!("Account Closed Successfully");
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // User's account
    #[account(mut)]
    pub signer: Signer<'info>,
    // Creating a new account for every user with seed of their wallet address.
    // This constraint allow on-account per wallet address
    #[account(
        init,
        payer = signer,
        space = FacebookAccount::LEN,
        seeds = ["self-custodial-facebook".as_bytes(), signer.key().as_ref()],
        bump
    )]

    pub facebook_account: Account<'info, FacebookAccount>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct FacebookAccount {
    pub authority: Pubkey, //Authority of this account
    pub bump: u8,
    pub name: String, // Max 10 Chars
    pub status: String, // Max 100 chars
    pub twitter: String, // Max 10 chars
}

impl FacebookAccount {
    const LEN: usize = 
        8 + // discriminator
        32 + // PubKey
        1 + // bump
        ( 4 + 10 ) + // 10 chars of Name
        ( 4  + 100) + // 100 chars of Status
        ( 4 + 10 ); // 10 chars of Twitter
}


#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    //user's facebook account
    #[account(
        mut,
        seeds = ["self-custodial-facebook".as_bytes(), signer.key().as_ref()],
        bump = facebook_account.bump,
    )]
    pub facebook_account: Account<'info, FacebookAccount>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // we will use'close' for closing user's facebook account
    #[account(
        mut,
        seeds = ["self-custodial-facebook".as_bytes(), signer.key().as_ref()],
        bump = facebook_account.bump,
        close = signer
    )]
    pub facebook_account: Account<'info, FacebookAccount>,
}