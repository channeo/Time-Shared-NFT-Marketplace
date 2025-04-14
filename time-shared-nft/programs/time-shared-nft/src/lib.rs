use anchor_lang::prelude::*;

declare_id!("DE1wx1MeAQeZQCepnd2Ukd5o2S7v6JJaRfuX1G3EG1rs");

#[account]
pub struct TimeSharedNFT {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub time_slots: Vec<TimeSlot>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TimeSlot {
    pub start_time: i64,   
    pub end_time: i64,
    pub holder: Pubkey,    
    pub is_available: bool,
}
#[account]
pub struct RentalAgreement {
    pub renter: Pubkey,              
    pub nft: Pubkey,                 
    pub slot_index: u64,            
    pub price: u64,                 
    pub status: RentalStatus,       
    pub created_at: i64,           
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RentalStatus {
    Pending,      
    Active,      
    Completed,   
    Cancelled,    
}

#[account]
pub struct MarketplaceState {
    pub admin: Pubkey,
    pub platform_fee_bps: u16, // Tính theo basis points: 100 = 1%
    pub allow_slot_transfer: bool,
    pub created_at: i64,
}
#[account]
pub struct UserProfile {
    pub wallet: Pubkey,
    pub total_rentals: u32,
    pub good_reviews: u32,
    pub nickname: [u8; 32], // Dùng Web2 sync nickname nếu cần
}
