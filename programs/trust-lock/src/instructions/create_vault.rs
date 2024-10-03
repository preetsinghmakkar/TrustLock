use anchor_lang::prelude::*;

pub fn create_vault(_ctx: Context<CreateVault>) -> Result<()> {
    // First Create a List of Mint Accounts That is they are supported or not.
    // That List can be updated by the Owner Only. (Include Bonk Token and Many More).

    Ok(())
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}
