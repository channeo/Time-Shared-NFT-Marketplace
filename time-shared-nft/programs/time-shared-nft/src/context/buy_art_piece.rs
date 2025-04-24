use crate::state::*; 
use anchor_lang::prelude::*;
use crate::error::NftMarketplaceError;

#[derive(Accounts)]
#[instruction()]
pub struct BuyNft<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub seller: AccountInfo<'info>,

    #[account(mut)]
    pub art_piece: Account<'info, ArtPiece>,

    #[account(mut)]
    pub listing: Account<'info, Listing>,

    #[account(init, payer = buyer, space = 8 + TransactionRecord::INIT_SPACE)]
    pub transaction_log: Account<'info, TransactionRecord>,

    #[account(mut)]
    pub art_stats: Account<'info, ArtStats>,

    #[account()]
    pub market_settings: Account<'info, MarketplaceSettings>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuyNft>) -> Result<()> {
    let buyer = &ctx.accounts.buyer;
    let seller = &ctx.accounts.seller;
    let listing = &mut ctx.accounts.listing;
    let art_piece = &mut ctx.accounts.art_piece;
    let transaction_log = &mut ctx.accounts.transaction_log;
    let art_stats = &mut ctx.accounts.art_stats;
    let settings = &ctx.accounts.market_settings;
    let price = listing.price;
    let fee = (price * settings.platform_fee_basis_points as u64) / 10_000;
    let seller_amount = price - fee;

    **seller.try_borrow_mut_lamports()? += seller_amount;
    **buyer.to_account_info().try_borrow_mut_lamports()? -= price;

    listing.is_active = false;
    art_piece.is_listed = false;

    transaction_log.buyer = buyer.key();
    transaction_log.seller = *seller.key;
    transaction_log.art_mint = art_piece.mint;
    transaction_log.price = price;
    transaction_log.timestamp = Clock::get()?.unix_timestamp;

    art_stats.total_sales += 1;
    art_stats.last_sold_price = price;
    art_stats.updated_at = Clock::get()?.unix_timestamp;

    Ok(())
}
