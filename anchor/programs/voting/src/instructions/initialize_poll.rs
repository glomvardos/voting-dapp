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
