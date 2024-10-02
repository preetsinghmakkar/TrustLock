use anchor_lang::prelude::*;
use instructions::*;
use states::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;
declare_id!("9cZWC3nMxmkQGYw7JGMeaeaUmmQ3bfByaoZwvmyadvGx");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("58KP8zx3yTLwa7tvT2dJdiG7n7uQKgTGDvhr3fV21z8a");
    #[cfg(not(feature = "devnet"))]
    declare_id!("GWLpjnxVGQTBKHp9CD3rkgi76P2hxWoDkPTk5LBvtsDq");
}

#[program]
pub mod trust_lock {
    use super::*;

    // Initialized Configuration Account
    pub fn initialize_trustlock_configuration(
        _ctx: Context<InitializeTrustLockConfig>,
        _index: u8,
    ) -> Result<()> {
        instructions::initialize_trustlock_configuration(_ctx, _index)?;
        Ok(())
    }

    // Create My Account
    pub fn create_trustlock_account(_ctx: Context<CreateTrustLockAccount>) -> Result<()> {
        instructions::create_trustlock_account(_ctx)?;
        Ok(())
    }

    // Create Order
    pub fn create_order(
        _ctx: Context<CreateOrder>,
        _demand: String,
        _released_on: Option<u64>,
        _order_fulfiler: Option<Pubkey>,
    ) -> Result<()> {
        instructions::create_order(_ctx, _demand, _released_on, _order_fulfiler)?;
        Ok(())
    }

    // Admin Should create a Token Account in which user's are going to store their tokens.
    pub fn create_token_account(_ctx: Context<CreateToken>) -> Result<()> {
        instructions::create_token(_ctx)?;
        Ok(())
    }

    // Initialize Escrow Contract
    pub fn initalize_escrow_account_for_sol(_ctx: Context<InitializeEscrowContract>) -> Result<()> {
        instructions::initialize_escrow_contract(_ctx)?;
        Ok(())
    }
}
