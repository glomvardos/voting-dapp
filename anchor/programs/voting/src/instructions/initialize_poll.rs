use anchor_lang::prelude::*;

use crate::{constants::*, state::*};

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
      init, 
      payer=signer, 
      space=ANCHOR_DISCRIMINATOR_SIZE + Poll::INIT_SPACE, 
      seeds=[poll_id.to_le_bytes().as_ref()], 
      bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>
}

pub fn initialize_poll_handler(ctx: Context<InitializePoll>, poll_id: u64, description: String, poll_start: u64, poll_end: u64) -> Result<()> {
  let poll = Poll::new(poll_id, description, poll_start, poll_end, 0);

  ctx.accounts.poll.set_inner(poll);
  
  Ok(())
}
