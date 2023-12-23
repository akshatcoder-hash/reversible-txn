use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
pub mod error;
use error::ErrorCode;

declare_id!("5MEpbkjDofbmiQJ3TMXWGbYPXEmAArDphcMbDETvDMh1");

#[program]
pub mod reversible_txn {
    use super::*;

    pub fn initialize_voting(ctx: Context<InitializeVoting>, issue_timestamp: i64) -> Result<()> {
        let voting_state = &mut ctx.accounts.voting_state;
        voting_state.total_votes = 0;
        voting_state.issue_timestamp = issue_timestamp;
        voting_state.is_irreversible = false;
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, candidate_id: u64) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        let voting_state = &mut ctx.accounts.voting_state;

        let current_time = clock::Clock::get()?.unix_timestamp;
        if current_time - voting_state.issue_timestamp > 600 {
            voting_state.is_irreversible = true;
            return Err(ErrorCode::VotingPeriodExpired.into());
        }

        require!(!vote.is_voted, ErrorCode::AlreadyVoted);

        vote.voter = *ctx.accounts.voter.key;
        vote.candidate_id = candidate_id;
        vote.is_voted = true;
        voting_state.total_votes += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVoting<'info> {
    #[account(init, payer = user, space = 8 + 48)]
    pub voting_state: Account<'info, VotingState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub vote: Account<'info, Vote>,
    #[account(mut)]
    pub voting_state: Account<'info, VotingState>,
    #[account(mut)]
    pub voter: Signer<'info>,
}

#[account]
pub struct Vote {
    pub voter: Pubkey,
    pub candidate_id: u64,
    pub is_voted: bool,
}

#[account]
pub struct VotingState {
    pub total_votes: u64,
    pub issue_timestamp: i64,
    pub is_irreversible: bool,
}