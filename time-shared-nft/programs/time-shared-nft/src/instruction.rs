use anchor_lang::prelude::*;

declare_id!("DE1wx1MeAQeZQCepnd2Ukd5o2S7v6JJaRfuX1G3EG1rs");
#[account]
#[derive(InitSpace)] 
pub struct ArtPiece {
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub description: String,
    pub creator: Pubkey,
    #[max_len(1000)]
    pub image_url: String,
    #[max_len(1000)]
    pub metadata_uri: String,
    pub mint: Pubkey,
    pub collection_id: Option<Pubkey>, 
    #[max_len(1000)]
    pub category: String,       
    #[max_len(10,10)]       
    pub tags: Vec<String>,            
    pub is_listed: bool,
    pub price: u64,
    pub created_at: i64,           
}

#[account]
#[derive(InitSpace)] 
pub struct UserProfile {
    pub owner: Pubkey,    
    #[max_len(50)]           
    pub display_name: String, 
    #[max_len(50)]        
    pub bio: String,      
    #[max_len(50)]            
    pub profile_pic_url: String,     
    pub join_date: i64,             
}

#[account]
#[derive(InitSpace)] 
pub struct Listing {
    pub seller: Pubkey,              
    pub art_mint: Pubkey,            
    pub price: u64,                  
    pub listed_at: i64,              
    pub is_active: bool,             
}
#[account]
#[derive(InitSpace)] 
pub struct TransactionRecord {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub art_mint: Pubkey,
    pub price: u64,
    pub timestamp: i64,
}
#[account]
#[derive(InitSpace)] 
pub struct MarketplaceSettings {
    pub admin: Pubkey,
    pub platform_fee_basis_points: u16, 
    pub royalty_basis_points: u16,      
    pub treasury_wallet: Pubkey,        
}
#[account]
#[derive(InitSpace)]
pub struct Collection {
    pub id: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub description: String,
    pub creator: Pubkey,
    #[max_len(1000)]
    pub cover_image: String,
    pub created_at: i64,
    pub number_of_pieces: u32,
}
#[account]
#[derive(InitSpace)] 
pub struct CartItem {
    pub user: Pubkey,
    pub art_mint: Pubkey,
    pub added_at: i64,
}
#[account]
#[derive(InitSpace)] 
pub struct Favorite {
    pub user: Pubkey,
    pub art_mint: Pubkey,
    pub favorited_at: i64,
}
#[account]
#[derive(InitSpace)] 
pub struct ArtStats {
    pub art_mint: Pubkey,
    pub views: u64,
    pub favorites: u64,
    pub total_sales: u64,
    pub last_sold_price: u64,
    pub updated_at: i64,
}
#[account]
#[derive(InitSpace)] 
pub struct Bid {
    pub bidder: Pubkey,
    pub art_mint: Pubkey,
    pub bid_amount: u64,
    pub bid_time: i64,
}
#[account]
#[derive(InitSpace)] 
pub struct RoyaltyInfo {
    pub art_mint: Pubkey,
    #[max_len(50)] 
    pub creators: Vec<Pubkey>,
    #[max_len(50)]
    pub shares: Vec<u8>, 
}
#[account]
#[derive(InitSpace)] 
pub struct Report {
    pub reporter: Pubkey,
    pub art_mint: Pubkey,
    #[max_len(100)]
    pub reason: String,
    pub reported_at: i64,
    #[max_len(50)]
    pub status: String, 
}

