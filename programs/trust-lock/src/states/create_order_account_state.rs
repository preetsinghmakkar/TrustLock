use crate::constants::{FulfillerStatus, OrderStatus};
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct CreateOrderAccount {
    pub order_id: u64,
    pub demand: String,
    pub created_at: i64,
    pub released_on: i64,
    pub order_status: OrderStatus,
    pub pitchers: Vec<u16>,
    pub order_fulfiller: Pubkey,
    pub fulfiller_status: FulfillerStatus,
}

impl CreateOrderAccount {
    pub const LEN: usize = 8 + 8 + 4 + 100 + 8 + 8 + 1 + 4 + (50 * 2) + 32 + 1;
}
