use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

#[account]
#[derive(InitSpace, Default, Debug)]
pub struct CreateVaultState {
    /// Bump to identify PDA
    pub bump: u8,

    /// Token Mint
    pub token_mint: Pubkey,

    /// Token Vault
    pub token_vault: Pubkey,
}

impl CreateVaultState {
    pub fn initialize_vault(
        &mut self,
        bump: u8,
        token_mint: &InterfaceAccount<Mint>,
        token_vault: Pubkey,
    ) -> Result<()> {
        self.bump = bump;
        self.token_mint = token_mint.key();
        self.token_vault = token_vault;
        Ok(())
    }
}
