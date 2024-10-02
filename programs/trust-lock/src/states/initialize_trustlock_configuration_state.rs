use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct TrustLockConfig {
    pub admin: Pubkey,
    pub config_index: u8,
    pub order_id: u64,
}

impl TrustLockConfig {
    pub const LEN: usize = 8 + 32 + 1 + 8;

    pub fn is_authorized<'info>(&self, signer: &Signer<'info>) -> Result<()> {
        require!(signer.key() == self.admin, ErrorCode::NotApproved);
        Ok(())
    }
}
