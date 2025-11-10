use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV1CpiBuilder;
use mpl_core::types::{PluginAuthorityPair, Attributes, Attribute, DataState};

declare_id!("3SBr7mg13DbMyhEzgYvwGb4SLfd9iEiCfJ2r83f2YPiE");

#[program]
pub mod metaplex_core_example {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Program initialized successfully!");
        Ok(())
    }

    pub fn mint_asset(ctx: Context<MintAsset>) -> Result<()> {
        ctx.accounts.mint_core_asset()
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct MintAsset<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This is the Metaplex Core program (verified by address)
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> MintAsset<'info> {
    pub fn mint_core_asset(&self) -> Result<()> {
        // Call the Metaplex Core CPI builder
        CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .authority(Some(&self.user.to_account_info()))
            .payer(&self.user.to_account_info())
            .owner(Some(&self.user.to_account_info()))
            .update_authority(Some(&self.user.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name("My First Devnet NFT".to_string())
            .uri("https://ipfs.io/ipfs/bafkreicufyq6j66ojlfdyhxujg32uu4gmejwi5vjxshcxdawkqpt4b55vm".to_string())
            .data_state(DataState::AccountState)
            .plugins(vec![PluginAuthorityPair {
                plugin: mpl_core::types::Plugin::Attributes(Attributes {
                    attribute_list: vec![Attribute {
                        key: "Creator".to_string(),
                        value: "Kalash".to_string(),
                    }],
                }),
                authority: None,
            }])
            .invoke()?;
        Ok(())
    }
}
