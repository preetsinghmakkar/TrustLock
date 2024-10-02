use anchor_lang::prelude::*;

pub fn initialize_escrow_contract(_ctx: Context<InitializeEscrowContract>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeEscrowContract<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}
