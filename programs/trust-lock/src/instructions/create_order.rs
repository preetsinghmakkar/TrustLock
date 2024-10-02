use crate::constants::{FulfillerStatus, OrderStatus};
use crate::{constants::*, CreateOrderAccount, TrustLockConfig};
use anchor_lang::prelude::*;

// pub fn create_order(
//     _ctx: Context<CreateOrder>,
//     _demand: String,
//     _released_on: u64,
//     _order_fulfiler: Pubkey,
// ) -> Result<()> {
//     let create_order_account = &mut _ctx.accounts.create_order_account;
//     let trustlock_config_account = &mut _ctx.accounts.trustlock_config_account;

//     create_order_account.order_id = trustlock_config_account.order_id;
//     trustlock_config_account.order_id += 1;

//     create_order_account.demand = _demand;

//     create_order_account.created_at = Clock::get()?.unix_timestamp;

//     if _released_on != 0 {
//         create_order_account.released_on = _released_on as i64;
//     } else {
//         create_order_account.released_on = 0;
//     }

//     create_order_account.order_status = OrderStatus::Pending;

//     create_order_account.pitchers = Vec::new();

//     if _order_fulfiler != Pubkey::default() {
//         create_order_account.order_fulfiller = _order_fulfiler;
//     } else {
//         create_order_account.order_fulfiller = Pubkey::default();
//     }

//     create_order_account.fulfiller_status = FulfillerStatus::Inactive;

//     Ok(())
// }

pub fn create_order(
    _ctx: Context<CreateOrder>,
    _demand: String,
    _released_on: Option<u64>,       // Use Option for optional fields
    _order_fulfiler: Option<Pubkey>, // Option type to handle missing values
) -> Result<()> {
    let create_order_account = &mut _ctx.accounts.create_order_account;
    let trustlock_config_account = &mut _ctx.accounts.trustlock_config_account;

    create_order_account.order_id = trustlock_config_account.order_id;
    trustlock_config_account.order_id += 1;

    create_order_account.demand = _demand;

    create_order_account.created_at = Clock::get()?.unix_timestamp;

    create_order_account.released_on = _released_on.map(|ro| ro as i64).unwrap_or(0);

    create_order_account.order_status = OrderStatus::Pending;

    create_order_account.pitchers = Vec::new();

    create_order_account.order_fulfiller = _order_fulfiler.unwrap_or_else(Pubkey::default);

    create_order_account.fulfiller_status = FulfillerStatus::Inactive;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, seeds=[CREATE_ORDER.as_ref(), signer.key().as_ref()], bump, space=CreateOrderAccount::LEN)]
    pub create_order_account: Account<'info, CreateOrderAccount>,

    #[account(mut, seeds=[INTIALIZE_CONFIG.as_ref()], bump)]
    pub trustlock_config_account: Box<Account<'info, TrustLockConfig>>,

    pub system_program: Program<'info, System>,
}
