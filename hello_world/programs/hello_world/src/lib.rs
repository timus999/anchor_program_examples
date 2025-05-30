use anchor_lang::prelude::*;

declare_id!("CTe94hsTMTLr65WkGbCREgg5f6j2qVLFqa774NHj2UU1");


#[program]
pub mod hello_world {
    use super::*;

    pub fn hello_world(_ctx: Context<Initialize>) -> Result<()>{
        msg!("Hello world, from solana smart contract!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}