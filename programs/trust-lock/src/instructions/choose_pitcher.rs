use crate::constants::FulfillerStatus;
use crate::errors::ErrorCode;

use crate::{
    constants::{CREATE_ORDER, INTIALIZE_CONFIG},
    CreateOrderAccount, TrustLockConfig,
};
use anchor_lang::prelude::*;

pub fn choose_pitcher(_ctx: Context<ChoosePitcher>, _index: u8, _pitcher: Pubkey) -> Result<()> {
    let order = &mut _ctx.accounts.order;
    let signer = &mut _ctx.accounts.signer;

    if order.created_by != signer.key() {
        return Err(error!(ErrorCode::NotValidOwner));
    }

    order.order_fulfiller = _pitcher;
    order.fulfiller_status = FulfillerStatus::PROCESSING;

    order.locked = true;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_index : u8)]
pub struct ChoosePitcher<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[CREATE_ORDER.as_ref(), order.order_id.to_le_bytes().as_ref()], bump)]
    pub order: Box<Account<'info, CreateOrderAccount>>,

    #[account(mut, seeds=[INTIALIZE_CONFIG.as_ref(), &_index.to_be_bytes()], bump)]
    pub trustlock_config_account: Box<Account<'info, TrustLockConfig>>,

    pub system_program: Program<'info, System>,
}
