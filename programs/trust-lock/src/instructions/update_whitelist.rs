use crate::{errors::ErrorCode, TrustLockConfig};
use anchor_lang::prelude::*;

pub fn update_whitelist(_ctx: Context<UpdateWhitelist>, _new_whitelist: Vec<Pubkey>) -> Result<()> {
    let trustlock_config_account = &mut _ctx.accounts.trustlock_config_account;

    trustlock_config_account.mint_whitelist = _new_whitelist;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateWhitelist<'info> {
    #[account(mut,  address = crate::admin::id() @ ErrorCode::NotApproved)]
    pub admin: Signer<'info>,

    pub trustlock_config_account: Box<Account<'info, TrustLockConfig>>,

    pub system_program: Program<'info, System>,
}
