use anchor_lang::prelude::*;
use mpl_core::instructions::{
    CreateCollectionV2CpiBuilder, CreateV2CpiBuilder,
};
use mpl_core::types::{Asset, AssetData, Collection, Creator, PluginAuthorityPair, Royalties, RoyaltiesBasisPoints};

declare_id!("H5N1KkLRidcWdfBDcftW6MwDTquCj4NxEfBQ72XXQiyN");

#[program]
pub mod metaplex_core_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // Create a Core Collection via CPI into mpl-core
    pub fn create_collection(
        ctx: Context<CreateCollectionCtx>,
        name: String,
        uri: String,
    ) -> Result<()> {
        let mut cpi = CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core);
        cpi
            .collection(&ctx.accounts.collection)
            .payer(&ctx.accounts.payer)
            .update_authority(&ctx.accounts.update_authority)
            .system_program(&ctx.accounts.system_program)
            .name(name)
            .uri(uri);

        cpi.invoke()?;
        Ok(())
    }

    // Create a Core Asset (NFT) in a collection with metadata via CPI
    pub fn create_asset(
        ctx: Context<CreateAssetCtx>,
        name: String,
        uri: String,
    ) -> Result<()> {
        let mut cpi = CreateV2CpiBuilder::new(&ctx.accounts.mpl_core);
        cpi
            .asset(&ctx.accounts.asset)
            .payer(&ctx.accounts.payer)
            .owner(&ctx.accounts.owner)
            .update_authority(&ctx.accounts.update_authority)
            .collection(Some(&ctx.accounts.collection))
            .system_program(&ctx.accounts.system_program)
            .name(name)
            .uri(uri);

        cpi.invoke()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateCollectionCtx<'info> {
    /// CHECK: PDA created by mpl-core
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub update_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: External program
    pub mpl_core: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CreateAssetCtx<'info> {
    /// CHECK: PDA created by mpl-core
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: PDA of the collection
    #[account(mut)]
    pub collection: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// Owner of the asset
    pub owner: Signer<'info>,
    /// Update authority for the asset
    pub update_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: External program
    pub mpl_core: UncheckedAccount<'info>,
}
