# HeartLock System Architecture

## Component Diagram

```mermaid
graph TD
    A[User Interface] --> B[React Frontend]
    B --> C[Solana Wallet Adapter]
    C --> D[(Solana Blockchain)]
    B --> E[API Layer]
    E --> F[HeartLock Smart Contract]
    F --> D
    F --> G[Yield Protocols]
    G --> D
    H[Arbiters] --> I[Dispute Resolution Module]
    I --> F
    J[Off-chain Storage] --> K[Evidence/Documents]
    K --> I
    
    style A fill:#f9f,stroke:#333
    style B fill:#bbf,stroke:#333
    style F fill:#bfb,stroke:#333
    style D fill:#fbb,stroke:#333
```

## Data Flow Diagram

```mermaid
sequenceDiagram
    participant U1 as User 1
    participant U2 as User 2
    participant F as Frontend
    participant W as Wallet
    participant S as Solana
    participant C as Contract
    participant Y as Yield Protocol
    
    U1->>F: Create Commitment
    F->>W: Sign Transaction
    W->>S: Send Transaction
    S->>C: Initialize Contract
    C-->>S: Store Account
    S-->>F: Confirmation
    F-->>U1: Contract Created
    
    U2->>F: Sign Commitment
    F->>W: Sign Transaction
    W->>S: Send Transaction
    S->>C: Activate Contract
    C-->>S: Update State
    S-->>F: Confirmation
    F-->>U2: Contract Active
    
    loop Scheduled Deposits
        U1->>F: Make Deposit
        F->>W: Sign Transaction
        W->>S: Send Transaction
        S->>C: Process Deposit
        C->>Y: Deposit to Yield
        Y-->>S: Tokens Staked
        C-->>S: Update Records
        S-->>F: Confirmation
        F-->>U1: Deposit Confirmed
    end
    
    U1->>F: Report Breach
    F->>W: Sign Transaction
    W->>S: Send Transaction
    S->>C: Record Dispute
    C-->>S: Update State
    S-->>F: Confirmation
    F-->>U1: Dispute Filed
    
    H->>F: Resolve Dispute
    F->>W: Sign Transaction
    W->>S: Send Transaction
    S->>C: Apply Penalty
    C-->>S: Update State
    S-->>F: Confirmation
    F-->>H: Resolution Recorded
    
    U2->>F: Withdraw Funds
    F->>W: Sign Transaction
    W->>S: Send Transaction
    S->>C: Process Withdrawal
    C-->>S: Transfer Tokens
    S-->>W: Send to Wallet
    W-->>U2: Funds Received
```

## Smart Contract State Machine

```mermaid
stateDiagram-v2
    [*] --> Initialized: Create Contract
    Initialized --> Active: Both Sign
    Active --> Active: Deposits Made
    Active --> Completed: Term Ends
    Active --> Disputed: Breach Reported
    Disputed --> BreachValidated: Arbiter Confirms
    Disputed --> Active: Arbiter Rejects
    BreachValidated --> [*]: Penalty Applied
    Completed --> [*]: Successful Withdrawal