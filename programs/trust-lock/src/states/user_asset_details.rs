use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct UserAssetDetails {
    pub user: Pubkey,
    pub contributions: Vec<TokenContribution>,
}

impl UserAssetDetails {
    pub const LEN: usize = 8 + // Discriminator
    32 + // User Pubkey
    4 + // Length of contributions vector (u32)
    // Add size of each TokenContribution as needed
    (32 + 8 + 8 + 32 + 8) * MAX_CONTRIBUTIONS;
}
