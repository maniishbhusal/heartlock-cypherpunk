# HeartLock Frontend Specification

## Overview
This document details the implementation requirements for the HeartLock React frontend application.

## Technology Stack
- **Framework**: React 18+ with Hooks
- **State Management**: React Context API
- **Wallet Integration**: @solana/wallet-adapter
- **Styling**: TailwindCSS
- **Build Tool**: Vite
- **Language**: TypeScript

## Project Structure
```
app/
├── src/
│   ├── components/
│   │   ├── common/           # Shared components
│   │   ├── layout/           # Layout components
│   │   ├── commitment/       # Commitment-related components
│   │   ├── deposit/          # Deposit-related components
│   │   ├── dispute/          # Dispute-related components
│   │   └── withdrawal/       # Withdrawal-related components
│   ├── hooks/                # Custom hooks
│   ├── contexts/             # React contexts
│   ├── utils/                # Utility functions
│   ├── services/             # API/services integration
│   ├── pages/                # Page components
│   ├── types/                # TypeScript types
│   └── App.tsx               # Main app component
├── public/                   # Static assets
└── index.html                # Entry point
```

## Core Components

### 1. Wallet Connection
```tsx
// src/components/common/WalletConnection.tsx
interface WalletConnectionProps {
  onConnect?: () => void;
  onDisconnect?: () => void;
}

const WalletConnection: React.FC<WalletConnectionProps> = ({ 
  onConnect, 
  onDisconnect 
}) => {
  // Implementation for wallet connection using @solana/wallet-adapter
};
```

### 2. Commitment Creation Form
```tsx
// src/components/commitment/CreateCommitmentForm.tsx
interface CreateCommitmentFormProps {
  onSubmit: (data: CommitmentData) => void;
  isLoading: boolean;
}

interface CommitmentData {
  partner: string; // Public key
  startTime: Date;
  duration: number; // In days
  contributionAmount: number;
  contributionFrequency: 'weekly' | 'monthly';
  disputeRules: string;
  arbiters: string[]; // Public keys (1-3)
  penaltyType: 'winner-takes-all' | 'split-with-charity' | 'percentage';
  charityAddress?: string;
  splitPercentage?: number;
  percentageToPartner?: number;
  yieldEnabled: boolean;
}

const CreateCommitmentForm: React.FC<CreateCommitmentFormProps> = ({
  onSubmit,
  isLoading
}) => {
  // Form implementation with validation
};
```

### 3. Commitment Dashboard
```tsx
// src/components/commitment/CommitmentDashboard.tsx
interface CommitmentDashboardProps {
  commitment: CommitmentAccount;
  userRole: 'creator' | 'partner' | 'arbiter';
}

interface CommitmentAccount {
  id: string;
  participants: [string, string];
  createdAt: Date;
  activatedAt: Date | null;
  startTime: Date;
  endTime: Date;
  contributionAmount: number;
  contributionFrequency: 'weekly' | 'monthly';
  totalDeposited: number;
  yieldEarned: number;
  status: 'initialized' | 'active' | 'completed' | 'disputed' | 'breach-validated';
  disputeRules: string;
  arbiters: string[];
  penaltyRules: PenaltyRules;
  yieldEnabled: boolean;
}

const CommitmentDashboard: React.FC<CommitmentDashboardProps> = ({
  commitment,
  userRole
}) => {
  // Dashboard showing commitment details, timeline, deposits, etc.
};
```

### 4. Deposit Interface
```tsx
// src/components/deposit/DepositInterface.tsx
interface DepositInterfaceProps {
  commitment: CommitmentAccount;
  onDeposit: (period: number, amount: number) => void;
  isLoading: boolean;
}

interface DepositHistoryItem {
  id: string;
  participant: string;
  amount: number;
  timestamp: Date;
  period: number;
}

const DepositInterface: React.FC<DepositInterfaceProps> = ({
  commitment,
  onDeposit,
  isLoading
}) => {
  // Interface for making deposits and viewing deposit history
};
```

### 5. Dispute Reporting
```tsx
// src/components/dispute/DisputeReporting.tsx
interface DisputeReportingProps {
  commitment: CommitmentAccount;
  onReport: (data: DisputeData) => void;
  isLoading: boolean;
}

interface DisputeData {
  description: string;
  evidenceLink: string;
}

const DisputeReporting: React.FC<DisputeReportingProps> = ({
  commitment,
  onReport,
  isLoading
}) => {
  // Form for reporting breaches and submitting evidence
};
```

### 6. Arbiter Resolution Panel
```tsx
// src/components/dispute/ArbiterResolutionPanel.tsx
interface ArbiterResolutionPanelProps {
  dispute: DisputeRecord;
  commitment: CommitmentAccount;
  onResolve: (isValid: boolean, notes: string) => void;
  isLoading: boolean;
}

interface DisputeRecord {
  id: string;
  commitmentId: string;
  reporter: string;
  description: string;
  evidenceLink: string;
  timestamp: Date;
  resolutionStatus: 'pending' | 'resolved' | 'rejected';
  resolver: string | null;
  resolutionNotes: string | null;
}

const ArbiterResolutionPanel: React.FC<ArbiterResolutionPanelProps> = ({
  dispute,
  commitment,
  onResolve,
  isLoading
}) => {
  // Panel for arbiters to review disputes and make resolutions
};
```

