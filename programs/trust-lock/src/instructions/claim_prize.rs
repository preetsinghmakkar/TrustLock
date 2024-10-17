use crate::constants::{FulfillerStatus, OrderStatus, CREATE_ORDER};
use crate::errors::ErrorCode;
use crate::{CreateOrderAccount, UserAssetDetails};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

pub fn claim_prize(_ctx: Context<ClaimPrize>) -> Result<()> {
    let signer = &mut _ctx.accounts.signer;
    let order = &mut _ctx.accounts.order;
    let order_owner_asset_details = &mut _ctx.accounts.order_owner_asset_details;
    let token_vault_account = &mut _ctx.accounts.token_vault_account;
    let fulfiller_token_account = &mut _ctx.accounts.fulfiller_token_account;

    // Authorization checks
    if order.order_fulfiller != signer.key()
        || order.fulfiller_status != FulfillerStatus::FULFILLED
        || order.order_status != OrderStatus::FULFILLED
    {
        return Err(error!(ErrorCode::NotAuthorizedToClaimPrize));
    }

    // Find the TokenContribution matching the order_id
    let token_contribution = order_owner_asset_details
        .contributions
        .iter()
        .find(|contribution| contribution.order_id == order.order_id)
        .ok_or_else(|| error!(ErrorCode::ContributionNotFound))?;

    // Ensure the vault is correct
    require!(
        token_vault_account.key() == token_contribution.vault,
        ErrorCode::InvalidVaultAccount
    );

    // Transfer tokens from the vault to the fulfiller
    let cpi_accounts = Transfer {
        from: token_vault_account.to_account_info(),
        to: fulfiller_token_account.to_account_info(),
        authority: order_owner_asset_details.to_account_info(), // Assuming the vault is owned by the UserAssetDetails account
    };

    let cpi_program = _ctx.accounts.token_program.to_account_info();

    // Signer seeds for vault authority (if the vault is a PDA)
    let seeds = &[
        CREATE_ORDER.as_ref(),
        order.created_by.as_ref(),
        &[_ctx.bumps.order_owner_asset_details],
    ];

    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    token::transfer(cpi_ctx, token_contribution.amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [CREATE_ORDER.as_ref(), order.order_id.to_le_bytes().as_ref()],
        bump
    )]
    pub order: Box<Account<'info, CreateOrderAccount>>,

    #[account(
        mut,
        seeds = [CREATE_ORDER.as_ref(), order.created_by.as_ref()],
        bump
    )]
    pub order_owner_asset_details: Account<'info, UserAssetDetails>,

    #[account(mut)]
    pub token_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = fulfiller_token_account.mint == token_vault_account.mint,
        constraint = fulfiller_token_account.owner == signer.key()
    )]
    pub fulfiller_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
