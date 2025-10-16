# HeartLock Smart Contract Specification

## Overview
This document details the implementation requirements for the HeartLock Solana smart contract using the Anchor framework.

## Account Structures

### CommitmentAccount
```rust
#[account]
pub struct CommitmentAccount {
    /// Both partners in the commitment
    pub participants: [Pubkey; 2],
    
    /// Partner who initiated the contract
    pub creator: Pubkey,
    
    /// Timestamp when contract was created
    pub created_at: i64,
    
    /// Timestamp when both parties signed
    pub activated_at: i64,
    
    /// Start of commitment period
    pub start_time: i64,
    
    /// End of commitment period
    pub end_time: i64,
    
    /// Fixed amount each participant must contribute per period
    pub contribution_amount: u64,
    
    /// Frequency of contributions (weekly/monthly)
    pub contribution_frequency: ContributionFrequency,
    
    /// Total amount deposited by both parties
    pub total_deposited: u64,
    
    /// Total yield earned (if yield enabled)
    pub yield_earned: u64,
    
    /// Current status of the commitment
    pub status: CommitmentStatus,
    
    /// Description of conditions that constitute a breach
    pub dispute_rules: String, // Max 200 chars
    
    /// Approved arbiters who can resolve disputes
    pub arbiters: [Pubkey; 3], // Up to 3 arbiters
    
    /// How penalties are distributed
    pub penalty_rules: PenaltyRules,
    
    /// Whether funds are put to yield
    pub yield_enabled: bool,
    
    /// Yield provider protocol (if applicable)
    pub yield_provider: Option<Pubkey>,
    
    /// Bump seed for PDA
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ContributionFrequency {
    Weekly,
    Monthly,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CommitmentStatus {
    Initialized,
    Active,
    Completed,
    Disputed,
    BreachValidated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PenaltyRules {
    /// Other partner gets everything
    WinnerTakesAll,
    /// Split evenly between partner and charity
    SplitWithCharity { charity: Pubkey, split_percentage: u8 },
    /// Other partner gets specified percentage
    PercentageToPartner { percentage: u8 },
}
```

### DepositRecord
```rust
#[account]
pub struct DepositRecord {
    /// The commitment this deposit is for
    pub commitment: Pubkey,
    
    /// Who made the deposit
    pub participant: Pubkey,
    
    /// Amount deposited
    pub amount: u64,
    
    /// When deposit occurred
    pub timestamp: i64,
    
    /// Which period this covers
    pub period: u32,
    
    /// Bump seed for PDA
    pub bump: u8,
}
```

### DisputeRecord
```rust
#[account]
pub struct DisputeRecord {
    /// The commitment in dispute
    pub commitment: Pubkey,
    
    /// Who reported the breach
    pub reporter: Pubkey,
    
    /// Description of alleged breach
    pub description: String, // Max 500 chars
    
    /// When dispute was reported
    pub timestamp: i64,
    
    /// Link to evidence (off-chain storage)
    pub evidence_link: String, // Max 200 chars
    
    /// Current resolution status
    pub resolution_status: DisputeResolutionStatus,
    
    /// Arbiter who resolved dispute (if resolved)
    pub resolver: Option<Pubkey>,
    
    /// Notes from arbiter
    pub resolution_notes: String, // Max 500 chars
    
    /// Bump seed for PDA
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum DisputeResolutionStatus {
    Pending,
    Resolved,
    Rejected,
}
```

## Instruction Definitions

### initialize_commitment
Creates a new commitment contract.

**Accounts:**
- `[signer] creator` - The person creating the commitment
- `[writable] commitment_account` - PDA for commitment storage
- `system_program` - Required for account creation

**Arguments:**
- `participants`: [Pubkey; 2] - Both partners (creator must be first)
- `start_time`: i64 - When commitment begins
- `duration_seconds`: i64 - Length of commitment in seconds
- `contribution_amount`: u64 - Amount per contribution period
- `contribution_frequency`: ContributionFrequency - Weekly or monthly
- `dispute_rules`: String - Conditions that constitute breach
- `arbiters`: [Pubkey; 3] - Approved arbiters
- `penalty_rules`: PenaltyRules - How penalties are distributed
- `yield_enabled`: bool - Whether to put funds to yield

### sign_commitment
Second partner signs to activate the contract.

**Accounts:**
- `[signer] signer` - The second partner
- `[writable] commitment_account` - The commitment to activate

### make_deposit
Allows participants to make scheduled deposits.

**Accounts:**
- `[signer] participant` - Person making deposit
- `[writable] commitment_account` - The commitment
- `[writable] deposit_record` - Record of this deposit
- `[writable] participant_token_account` - Participant's token account
- `[writable] commitment_vault` - Vault holding commitment funds
- `token_program` - SPL Token program

**Arguments:**
- `period`: u32 - Which period this deposit covers

### report_breach
Reports a potential breach of commitment.

**Accounts:**
- `[signer] reporter` - Person reporting breach
- `[writable] commitment_account` - The commitment in question
- `[writable] dispute_record` - Record of this dispute
- `system_program` - Required for account creation

