use crate::errors::ErrorCode;
use crate::{
    constants::{FulfillerStatus, OrderStatus, CREATE_ORDER},
    CreateOrderAccount,
};
use anchor_lang::prelude::*;

pub fn close_order(_ctx: Context<CloseOrder>) -> Result<()> {
    let signer = &mut _ctx.accounts.signer;
    let order = &mut _ctx.accounts.order;

    if order.created_by != signer.key() {
        return Err(error!(ErrorCode::NotAuthorizedToCloseOrder));
    }
    if order.fulfiller_status != FulfillerStatus::FULFILLED {
        return Err(error!(ErrorCode::FulfillerNotFulfilled));
    }

    if order.order_status != OrderStatus::FULFILLED {
        return Err(error!(ErrorCode::FulfillerNotFulfilled));
    }

    if order.order_status == OrderStatus::CLOSED {
        return Err(error!(ErrorCode::OrderAlreadyClosedOrFulfilled));
    }

    order.order_status = OrderStatus::CLOSED;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[CREATE_ORDER.as_ref(), order.order_id.to_le_bytes().as_ref()], bump)]
    pub order: Box<Account<'info, CreateOrderAccount>>,

    pub system_account: Program<'info, System>,
}
