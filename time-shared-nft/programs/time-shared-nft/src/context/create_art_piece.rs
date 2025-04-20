use anchor_lang::prelude::*;
use crate::state::*; 
use crate::error::NftMarketplaceError;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateArtPiece<'info> {
    #[account(
        init,
        payer = user,
        space = ArtPiece::INIT_SPACE,
        seeds = [b"art_piece", user.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub art_piece: Account<'info, ArtPiece>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateArtPieceArgs {
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub metadata_uri: String,
    pub category: String,
    pub tags: Vec<String>,
    pub collection_id: Option<Pubkey>,
}

pub fn handler(
    ctx: Context<CreateArtPiece>,
    args: CreateArtPieceArgs,
) -> Result<()> {
    let art = &mut ctx.accounts.art_piece;
    let user = &ctx.accounts.user;
    if args.title.trim().is_empty() || args.image_url.trim().is_empty() {
        return Err(NftMarketplaceError::InvalidMetadata.into());
    }

    art.title = args.title;
    art.description = args.description;
    art.image_url = args.image_url;
    art.metadata_uri = args.metadata_uri;
    art.creator = user.key();
    art.mint = Pubkey::default(); 
    art.collection_id = args.collection_id;
    art.category = args.category;
    art.tags = args.tags;
    art.is_listed = false;
    art.price = 0;
    art.created_at = Clock::get()?.unix_timestamp;

    Ok(())
}