### 7. Withdrawal Interface
```tsx
// src/components/withdrawal/WithdrawalInterface.tsx
interface WithdrawalInterfaceProps {
  commitment: CommitmentAccount;
  userPublicKey: string;
  onWithdraw: (amount: number) => void;
  isLoading: boolean;
}

const WithdrawalInterface: React.FC<WithdrawalInterfaceProps> = ({
  commitment,
  userPublicKey,
  onWithdraw,
  isLoading
}) => {
  // Interface for withdrawing funds based on contract status
};
```

## Pages

### 1. Home Page
- Wallet connection
- Brief introduction to HeartLock
- Call-to-action buttons (Create Commitment, View My Commitments)

### 2. Create Commitment Page
- Commitment creation form
- Preview of commitment terms
- Transaction confirmation

### 3. Commitment Detail Page
- Commitment dashboard
- Deposit interface
- Dispute reporting (if applicable)
- Withdrawal interface (if eligible)

### 4. My Commitments Page
- List of commitments user is involved in
- Filter by role (creator, partner, arbiter)
- Quick actions for each commitment

### 5. Arbiter Dashboard Page
- List of disputes requiring resolution
- Quick access to resolution panels

## State Management

### Global Contexts
```tsx
// src/contexts/WalletContext.tsx
interface WalletContextType {
  connected: boolean;
  publicKey: string | null;
  connect: () => void;
  disconnect: () => void;
}

// src/contexts/CommitmentContext.tsx
interface CommitmentContextType {
  commitments: CommitmentAccount[];
  fetchCommitments: () => Promise<void>;
  createCommitment: (data: CommitmentData) => Promise<string>; // Returns commitment ID
  refreshCommitment: (id: string) => Promise<void>;
}
```

## Services Layer

### Solana Service
```tsx
// src/services/solana.ts
class SolanaService {
  // Methods for interacting with smart contracts
  async initializeCommitment(data: CommitmentData): Promise<string> {}
  async signCommitment(commitmentId: string): Promise<boolean> {}
  async makeDeposit(commitmentId: string, period: number, amount: number): Promise<boolean> {}
  async reportBreach(commitmentId: string, data: DisputeData): Promise<boolean> {}
  async resolveDispute(disputeId: string, isValid: boolean, notes: string): Promise<boolean> {}
  async withdrawSuccess(commitmentId: string, amount: number): Promise<boolean> {}
  async withdrawPenalty(commitmentId: string): Promise<boolean> {}
}
```

### Commitment Service
```tsx
// src/services/commitment.ts
class CommitmentService {
  // Methods for fetching and managing commitment data
  async getCommitment(id: string): Promise<CommitmentAccount> {}
  async getUserCommitments(userPublicKey: string): Promise<CommitmentAccount[]> {}
  async getPendingDisputes(arbiterPublicKey: string): Promise<DisputeRecord[]> {}
}
```

## Responsive Design Requirements

### Breakpoints
- Mobile: 0px - 768px
- Tablet: 769px - 1024px
- Desktop: 1025px+

### Key Responsive Considerations
1. **Navigation**: Hamburger menu on mobile
2. **Forms**: Vertical layout on mobile, horizontal on desktop
3. **Tables/Dashboards**: Scrollable containers on mobile
4. **Buttons**: Larger touch targets on mobile
5. **Typography**: Scalable font sizes

## Accessibility Requirements

1. **Keyboard Navigation**: All interactive elements accessible via keyboard
2. **Screen Reader Support**: Proper ARIA labels and roles
3. **Color Contrast**: Minimum 4.5:1 contrast ratio for text
4. **Focus Indicators**: Visible focus states for interactive elements
5. **Semantic HTML**: Proper use of heading levels and landmarks

## Performance Optimization

1. **Lazy Loading**: Components loaded only when needed
2. **Memoization**: React.memo for expensive components
3. **Bundle Splitting**: Separate bundles for different routes
4. **Image Optimization**: Properly sized images with WebP format
5. **Caching**: Cache commitment data where appropriate

## Error Handling

### Error Boundaries
- Top-level error boundary for unhandled exceptions
- Component-specific error boundaries for graceful degradation

### User Feedback
- Toast notifications for success/error messages
- Inline validation errors in forms
- Loading states for asynchronous operations
- Empty states for lists with no data

## Testing Strategy

### Unit Tests
- Test individual components with Jest and React Testing Library
- Mock service layer for isolated component testing

### Integration Tests
- Test wallet connection flow
- Test contract interaction flows

### End-to-End Tests
- Critical user journeys with Cypress
- Wallet connection and transaction flows

## Security Considerations

1. **Input Validation**: Client-side and server-side validation
2. **Sanitization**: Sanitize user inputs displayed in UI
3. **Secure Storage**: No sensitive data stored in localStorage
4. **CSP**: Content Security Policy headers
5. **HTTPS**: All connections over HTTPS