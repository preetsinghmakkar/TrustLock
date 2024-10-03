use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_2022::spl_token_2022::{
        self,
        extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions},
    },
    token_interface::Mint,
};

#[account]
#[derive(Default, Debug)]
pub struct TrustLockConfig {
    pub admin: Pubkey,
    pub config_index: u8,
    pub order_id: u64,
    pub mint_whitelist: Vec<Pubkey>,
}

impl TrustLockConfig {
    pub const LEN: usize = 8 + 32 + 1 + 8 + (4 * 32);

    pub fn is_authorized<'info>(&self, signer: &Signer<'info>) -> Result<()> {
        require!(signer.key() == self.admin, ErrorCode::NotApproved);
        Ok(())
    }

    pub fn is_supported(&self, mint_account: &InterfaceAccount<Mint>) -> Result<bool> {
        let mint_info = mint_account.to_account_info();

        if *mint_info.owner == Token::id() {
            return Ok(true);
        }

        // Check if the mint is in the dynamic whitelist
        if self.mint_whitelist.contains(&mint_account.key()) {
            return Ok(true);
        }

        // Additional checks for SPL Token 2022 extensions
        let mint_data = mint_info.try_borrow_data()?;
        let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
        let extensions = mint.get_extension_types()?;
        for e in extensions {
            if e != ExtensionType::TransferFeeConfig
                && e != ExtensionType::MetadataPointer
                && e != ExtensionType::TokenMetadata
            {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
