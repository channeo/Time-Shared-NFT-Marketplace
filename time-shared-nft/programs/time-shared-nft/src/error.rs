use anchor_lang::prelude::*;
#[error_code]
pub enum NftMarketplaceError {
    #[msg("The price must be greater than zero.")]
    InvalidPrice,

    #[msg("You are not the owner of this art piece.")]
    Unauthorized,

    #[msg("The art piece is already listed.")]
    AlreadyListed,

    #[msg("The art piece is not listed.")]
    NotListed,

    #[msg("Insufficient funds to complete the purchase.")]
    InsufficientFunds,

    #[msg("Invalid metadata or URI.")]
    InvalidMetadata,

    #[msg("This artwork does not exist.")]
    ArtPieceNotFound,

    #[msg("Royalties information is missing or invalid.")]
    InvalidRoyaltyInfo,

    #[msg("You cannot buy your own artwork.")]
    CannotBuyOwnArtwork,

    #[msg("Overflow or underflow occurred during calculation.")]
    MathError,

    #[msg("Marketplace is currently paused.")]
    MarketplacePaused,
}
