use anchor_lang::prelude::*;

use crate::{constants::CREATE_ORDER, CreateOrderAccount};

pub fn claim_prize(_ctx: Context<ClaimPrize>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[CREATE_ORDER.as_ref(), order.order_id.to_le_bytes().as_ref()], bump)]
    pub order: Box<Account<'info, CreateOrderAccount>>,

    pub system_program: Program<'info, System>,
}
