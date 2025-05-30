use anchor_lang::prelude::*;

declare_id!("9TvYcNqLptsF9PMgr64bZkMfEhJcomvkCtVwTdFXu8x2");

#[program]
pub mod onchain_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
