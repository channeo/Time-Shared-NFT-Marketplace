use anchor_lang::prelude::*;

declare_id!("DE1wx1MeAQeZQCepnd2Ukd5o2S7v6JJaRfuX1G3EG1rs");

#[account]
pub struct ArtPiece {
    pub title: String,
    pub description: String,
    pub creator: Pubkey,
    pub image_url: String,
    pub metadata_uri: String,
    pub mint: Pubkey,
    pub collection_id: Option<Pubkey>, 
    pub category: String,              
    pub tags: Vec<String>,            
    pub is_listed: bool,
    pub price: u64,
    pub created_at: i64,           
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,               
    pub display_name: String,        
    pub bio: String,                 
    pub profile_pic_url: String,     
    pub join_date: i64,             
}

#[account]
pub struct Listing {
    pub seller: Pubkey,              
    pub art_mint: Pubkey,            
    pub price: u64,                  
    pub listed_at: i64,              
    pub is_active: bool,             
}
#[account]
pub struct TransactionRecord {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub art_mint: Pubkey,
    pub price: u64,
    pub timestamp: i64,
}
#[account]
pub struct MarketplaceSettings {
    pub admin: Pubkey,
    pub platform_fee_basis_points: u16, 
    pub royalty_basis_points: u16,      
    pub treasury_wallet: Pubkey,        
}
#[account]
pub struct Collection {
    pub id: Pubkey,
    pub title: String,
    pub description: String,
    pub creator: Pubkey,
    pub cover_image: String,
    pub created_at: i64,
    pub number_of_pieces: u32,
}
#[account]
pub struct CartItem {
    pub user: Pubkey,
    pub art_mint: Pubkey,
    pub added_at: i64,
}
#[account]
pub struct Favorite {
    pub user: Pubkey,
    pub art_mint: Pubkey,
    pub favorited_at: i64,
}
#[account]
pub struct ArtStats {
    pub art_mint: Pubkey,
    pub views: u64,
    pub favorites: u64,
    pub total_sales: u64,
    pub last_sold_price: u64,
    pub updated_at: i64,
}
#[account]
pub struct Bid {
    pub bidder: Pubkey,
    pub art_mint: Pubkey,
    pub bid_amount: u64,
    pub bid_time: i64,
}
#[account]
pub struct RoyaltyInfo {
    pub art_mint: Pubkey,
    pub creators: Vec<Pubkey>,
    pub shares: Vec<u8>, // Tỉ lệ phần trăm (tổng = 100)
}
#[account]
pub struct Report {
    pub reporter: Pubkey,
    pub art_mint: Pubkey,
    pub reason: String,
    pub reported_at: i64,
    pub status: String, 
}

