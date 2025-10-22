use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct UpdatePollDescription<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [poll_id.to_le_bytes().as_ref()], bump)]
    pub poll: Account<'info, Poll>,
}

pub fn update_poll_description_handler(
    ctx: Context<UpdatePollDescription>,
    description: String,
) -> Result<()> {
    ctx.accounts.poll.description = description;
    Ok(())
}
