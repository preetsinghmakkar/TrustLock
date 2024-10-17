use crate::constants::{FulfillerStatus, OrderStatus};
use crate::errors::ErrorCode;
use crate::{
    constants::*, CreateOrderAccount, CreateTrustLockAccountState, TrustLockConfig,
    UserAssetDetails,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub fn create_order(
    _ctx: Context<CreateOrder>,
    _demand: String,                 // Use Option for optional fields
    _order_fulfiler: Option<Pubkey>, // Option type to handle missing values
    _amount: u64,
) -> Result<()> {
    let create_order_account = &mut _ctx.accounts.create_order_account;
    let trustlock_config_account = &mut _ctx.accounts.trustlock_config_account;
    let user_token_account = &mut _ctx.accounts.user_token_account;
    let token_vault_account = &mut _ctx.accounts.token_vault_account;
    let user_asset_details = &mut _ctx.accounts.user_asset_details;
    let signer = &mut _ctx.accounts.signer;
    let trustlock_account = &mut _ctx.accounts.trustlock_account;

    if user_token_account.amount < _amount {
        return Err(error!(ErrorCode::InsufficientFunds));
    }

    create_order_account.order_id = trustlock_config_account.order_id;
    trustlock_config_account.order_id = trustlock_config_account
        .order_id
        .checked_add(1)
        .ok_or_else(|| error!(ErrorCode::OverflowError))?;

    create_order_account.demand = _demand;

    create_order_account.created_by = signer.key();

    create_order_account.amount = _amount;

    create_order_account.created_at = Clock::get()?.unix_timestamp;

    create_order_account.order_status = OrderStatus::CREATED;

    create_order_account.pitchers = Vec::new();

    create_order_account.order_fulfiller = _order_fulfiler.unwrap_or_else(Pubkey::default);

    create_order_account.fulfiller_status = FulfillerStatus::INACTIVE;

    // After Creating Order User should transfer money to the vault.

    // // Token transfer via CPI
    let cpi_accounts = Transfer {
        from: user_token_account.to_account_info(),
        to: token_vault_account.to_account_info(),
        authority: _ctx.accounts.signer.to_account_info(),
    };
    let cpi_program = _ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, _amount)?;

    // Record user's token contribution
    let token_mint = _ctx.accounts.token_mint.key(); // Get the mint key

    // Check if the user has already contributed for this token
    if let Some(contribution) = user_asset_details
        .contributions
        .iter_mut()
        .find(|c| c.mint == token_mint)
    {
        contribution.amount = contribution
            .amount
            .checked_add(_amount)
            .ok_or_else(|| error!(ErrorCode::OverflowError))?;
    } else {
        // Add a new contribution
        user_asset_details.contributions.push(TokenContribution {
            order_id: create_order_account.order_id,
            mint: token_mint,
            amount: _amount,
            vault: token_vault_account.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
    }

    trustlock_account
        .my_opened_orders
        .push(trustlock_config_account.order_id);

    Ok(())
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, seeds=[CREATE_ORDER.as_ref(), trustlock_config_account.order_id.to_le_bytes().as_ref()], bump, space=CreateOrderAccount::LEN)]
    pub create_order_account: Account<'info, CreateOrderAccount>,

    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)] // Ensure the vault already exists, pre-created by admin
    pub token_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds=[INTIALIZE_CONFIG.as_ref()], bump)]
    pub trustlock_config_account: Box<Account<'info, TrustLockConfig>>,

    #[account(init, payer = signer, seeds = [CREATE_ORDER.as_ref(), signer.key().as_ref()], bump, space = UserAssetDetails::LEN)]
    pub user_asset_details: Account<'info, UserAssetDetails>,

    #[account(mut, seeds=[INITIALIZE_TRUSTLOCK_ACCOUNT.as_ref(), signer.key().as_ref()], bump)]
    pub trustlock_account: Box<Account<'info, CreateTrustLockAccountState>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
