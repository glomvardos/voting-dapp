#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod voting {
    use crate::state::Poll;

    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        let poll = Poll::new(poll_id, description, poll_start, poll_end, 0);

        ctx.accounts.poll.set_inner(poll);

        Ok(())
    }
}
