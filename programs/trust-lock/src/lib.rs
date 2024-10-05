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

    //Update Allowed Tokens WhiteList
    pub fn update_whitelist(
        _ctx: Context<UpdateWhitelist>,
        new_whitelist: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::update_whitelist(_ctx, new_whitelist)?;
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
        _amount: u64,
    ) -> Result<()> {
        instructions::create_order(_ctx, _demand, _released_on, _order_fulfiler, _amount)?;
        Ok(())
    }

    pub fn create_vault(_ctx: Context<CreateVault>) -> Result<()> {
        instructions::create_vault(_ctx)?;
        Ok(())
    }
}
