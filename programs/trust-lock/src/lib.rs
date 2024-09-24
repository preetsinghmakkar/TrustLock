use anchor_lang::prelude::*;

declare_id!("9cZWC3nMxmkQGYw7JGMeaeaUmmQ3bfByaoZwvmyadvGx");

#[program]
pub mod trust_lock {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
