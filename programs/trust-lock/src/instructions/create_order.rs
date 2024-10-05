use crate::constants::{FulfillerStatus, OrderStatus};
use crate::errors::ErrorCode;
use crate::{constants::*, CreateOrderAccount, TrustLockConfig, UserAssetDetails};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

pub fn create_order(
    _ctx: Context<CreateOrder>,
    _demand: String,
    _released_on: Option<u64>,       // Use Option for optional fields
    _order_fulfiler: Option<Pubkey>, // Option type to handle missing values
    _amount: u64,
) -> Result<()> {
    let create_order_account = &mut _ctx.accounts.create_order_account;
    let trustlock_config_account = &mut _ctx.accounts.trustlock_config_account;
    let user_token_account = &_ctx.accounts.user_token_account;
    let token_vault_account = &mut _ctx.accounts.token_vault_account;
    let user_asset_details = &mut _ctx.accounts.user_asset_details;

    if user_token_account.amount < _amount {
        return Err(error!(ErrorCode::InsufficientFunds));
    }

    create_order_account.order_id = trustlock_config_account.order_id;
    trustlock_config_account.order_id = trustlock_config_account
        .order_id
        .checked_add(1)
        .ok_or_else(|| error!(ErrorCode::OverflowError))?;

    create_order_account.demand = _demand;

    create_order_account.created_at = Clock::get()?.unix_timestamp;

    create_order_account.released_on = _released_on.map(|ro| ro as i64).unwrap_or(0);

    create_order_account.order_status = OrderStatus::Pending;

    create_order_account.pitchers = Vec::new();

    create_order_account.order_fulfiller = _order_fulfiler.unwrap_or_else(Pubkey::default);

    create_order_account.fulfiller_status = FulfillerStatus::Inactive;

    // After Creating Order User should transfer money to the vault.

    // Token transfer via CPI
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
        // Update existing contribution
        contribution.amount = contribution
            .amount
            .checked_add(_amount)
            .ok_or_else(|| error!(ErrorCode::OverflowError))?;
    } else {
        // Add a new contribution
        user_asset_details.contributions.push(TokenContribution {
            mint: token_mint,
            amount: _amount,
            vault: token_vault_account.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, seeds=[CREATE_ORDER.as_ref(), signer.key().as_ref()], bump, space=CreateOrderAccount::LEN)]
    pub create_order_account: Account<'info, CreateOrderAccount>,

    #[account(mut)]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub token_vault_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, seeds=[INTIALIZE_CONFIG.as_ref()], bump)]
    pub trustlock_config_account: Box<Account<'info, TrustLockConfig>>,

    #[account(init, payer = signer, seeds = [CREATE_ORDER.as_ref(), signer.key().as_ref()], bump, space = UserAssetDetails::LEN)]
    pub user_asset_details: Account<'info, UserAssetDetails>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
