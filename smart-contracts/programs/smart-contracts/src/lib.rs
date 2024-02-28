use anchor_lang::prelude::*;

declare_id!("GLxHTC5L9xNL34hGmpQWnLvi1kANyPJmL4FhdeNvmBQq");

#[program]
pub mod smart_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
