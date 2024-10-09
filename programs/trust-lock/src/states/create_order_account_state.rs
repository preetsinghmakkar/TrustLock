use crate::constants::{FulfillerStatus, OrderStatus};
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct CreateOrderAccount {
    pub order_id: u64,
    pub created_by: Pubkey,
    pub demand: String,
    pub created_at: i64,
    pub order_status: OrderStatus,
    pub pitchers: Vec<Pubkey>,
    pub order_fulfiller: Pubkey,
    pub fulfiller_status: FulfillerStatus,
    pub locked: bool,
    pub amount: u64,
}

impl CreateOrderAccount {
    pub const LEN: usize = 8 + 8 + 32 + 4 + 100 + 8 + 8 + 1 + 4 + (32 * 10) + 32 + 1 + 1;
}
