use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct Vote<'info> {
    #[account()]
    pub signer: Signer<'info>,

    #[account(mut, seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()], bump)]
    pub candidate: Account<'info, Candidate>,
}

pub fn vote_handler(ctx: Context<Vote>) -> Result<()> {
    ctx.accounts.candidate.candidate_votes += 1;
    msg!("{}", ctx.accounts.candidate.candidate_votes);
    Ok(())
}
