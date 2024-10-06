use crate::constants::{OrderStatus, INITIALIZE_TRUSTLOCK_ACCOUNT};
use crate::errors::ErrorCode;
use crate::CreateTrustLockAccountState;
use crate::{constants::CREATE_ORDER, CreateOrderAccount};
use anchor_lang::prelude::*;

pub fn pitch_for_order(_ctx: Context<PitchForOrder>, _order_id: u64) -> Result<()> {
    let signer = &mut _ctx.accounts.signer;
    let order = &mut _ctx.accounts.order;
    let trustlock_account = &mut _ctx.accounts.trustlock_account;

    trustlock_account.my_pitches.push(order.order_id);

    if order.pitchers.contains(&signer.key()) {
        return Err(error!(ErrorCode::AlreadyPitched));
    }

    if order.locked {
        return Err(error!(ErrorCode::OrderLocked));
    }

    order.pitchers.push(signer.key());

    order.order_status = OrderStatus::PITCHED;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_order_id : u64)]
pub struct PitchForOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[CREATE_ORDER.as_ref(), order.order_id.to_le_bytes().as_ref()], bump)]
    pub order: Box<Account<'info, CreateOrderAccount>>,

    #[account(mut, seeds=[INITIALIZE_TRUSTLOCK_ACCOUNT.as_ref(), signer.key().as_ref()], bump)]
    pub trustlock_account: Box<Account<'info, CreateTrustLockAccountState>>,

    pub system_account: Program<'info, System>,
}
