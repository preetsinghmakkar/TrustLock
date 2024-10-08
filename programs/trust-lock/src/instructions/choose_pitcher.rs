use crate::constants::FulfillerStatus;
use crate::errors::ErrorCode;

use crate::{
    constants::{CREATE_ORDER, INTIALIZE_CONFIG},
    CreateOrderAccount, TrustLockConfig,
};
use anchor_lang::prelude::*;

pub fn choose_pitcher(
    _ctx: Context<ChoosePitcher>,
    _order_id: u64,
    _pitcher: Pubkey,
    _release_on: Option<i64>,
) -> Result<()> {
    let order = &mut _ctx.accounts.order;
    let signer = &mut _ctx.accounts.signer;

    if order.created_by != signer.key() {
        return Err(error!(ErrorCode::NotValidOwner));
    }

    // Check if release_on is provided and compare timestamps if so
    if let Some(release_on_time) = _release_on {
        if order.created_at > release_on_time {
            return Err(error!(ErrorCode::InvalidReleaseTime));
        }
    }

    order.order_fulfiller = _pitcher;
    order.fulfiller_status = FulfillerStatus::PROCESSING;

    order.locked = true;

    Ok(())
}

#[derive(Accounts)]
pub struct ChoosePitcher<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[CREATE_ORDER.as_ref(), trustlock_config_account.order_id.to_le_bytes().as_ref()], bump)]
    pub order: Box<Account<'info, CreateOrderAccount>>,

    #[account(mut, seeds=[INTIALIZE_CONFIG.as_ref()], bump)]
    pub trustlock_config_account: Box<Account<'info, TrustLockConfig>>,

    pub system_program: Program<'info, System>,
}