**Arguments:**
- `description`: String - Description of alleged breach
- `evidence_link`: String - Link to evidence

### resolve_dispute
Arbiter validates or rejects breach claims.

**Accounts:**
- `[signer] arbiter` - Approved arbiter
- `[writable] commitment_account` - The commitment
- `[writable] dispute_record` - Record to update
- `[writable] vault_account` - Vault with funds
- `[writable] recipient_account` - Account to receive penalty (varies by penalty rules)
- `token_program` - SPL Token program

**Arguments:**
- `resolution`: bool - True if breach is valid
- `notes`: String - Arbiter's notes

### withdraw_success
Withdraw funds after successful completion.

**Accounts:**
- `[signer] participant` - Person withdrawing
- `[writable] commitment_account` - The commitment
- `[writable] vault_account` - Vault with funds
- `[writable] participant_account` - Participant's receiving account
- `token_program` - SPL Token program

**Arguments:**
- `amount`: u64 - Amount to withdraw

### withdraw_penalty
Withdraw funds after validated breach.

**Accounts:**
- `[signer] winner` - Person entitled to penalty
- `[writable] commitment_account` - The commitment
- `[writable] vault_account` - Vault with funds
- `[writable] recipient_account` - Winner's receiving account
- `token_program` - SPL Token program

## Constants and Constraints

### Space Calculations
```rust
// CommitmentAccount space calculation
const COMMITMENT_ACCOUNT_SIZE: usize = 8 +  // Discriminator
    32 * 2 +  // participants
    32 +      // creator
    8 +       // created_at
    8 +       // activated_at
    8 +       // start_time
    8 +       // end_time
    8 +       // contribution_amount
    1 +       // contribution_frequency
    8 +       // total_deposited
    8 +       // yield_earned
    1 +       // status
    200 +     // dispute_rules (max)
    32 * 3 +  // arbiters
    1 +       // penalty_rules variant
    32 + 1 +  // PenaltyRules::SplitWithCharity (charity + percentage)
    1 +       // PenaltyRules::PercentageToPartner (percentage)
    1 +       // yield_enabled
    32 +      // yield_provider (Option)
    1;        // bump

// DepositRecord space calculation
const DEPOSIT_RECORD_SIZE: usize = 8 +  // Discriminator
    32 +  // commitment
    32 +  // participant
    8 +   // amount
    8 +   // timestamp
    4 +   // period
    1;    // bump

// DisputeRecord space calculation
const DISPUTE_RECORD_SIZE: usize = 8 +  // Discriminator
    32 +  // commitment
    32 +  // reporter
    500 + // description (max)
    8 +   // timestamp
    200 + // evidence_link (max)
    1 +   // resolution_status
    32 +  // resolver (Option)
    500 + // resolution_notes (max)
    1;    // bump
```

### Seeds for PDAs
```rust
// Commitment account seeds
&[b"commitment", creator.key().as_ref(), &[bump]]

// Deposit record seeds
&[b"deposit", commitment.key().as_ref(), participant.key().as_ref(), &period.to_le_bytes(), &[bump]]

// Dispute record seeds
&[b"dispute", commitment.key().as_ref(), reporter.key().as_ref(), &[bump]]
```

## Error Handling

```rust
#[error_code]
pub enum HeartLockError {
    #[msg("Invalid participant")]
    InvalidParticipant,
    
    #[msg("Contract not active")]
    NotActive,
    
    #[msg("Contract already active")]
    AlreadyActive,
    
    #[msg("Invalid time period")]
    InvalidTimePeriod,
    
    #[msg("Incorrect contribution amount")]
    IncorrectAmount,
    
    #[msg("Deposit period already covered")]
    PeriodAlreadyDeposited,
    
    #[msg("Contract not completed")]
    NotCompleted,
    
    #[msg("Contract not in dispute")]
    NotDisputed,
    
    #[msg("Unauthorized arbiter")]
    UnauthorizedArbiter,
    
    #[msg("Dispute already resolved")]
    DisputeAlreadyResolved,
    
    #[msg("Invalid penalty rules")]
    InvalidPenaltyRules,
    
    #[msg("Unauthorized withdrawal")]
    UnauthorizedWithdrawal,
}
```

## Validation Requirements

1. **Initialization Validation:**
   - Creator must be one of the participants
   - Start time must be in the future
   - Duration must be positive
   - Contribution amount must be greater than 0
   - At least one arbiter must be provided
   - Dispute rules must not be empty

2. **Signing Validation:**
   - Signer must be the second participant
   - Contract must be in Initialized state
   - Current time must be before start time

3. **Deposit Validation:**
   - Participant must be one of the contract participants
   - Contract must be Active
   - Amount must match required contribution amount
   - Period must be current or future
   - Period must not already have a deposit

4. **Dispute Validation:**
   - Reporter must be a participant
   - Contract must be Active
   - Description must not be empty

5. **Resolution Validation:**
   - Resolver must be an approved arbiter
   - Dispute must be in Pending state

6. **Withdrawal Validation:**
   - For success withdrawal: contract must be Completed
   - For penalty withdrawal: contract must be BreachValidated
   - Requester must be authorized per penalty rules