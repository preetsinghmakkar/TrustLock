use anchor_lang::prelude::*;

use crate::TrustLockConfig;

pub fn create_token(_ctx: Context<CreateToken>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub amm_config: Box<Account<'info, TrustLockConfig>>,
}
