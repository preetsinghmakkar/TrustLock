use anchor_lang::prelude::*;

pub const INTIALIZE_CONFIG: &[u8] = b"Config_Initialized";
pub const INITIALIZE_TRUSTLOCK_ACCOUNT: &[u8] = b"TrustLock_Account";
pub const CREATE_ORDER: &[u8] = b"Create_Order";
pub const CREATE_VAULT: &[u8] = b"Create_Vault";

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Fulfilled,
    Cancelled,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum FulfillerStatus {
    Active,
    Inactive,
}
