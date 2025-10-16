# HeartLock - Consensual Commitment Escrow on Solana

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

HeartLock is a Solana-based smart contract that enables two parties to create a joint commitment contract with scheduled deposits, optional yield generation, and consensual dispute resolution mechanisms.

Built for the Solana Hackathon.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Smart Contract Development](#smart-contract-development)
- [Frontend Development](#frontend-development)
- [Testing](#testing)
- [Deployment](#deployment)
- [Documentation](#documentation)
- [Contributing](#contributing)

## Overview

HeartLock allows couples or partners to create financial commitments with the following characteristics:

- **Consensual**: Both parties must agree to all terms before activation
- **Scheduled**: Fixed contributions on weekly or monthly schedules
- **Yield-bearing**: Optional yield generation on locked funds
- **Dispute-aware**: Predefined rules for handling breaches
- **Privacy-focused**: Minimal on-chain personal data

## Features

### Core Functionality
- ✅ Joint commitment contract creation
- ✅ Scheduled deposit enforcement
- ✅ Consent-based activation
- ✅ Successful completion withdrawals
- ✅ Dispute reporting and resolution
- ✅ Penalty distribution mechanisms

### Advanced Features
- 🔄 Optional yield integration
- 👨‍⚖️ Arbiter-based dispute resolution
- 📱 Responsive web interface
- 🔐 Secure wallet integration
- 📊 Real-time commitment tracking

## Architecture

The system consists of two main components:

1. **Solana Smart Contract** (Rust/Anchor)
2. **React Frontend** (TypeScript)

For detailed architecture diagrams and specifications, see:
- [System Architecture](docs/system_diagram.md)
- [Smart Contract Specification](docs/contract_spec.md)
- [Frontend Specification](docs/frontend_spec.md)

## Prerequisites

Before you begin, ensure you have the following installed:

- Node.js (v16 or higher)
- Yarn (v1.22 or higher)
- Rust and Cargo
- Solana CLI tools
- Anchor CLI (v0.31.1)
- Git

### Installing Solana CLI
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

### Installing Anchor CLI
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

## Setup

1. **Clone the repository:**
```bash
git clone <repository-url>
cd heartlock-cypherpunk
```

2. **Install dependencies:**
```bash
yarn install
```

3. **Build the smart contract:**
```bash
anchor build
```

4. **Run tests:**
```bash
anchor test
```

## Project Structure

```
heartlock-cypherpunk/
├── programs/                 # Solana smart contracts
│   └── heartlock-cypherpunk/ # Main program
├── app/                      # React frontend (to be created)
├── migrations/               # Deployment scripts
├── tests/                    # Smart contract tests
├── docs/                     # Documentation
├── target/                   # Build artifacts
├── Anchor.toml               # Anchor configuration
├── Cargo.toml                # Rust workspace configuration
└── package.json              # Node.js dependencies
```

## Development Workflow

### Daily Standups
- Morning sync at 9:00 AM
- Discuss progress, blockers, and today's goals
- Update task board

### Code Reviews
- All PRs require review from at least one other team member
- Follow the style guides in documentation
- Ensure tests pass before merging

### Branch Strategy
- `main`: Production-ready code
- `develop`: Integration branch
- `feature/*`: Feature branches
- `hotfix/*`: Urgent fixes

## Smart Contract Development

### Building
```bash
anchor build
```

### Testing
```bash
anchor test
```

### Local Deployment
```bash
solana config set --url localhost
anchor deploy
```

### Key Files
- `programs/heartlock-cypherpunk/src/lib.rs`: Main program logic
- `tests/heartlock-cypherpunk.ts`: Integration tests
- `docs/contract_spec.md`: Detailed specification

## Frontend Development

### Starting Development Server
```bash
cd app
yarn dev
```

### Building for Production
```bash
yarn build
```

### Key Files
- `app/src/App.tsx`: Main application component
- `app/src/components/`: UI components
- `docs/frontend_spec.md`: Detailed specification

## Testing

### Smart Contract Tests
Located in `tests/heartlock-cypherpunk.ts`

Run with:
```bash
anchor test
```

### Frontend Tests
Component tests using Jest and React Testing Library

Run with:
```bash
cd app
yarn test
```

## Deployment

### Devnet Deployment
```bash
solana config set --url devnet
anchor deploy
```

### Updating Program ID
1. Generate new keypair:
```bash
solana-keygen new -o target/deploy/heartlock_cypherpunk-keypair.json
```

2. Update `declare_id!` in `programs/heartlock-cypherkpunk/src/lib.rs`
3. Update `Anchor.toml` with new program ID

## Documentation

All documentation is located in the `docs/` directory:

- [Architecture Overview](docs/architecture.md)
- [Implementation Plan](docs/implementation_plan.md)
- [System Diagrams](docs/system_diagram.md)
- [Smart Contract Specification](docs/contract_spec.md)
- [Frontend Specification](docs/frontend_spec.md)

## Contributing

This project is being developed for a hackathon with a tight deadline. To contribute effectively:

1. Review the [implementation plan](docs/implementation_plan.md)
2. Check the task board for assigned items
3. Follow the established coding standards
4. Write tests for new functionality
5. Keep PRs small and focused
6. Communicate blockers immediately

### Team Roles
- **Team Member 1**: Smart Contract Development
- **Team Member 2**: Frontend Development
- **Team Member 3**: Integration, Testing, and DevOps

For any questions or issues, contact the team lead immediately.

---

*This project is for educational purposes and hackathon submission. Not intended for production use without extensive security auditing.*