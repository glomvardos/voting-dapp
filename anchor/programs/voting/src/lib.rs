#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>, poll_id: u64) -> Result<()> {
        Ok(())
    }
}
