use crate::constants::OrderStatus;
use crate::errors::ErrorCode;
use crate::{
    constants::{FulfillerStatus, CREATE_ORDER},
    CreateOrderAccount,
};
use anchor_lang::prelude::*;

pub fn order_review_by_owner(_ctx: Context<OrderReview>) -> Result<()> {
    let signer = &mut _ctx.accounts.signer;
    let order = &mut _ctx.accounts.order;

    if order.fulfiller_status != FulfillerStatus::FULFILLED && order.created_by != signer.key() {
        return Err(error!(ErrorCode::NotAuthorizedToReview));
    }

    order.order_status = OrderStatus::FULFILLED;

    msg!("Order ID : {} is Delivered To Owner. ", order.order_id);

    Ok(())
}

#[derive(Accounts)]
pub struct OrderReview<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[CREATE_ORDER.as_ref(), order.order_id.to_le_bytes().as_ref()], bump)]
    pub order: Box<Account<'info, CreateOrderAccount>>,

    pub system_program: Program<'info, System>,
}
