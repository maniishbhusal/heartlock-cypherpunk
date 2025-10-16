# HeartLock - Consensual Commitment Escrow Architecture

## Overview
HeartLock is a Solana-based smart contract that enables two parties to create a joint commitment contract with scheduled deposits, optional yield generation, and consensual dispute resolution mechanisms.

## Core Components

### 1. Data Structures

#### CommitmentAccount
Main account storing all commitment details:
- `participants`: Array of 2 public keys (both partners)
- `start_time`: Unix timestamp when contract begins
- `end_time`: Unix timestamp when contract ends
- `contribution_amount`: Fixed amount each participant must contribute
- `contribution_frequency`: Weekly or Monthly
- `total_deposited`: Total amount deposited by both parties
- `yield_earned`: Total yield earned (if yield enabled)
- `status`: Active, Completed, Disputed, Breach Reported, Breach Validated
- `dispute_rules`: Description of dispute conditions
- `arbiters`: List of approved arbiters (public keys)
- `penalty_rules`: How penalties are distributed
- `yield_enabled`: Boolean indicating if funds are put to yield
- `yield_provider`: Address of yield protocol (if applicable)

#### DepositRecord
Tracking individual deposits:
- `participant`: Public key of depositor
- `amount`: Amount deposited
- `timestamp`: When deposit occurred
- `period`: Which period this deposit covers

#### DisputeRecord
Tracking dispute reports:
- `reporter`: Public key of person reporting breach
- `description`: Description of alleged breach
- `timestamp`: When dispute was reported
- `evidence`: Link to evidence (off-chain storage)
- `resolution_status`: Pending, Resolved, Rejected
- `resolver`: Arbiter who resolved dispute (if resolved)
- `resolution_notes`: Notes from arbiter

### 2. Contract States

1. **Initialized**: Contract created but not yet active
2. **Active**: Both parties have signed and deposits can be made
3. **Completed**: Term ended successfully, funds available for withdrawal
4. **Disputed**: Breach reported but not yet validated
5. **Breach Validated**: Arbiter has validated breach report
6. **Cancelled**: Contract cancelled by mutual agreement

### 3. Key Functions

#### Initialization Functions
- `initialize_commitment()`: Creates new commitment contract
- `sign_commitment()`: Second party signs to activate contract

#### Deposit Functions
- `make_deposit()`: Allows participants to make scheduled deposits
- `deposit_yield()`: Deposits funds to yield protocol (if enabled)

#### Dispute Functions
- `report_breach()`: Reports a potential breach of commitment
- `resolve_dispute()`: Arbiter validates/rejects breach claims
- `submit_evidence()`: Adds evidence to dispute record

#### Withdrawal Functions
- `withdraw_success()`: Withdraw funds after successful completion
- `withdraw_penalty()`: Withdraw funds after validated breach
- `emergency_withdraw()`: Withdraw in special circumstances (defined by contract)

### 4. Access Control

- Only contract participants can make deposits
- Only predefined arbiters can resolve disputes
- Withdrawals have specific eligibility requirements based on contract state
- Contract parameters are immutable after activation

### 5. Integration Points

#### Yield Protocols
Potential integration with:
- Solend
- Mango Markets
- Tulip Protocol
- Port Finance

#### Wallet Integration
- Phantom
- Solflare
- Backpack
- Other Solana wallets

#### Off-chain Storage
- Evidence documents
- Dispute descriptions
- User preferences