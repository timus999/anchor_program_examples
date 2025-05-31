use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("37CFPyYGoaVpssrSmRUaGoVUXqWXWPKmx4Acj3fhzKjF");

#[program]
pub mod decentralized_crowd_fund {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        description: String,
        goal: u64,
        deadline: i64,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.goal = goal;
        campaign.amount = 0;
        campaign.deadline = deadline;
        campaign.owner = *ctx.accounts.owner.key;
        campaign.vault = ctx.accounts.vault.key();
        campaign.vault_bump = ctx.bumps.vault;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.donor.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        let campaign = &mut ctx.accounts.campaign;
        campaign.amount = campaign.amount
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        Ok(())
    }
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
    
        require!(
            campaign.owner == *ctx.accounts.owner.key,
            ErrorCode::Unauthorized
        );
        require!(
            Clock::get()?.unix_timestamp > campaign.deadline,
            ErrorCode::DeadlineNotPassed
        );
        require!(
            campaign.amount >= campaign.goal,
            ErrorCode::GoalNotReached
        );
    
        // Directly transfer lamports instead of using System Program
        let vault_balance = ctx.accounts.vault.lamports();
        let vault_info = ctx.accounts.vault.to_account_info();
        let owner_info = ctx.accounts.owner.to_account_info();
        
        // Transfer lamports directly
        **vault_info.try_borrow_mut_lamports()? -= vault_balance;
        **owner_info.try_borrow_mut_lamports()? += vault_balance;
    
        Ok(())
    }
   
    }


#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 4 + name.len() + 4 + description.len() + 8 + 8 + 8 + 32 + 32 + 1,
        seeds = [b"campaign", owner.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,
    
    #[account(
        init,
        payer = owner,
        seeds = [b"vault", campaign.key().as_ref()],
        bump,
        space = 0
    )]
    /// CHECK: This is a SOL vault account
    pub vault: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump = campaign.vault_bump
    )]
    /// CHECK: This is a SOL vault account
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub donor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(
        mut,
        seeds = [b"vault", campaign.key().as_ref()],
        bump = campaign.vault_bump
    )]
    /// CHECK: This is a SOL vault account
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Campaign {
    pub name: String,
    pub description: String,
    pub goal: u64,
    pub amount: u64,
    pub deadline: i64,
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub vault_bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not the campaign owner")]
    Unauthorized,
    #[msg("Campaign deadline has not passed")]
    DeadlineNotPassed,
    #[msg("Funding goal not reached")]
    GoalNotReached,
    #[msg("Integer overflow")]
    Overflow,
}