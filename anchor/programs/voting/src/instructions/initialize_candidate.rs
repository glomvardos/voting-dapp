use anchor_lang::prelude::*;

use crate::{constants::*, state::*};

#[derive(Accounts)]
#[instruction(candidate_name:String, poll_id:u64)]
pub struct InitializeCandidate<'info> {

  #[account(mut)]
  pub signer: Signer<'info>,

  #[account(
    mut,
    seeds=[poll_id.to_le_bytes().as_ref()], 
      bump  
    )]
    pub poll: Account<'info, Poll>,
  
  #[account( 
    init,
    payer = signer,
    space = ANCHOR_DISCRIMINATOR_SIZE + Candidate::INIT_SPACE,
    seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
    bump
  )]
  pub candidate: Account<'info, Candidate>,

  pub system_program: Program<'info, System>
}

pub fn initialize_candidate_handler(ctx: Context<InitializeCandidate>, candidate_name: String )-> Result<()> {

  ctx.accounts.poll.candidate_amount += 1;

  let candidate = Candidate::new(candidate_name, 0);
  ctx.accounts.candidate.set_inner(candidate);

  Ok(())
}
