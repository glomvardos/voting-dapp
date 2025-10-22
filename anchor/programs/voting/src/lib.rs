#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("7v4H2hGM4kbpDbkvBu74dbFiwD29m2Sibm3cK3rcF4u1");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        initialize_poll_handler(ctx, poll_id, description, poll_start, poll_end)
    }

    pub fn update_poll_description(
        ctx: Context<UpdatePollDescription>,
        _poll_id: u64,
        description: String,
    ) -> Result<()> {
        update_poll_description_handler(ctx, description)
    }
}
