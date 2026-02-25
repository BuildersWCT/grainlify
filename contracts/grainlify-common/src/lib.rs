#![no_std]
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // --- Generic/Shared (1-99) ---
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InsufficientFunds = 5,
    Overflow = 6,
    InvalidBatchSize = 7,
    BatchSizeMismatch = 8,
    LengthMismatch = 9,
    EmptyBatch = 10,
    InvalidDeadline = 11,
    DeadlineNotPassed = 12,
    CapabilityNotFound = 23,
    CapabilityExpired = 24,
    CapabilityRevoked = 25,
    CapabilityActionMismatch = 26,
    CapabilityAmountExceeded = 27,
    CapabilityUsesExhausted = 28,
    CapabilityExceedsAuthority = 29,

    // --- Governance (100-199) ---
    GovInvalidThreshold = 100,
    GovThresholdTooLow = 101,
    GovInsufficientStake = 102,
    GovProposalsNotFound = 103,
    GovProposalNotFound = 104,
    GovProposalNotActive = 105,
    GovVotingNotStarted = 106,
    GovVotingEnded = 107,
    GovVotingStillActive = 108,
    GovAlreadyVoted = 109,
    GovProposalNotApproved = 110,
    GovExecutionDelayNotMet = 111,
    GovProposalExpired = 112,

    // --- Bounty Escrow (200-299) ---
    BountyExists = 200,
    BountyNotFound = 201,
    BountyFundsNotLocked = 202,
    BountyInvalidFeeRate = 203,
    BountyFeeRecipientNotSet = 204,
    BountyDuplicateId = 205,
    BountyRefundNotApproved = 206,
    BountyFundsPaused = 207,
    BountyAmountBelowMinimum = 208,
    BountyAmountAboveMaximum = 209,
    BountyNotPaused = 210,
    BountyClaimPending = 211,

    // --- Program Escrow (300-399) ---
    ProgProgramAlreadyExists = 300,
    ProgDuplicateProgramId = 301,
    ProgProgramNotFound = 302,
    ProgFundsPaused = 303,
    ProgAlreadyReleased = 304,
    ProgNotYetDue = 305,
    ProgScheduleNotFound = 306,

    // --- Circuit Breaker / Error Recovery (400-499) ---
    CircuitOpen = 400,
    CircuitTransferFailed = 401,
    CircuitInsufficientBalance = 402,
}
