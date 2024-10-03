use crate::{constants::INTIALIZE_CONFIG, errors::ErrorCode, TrustLockConfig};
use anchor_lang::prelude::*;

pub fn initialize_trustlock_configuration(
    _ctx: Context<InitializeTrustLockConfig>,
    _index: u8,
) -> Result<()> {
    let sol_trust_config = &mut _ctx.accounts.trustlock_config_account;
    sol_trust_config.config_index = _index;
    sol_trust_config.order_id = 0;
    sol_trust_config.admin = _ctx.accounts.admin.key();

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeTrustLockConfig<'info> {
    // Address to be set as the owner.
    #[account(mut,
        address = crate::admin::id() @ ErrorCode::NotApproved)]
    pub admin: Signer<'info>,

    #[account(init, payer=admin, seeds=[INTIALIZE_CONFIG.as_ref(), admin.key().as_ref()], bump, space= TrustLockConfig::LEN)]
    pub trustlock_config_account: Account<'info, TrustLockConfig>,

    pub system_program: Program<'info, System>,
}
