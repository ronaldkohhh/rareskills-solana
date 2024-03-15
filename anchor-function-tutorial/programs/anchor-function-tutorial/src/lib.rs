use anchor_lang::prelude::*;

declare_id!("9kYTEsjjLFQUUvaV78GmBQLaSkEUuc31SB4VkErtRaj8");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
