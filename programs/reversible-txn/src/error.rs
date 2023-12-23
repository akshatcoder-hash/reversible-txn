use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    /// voter has already cast their vote.
    #[msg("You have already voted.")]
    AlreadyVoted,

    /// voting period has expired.
    #[msg("The voting period has expired.")]
    VotingPeriodExpired,
}