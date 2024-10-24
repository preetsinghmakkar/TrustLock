use crate::{errors::ErrorCode, TrustLockConfig};
use anchor_lang::prelude::*;

use crate::{constants::*, CreateTrustLockAccountState};

pub fn create_trustlock_account(_ctx: Context<CreateTrustLockAccount>) -> Result<()> {
    msg!("TrustLock Account Creation Being Called");

    let create_trustlock_account = &mut _ctx.accounts.create_trustlock_account;
    let signer = &mut _ctx.accounts.signer.key();
    create_trustlock_account.holder = *signer;
    create_trustlock_account
        .account_no
        .checked_add(1)
        .ok_or_else(|| error!(ErrorCode::OverflowError))?;

    // Initialize opened_orders and my_pitches as empty vectors
    create_trustlock_account.my_opened_orders = Vec::new();
    create_trustlock_account.my_pitches = Vec::new();
    create_trustlock_account.contributions = Vec::new();

    msg!("Account Created for : {} ", signer);

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTrustLockAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub trust_lock_config_account: Box<Account<'info, TrustLockConfig>>,

    #[account(init, payer = signer, seeds=[INITIALIZE_TRUSTLOCK_ACCOUNT.as_ref(), signer.key().as_ref()], bump, space = CreateTrustLockAccountState::LEN)]
    pub create_trustlock_account: Account<'info, CreateTrustLockAccountState>,

    pub system_program: Program<'info, System>,
}
