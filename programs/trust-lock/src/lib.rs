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
        _release_on: Option<u64>,
        _order_fulfiler: Option<Pubkey>,
        _amount: u64,
    ) -> Result<()> {
        instructions::create_order(_ctx, _demand, _release_on, _order_fulfiler, _amount)?;
        Ok(())
    }

    // In this vault user will Store their Assets.
    // Ony Admin can initialize a vault for a particular token.
    pub fn create_vault(_ctx: Context<CreateVault>) -> Result<()> {
        instructions::create_vault(_ctx)?;
        Ok(())
    }

    //Pitch
    pub fn pitch_for_order(_ctx: Context<PitchForOrder>, order_id: u64) -> Result<()> {
        instructions::pitch_for_order(_ctx, order_id)?;
        Ok(())
    }

    pub fn choose_pitcher(
        _ctx: Context<ChoosePitcher>,
        _order_id: u64,
        _pitcher: Pubkey,
        release_on: Option<i64>,
    ) -> Result<()> {
        instructions::choose_pitcher(_ctx, _order_id, _pitcher, release_on)?;
        Ok(())
    }
}
