use crate::{constants::INTIALIZE_CONFIG, errors::ErrorCode, TrustLockConfig};
use anchor_lang::prelude::*;

pub fn initialize_trustlock_configuration(
    _ctx: Context<InitializeTrustLockConfig>,
    _index: u8,
    _mint_whitelist: Vec<Pubkey>,
) -> Result<()> {
    let sol_trust_config = &mut _ctx.accounts.trustlock_config_account;
    sol_trust_config.config_index = _index;
    sol_trust_config.order_id = 0;
    sol_trust_config.admin = _ctx.accounts.admin.key();
    sol_trust_config.mint_whitelist = _mint_whitelist;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_index : u8)]
pub struct InitializeTrustLockConfig<'info> {
    // Address to be set as the owner.
    #[account(mut,
        address = crate::admin::id() @ ErrorCode::NotApproved)]
    pub admin: Signer<'info>,

    #[account(init, payer=admin, seeds=[INTIALIZE_CONFIG.as_ref(), &_index.to_be_bytes()], bump, space= 8 + TrustLockConfig::INIT_SPACE)]
    pub trustlock_config_account: Account<'info, TrustLockConfig>,

    pub system_program: Program<'info, System>,
}
