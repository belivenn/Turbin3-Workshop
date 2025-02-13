use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;
mod constants;

declare_id!("6pL4yVPhisvMuWCpPBCWtaUeLHeW7pQZigQB9qmTydiR");

#[program]
pub mod workshop_raydium_cpi {
    use super::*;

    pub fn create_cpmm_pool(
        ctx: Context<CreateCpmmPool>,
        funding_amount: Option<u64>,
    ) -> Result<()> {
        ctx.accounts.issue_tokens()?;
        ctx.accounts.revoke_mint_authority()?;
        ctx.accounts.create_cpmm_pool(funding_amount)
    }
    pub fn lock_cpmm_liquidity(ctx: Context<LockCpmmLiquidity>) -> Result<()> {
        ctx.accounts.lock_cpmm_cpi()
    }
    pub fn harvest_locked_liquidity(ctx: Context<HarvestLockedLiquidity>) -> Result<()> {
        ctx.accounts.harvest_cp_fees_cpi()
    }
    pub fn swap(ctx: Context<Swap>, amount_in: u64, minimum_amount_out: u64) -> Result<()> {
        ctx.accounts.swap(amount_in, minimum_amount_out)
    }
}
