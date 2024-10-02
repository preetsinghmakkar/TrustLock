use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]

pub struct CreateTrustLockAccountState {
    pub holder: Pubkey,
    pub account_no: u64,
    pub opened_orders: Vec<u64>,
    pub my_pitches: Vec<u64>,
}

impl CreateTrustLockAccountState {
    pub const LEN: usize = 8 + 32 + 8 + 4 + 160 + 4 + 160;
}
