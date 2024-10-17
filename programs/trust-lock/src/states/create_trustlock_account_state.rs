use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Default, Debug)]

pub struct CreateTrustLockAccountState {
    pub holder: Pubkey,
    pub account_no: u64,
    #[max_len(20)]
    pub my_opened_orders: Vec<u64>,
    #[max_len(20)]
    pub my_pitches: Vec<u64>,
}
