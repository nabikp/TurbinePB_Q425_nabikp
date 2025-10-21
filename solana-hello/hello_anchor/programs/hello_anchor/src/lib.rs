use anchor_lang::prelude::*;

declare_id!("9YDkRRJy3steKyuXUSzo8piqVSr35onPV6hBkpk5Td3R");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn say_hello(ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, Solana!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
