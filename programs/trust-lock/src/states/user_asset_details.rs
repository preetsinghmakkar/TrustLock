use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserAssetDetails {
    pub user: Pubkey,
    pub contributions: Vec<TokenContribution>,
}

impl UserAssetDetails {
    pub const LEN: usize = 8 + 32 + 4 + (8 + 32 + 8 + 32 + 8) * 50;
}
