use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(DESCRIPTION_MAX_LENGTH)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}

impl Poll {
    pub fn new(
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
        candidate_amount: u64,
    ) -> Self {
        Self {
            poll_id,
            description,
            poll_start,
            poll_end,
            candidate_amount,
        }
    }
}

#[account]
#[derive(InitSpace)]

pub struct Candidate {
    #[max_len(CANDIDATE_NAME_MAX_LENGTH)]
    pub candidate_name: String,

    pub candidate_votes: u64,
}

impl Candidate {
    pub fn new(candidate_name: String, candidate_votes: u64) -> Self {
        Self {
            candidate_name,
            candidate_votes,
        }
    }
}
