use crate::constants::CREATE_VAULT;
use crate::errors::ErrorCode;
use crate::{CreateVaultState, TrustLockConfig};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub fn create_vault(_ctx: Context<CreateVault>) -> Result<()> {
    let trustlock_config_account = &mut _ctx.accounts.trustlock_config_account;
    let create_vault_state = &mut _ctx.accounts.create_vault_state;
    let mint_account = &_ctx.accounts.token_mint;

    // Ensure the mint is supported
    let is_supported = trustlock_config_account.is_supported(mint_account)?;

    require!(is_supported, ErrorCode::TokenNotSupported);

    let bump = _ctx.bumps.create_vault_state;

    create_vault_state.initialize_vault(
        bump,
        _ctx.accounts.token_mint.as_ref(),
        _ctx.accounts.token_vault.key(),
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut, address = crate::admin::id() @ ErrorCode::NotApproved)]
    pub admin: Signer<'info>,

    // TrustLock config account
    #[account(mut)]
    pub trustlock_config_account: Box<Account<'info, TrustLockConfig>>,

    /// Token mint
    #[account(
        token::token_program = token_program
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token vault for the pool
    #[account(
        init,
        seeds = [
            CREATE_VAULT.as_ref(),
            create_vault_state.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        bump,
        payer = admin,
        token::mint = token_mint,
        token::authority = create_vault_state,
        token::token_program = token_program,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(init, payer=admin, seeds=[CREATE_VAULT.as_ref(), admin.key().as_ref()], bump, space=CreateVaultState::LEN)]
    pub create_vault_state: Account<'info, CreateVaultState>,

    /// SPL Token program (or Token 2022 program)
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
