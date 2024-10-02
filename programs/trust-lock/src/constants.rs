use anchor_lang::prelude::*;
// use borsh::{BorshDeserialize, BorshSerialize};

pub const INTIALIZE_CONFIG: &[u8] = b"Config_Initialized";
pub const INITIALIZE_TRUSTLOCK_ACCOUNT: &[u8] = b"TrustLock_Account";
pub const CREATE_ORDER: &[u8] = b"Create_Order";

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Fulfilled,
    Cancelled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Pending
    }
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum FulfillerStatus {
    Active,
    Inactive,
}

impl Default for FulfillerStatus {
    fn default() -> Self {
        FulfillerStatus::Inactive
    }
}
