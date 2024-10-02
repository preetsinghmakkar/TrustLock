use anchor_lang::prelude::*;

use crate::{constants::*, CreateTrustLockAccountState};

pub fn create_trustlock_account(_ctx: Context<CreateTrustLockAccount>) -> Result<()> {
    msg!("Creating Your Bank");

    let create_trustlock_account = &mut _ctx.accounts.create_trustlock_account;
    create_trustlock_account.holder = _ctx.accounts.signer.key();
    create_trustlock_account.account_no += 1;

    // Initialize opened_orders and my_pitches as empty vectors

    create_trustlock_account.opened_orders = Vec::new();
    create_trustlock_account.my_pitches = Vec::new();

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTrustLockAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, seeds=[INITIALIZE_TRUSTLOCK_ACCOUNT.as_ref(), signer.key().as_ref()], bump, space = CreateTrustLockAccountState::LEN)]
    pub create_trustlock_account: Account<'info, CreateTrustLockAccountState>,

    pub system_program: Program<'info, System>,
}
