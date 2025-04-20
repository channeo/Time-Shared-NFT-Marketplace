pub mod context;
pub mod state;
pub mod error;
use anchor_lang::prelude::*;
use context::*;
declare_id!("DE1wx1MeAQeZQCepnd2Ukd5o2S7v6JJaRfuX1G3EG1rs");


pub mod nft_marketplace {
    use super::*;

    pub fn create_art_piece(
        ctx: Context<crate::context::CreateArtPiece>, 
        args: crate::context::CreateArtPieceArgs,
    ) -> Result<()> {
        crate::context::handler(ctx, args)
    }





}