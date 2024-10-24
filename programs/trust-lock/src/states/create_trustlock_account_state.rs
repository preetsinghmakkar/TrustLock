use crate::constants::TokenContribution;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]

pub struct CreateTrustLockAccountState {
    pub holder: Pubkey,
    pub account_no: u64,
    pub my_opened_orders: Vec<u64>,
    pub my_pitches: Vec<u64>,
    pub contributions: Vec<TokenContribution>,
}

impl CreateTrustLockAccountState {
    pub const LEN: usize = 32 // Pubkey
    + 8  // u64
    + 4 + 8 + 8 + (20 * 8) // Vec<u64> my_opened_orders
    + 4 + 8 + 8 + (20 * 8) + 32 + 4 + (8 + 32 + 8 + 32 + 8) * 50;
}
