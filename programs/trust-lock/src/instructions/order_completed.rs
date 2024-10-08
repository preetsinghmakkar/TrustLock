use crate::constants::FulfillerStatus;
use crate::errors::ErrorCode;
use crate::{constants::CREATE_ORDER, CreateOrderAccount};
use anchor_lang::prelude::*;

pub fn order_completed(_ctx: Context<OrderCompleted>) -> Result<()> {
    let signer = &mut _ctx.accounts.signer;
    let order = &mut _ctx.accounts.order;

    if order.order_fulfiller != signer.key() {
        return Err(error!(ErrorCode::WrongFulfiller));
    }

    order.fulfiller_status = FulfillerStatus::FULFILLED;

    msg!(
        "Order Id : {} , Order Status : {:?}",
        order.order_id,
        order.order_status
    );

    Ok(())
}

#[derive(Accounts)]
pub struct OrderCompleted<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[CREATE_ORDER.as_ref(), order.order_id.to_le_bytes().as_ref()], bump)]
    pub order: Box<Account<'info, CreateOrderAccount>>,

    pub system_program: Program<'info, System>,
}
