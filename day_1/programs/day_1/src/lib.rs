use anchor_lang::prelude::*;

declare_id!("ErtytA3rFvh9cJTcDgzr3EVfgTEVKECEA6c3mw644tVD");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
