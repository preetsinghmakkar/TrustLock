use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

#[account]
#[derive(InitSpace, Default, Debug)]
pub struct TrustLockConfig {
    pub admin: Pubkey,
    pub config_index: u8,
    pub order_id: u64,
    #[max_len(20)]
    pub mint_whitelist: Vec<Pubkey>,
}

impl TrustLockConfig {
    pub fn is_authorized<'info>(&self, signer: &Signer<'info>) -> Result<()> {
        require!(signer.key() == self.admin, ErrorCode::NotApproved);
        Ok(())
    }

    pub fn is_supported(&self, mint_account: &InterfaceAccount<Mint>) -> Result<bool> {
        let mint_info = mint_account.to_account_info();

        msg!("Hello I am in Is_Supported");
        msg!("Now here is the mint_info : {:?} ", mint_info.key());
        msg!("Here is the mint_whitelist : {:?} ", self.mint_whitelist);

        // Check if the mint is in the dynamic whitelist
        if self.mint_whitelist.contains(&mint_info.key()) {
            return Ok(true);
        }

        Ok(false)
    }
}
