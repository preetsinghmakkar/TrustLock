use crate::constants::{
    FulfillerStatus, OrderStatus, CREATE_ORDER, CREATE_VAULT,
    INITIALIZE_TRUSTLOCK_ACCOUNT,
};
use crate::errors::ErrorCode;
use crate::{CreateOrderAccount, CreateTrustLockAccountState};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub fn claim_prize(_ctx: Context<ClaimPrize>) -> Result<()> {
    let signer = &mut _ctx.accounts.signer;
    let order = &mut _ctx.accounts.order;
    let trust_lock_account = &mut _ctx.accounts.trustlock_account;
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
    let token_contribution = trust_lock_account
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
        authority: token_vault_account.to_account_info(),
    };

    let cpi_program = _ctx.accounts.token_program.to_account_info();

    let binding = token_vault_account.mint.key();
    let seeds = &[
        CREATE_VAULT.as_ref(),
        binding.as_ref(),
        &[_ctx.bumps.token_vault_account],
    ];
    let signer_seeds = &[&seeds[..]];

    // Log seeds for debugging
    msg!("Seeds in Claim Prize : {:?}", seeds);
    msg!("Signer Seeds in Claim Prize : {:?}", signer_seeds);

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
        seeds = [
            CREATE_VAULT.as_ref(),
            token_vault_account.mint.key().as_ref() 
        ],
        bump
    )]
    pub token_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds=[INITIALIZE_TRUSTLOCK_ACCOUNT.as_ref(), signer.key().as_ref()], bump)]
    pub trustlock_account: Box<Account<'info, CreateTrustLockAccountState>>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = fulfiller_token_account.mint == token_vault_account.mint,
        constraint = fulfiller_token_account.owner == signer.key()
    )]
    pub fulfiller_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
