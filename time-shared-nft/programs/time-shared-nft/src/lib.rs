pub mod context;
pub mod state;
pub mod error;

use anchor_lang::prelude::*;
use crate::context::{create_art_piece::*, buy_art_piece::*};
use crate::context::create_art_piece;
use crate::context::buy_art_piece;
declare_id!("DE1wx1MeAQeZQCepnd2Ukd5o2S7v6JJaRfuX1G3EG1rs");


pub mod nft_marketplace {
    use crate::context::CreateArtPieceArgs;

    use super::*;

   
    pub fn create_art_piece(
        ctx: Context<CreateArtPiece>,
        args: CreateArtPieceArgs,
    ) -> Result<()> {
        create_art_piece::handler(ctx, args)
    }

    pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
        buy_art_piece::handler(ctx)
    }





}