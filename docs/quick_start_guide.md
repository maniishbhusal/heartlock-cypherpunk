# HeartLock Quick Start Guide

This guide provides step-by-step instructions for team members to begin implementing the HeartLock application.

## Prerequisites

Before starting, ensure you have the following installed:

1. **Node.js** (v16 or higher)
2. **Yarn** (v1.22 or higher)
3. **Rust** and **Cargo**
4. **Solana CLI tools**
5. **Anchor CLI** (v0.31.1)
6. **Git**

### Installation Commands

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

## Repository Setup

1. **Clone the repository:**
```bash
git clone <repository-url>
cd heartlock-cypherpunk
```

2. **Install dependencies:**
```bash
yarn install
```

## Smart Contract Development Workflow

### 1. Understanding the Codebase

Key files to review:
- `programs/heartlock-cypherpunk/src/lib.rs` - Main program logic
- `docs/contract_spec.md` - Detailed specification
- `docs/smart_contract_tasks.md` - Implementation tasks

### 2. Building the Contract

```bash
# Build the program
anchor build

# Check build output
ls -la target/deploy/
```

### 3. Running Tests

```bash
# Run tests with local validator
anchor test

# Run tests with logs
anchor test --logs
```

### 4. Deploying Locally

```bash
# Start local validator
solana-test-validator

# In another terminal, deploy
anchor deploy
```

## Frontend Development Workflow

### 1. Setting Up the Frontend Environment

Create the app directory structure:
```bash
mkdir -p app/{src,public,tests}
```

Reference files:
- `docs/frontend_spec.md` - Component specifications
- `docs/frontend_structure_spec.md` - Project structure
- `docs/frontend_package_spec.md` - Dependencies
- `docs/frontend_tasks.md` - Implementation tasks

### 2. Installing Frontend Dependencies

```bash
cd app
# Create package.json based on frontend_package_spec.md
# Then install dependencies
yarn install
```

### 3. Starting Development Server

```bash
cd app
yarn dev
```

## Daily Development Process

### Morning Routine

1. **Sync with team**: Check task board and communications
2. **Review yesterday's work**: Pull latest changes
3. **Plan today's tasks**: Update personal task list

### Development Cycle

1. **Create feature branch**:
```bash
git checkout -b feature/your-feature-name
```

2. **Implement functionality**:
   - Follow specifications in documentation
   - Write tests alongside implementation
   - Commit frequently with clear messages

3. **Test changes**:
   - Run relevant test suites
   - Manually test user flows
   - Check for regressions

4. **Submit for review**:
   - Push changes to remote
   - Create pull request
   - Request review from team members

### Testing Process

1. **Unit tests**: Run with each change
2. **Integration tests**: Run before submitting PR
3. **Manual testing**: Test user flows in browser
4. **Cross-browser testing**: Verify on different browsers

## Communication Channels

### Team Coordination
- **Daily Standups**: 9:00 AM daily
- **Task Board**: GitHub Issues/Projects or similar
- **Code Reviews**: GitHub PR reviews
- **Blockers**: Communicate immediately

### Documentation Updates
- Update relevant docs when implementing features
- Add inline comments for complex logic
- Keep README up to date with changes

## Troubleshooting Common Issues

### Anchor Build Issues
```bash
# Clean build
anchor clean
anchor build

# Check versions
anchor --version
solana --version
rustc --version
```

### Wallet Connection Problems
1. Ensure wallet adapter is properly configured
2. Check network settings (devnet vs localnet)
3. Verify wallet is connected to correct network

### Test Failures
1. Check local validator is running
2. Ensure sufficient SOL in test accounts
3. Verify account states match expectations

## Resources

### Documentation
- `docs/architecture.md` - Overall system design
- `docs/implementation_plan.md` - Timeline and responsibilities
- `docs/system_diagram.md` - Visual architecture
- `docs/contract_spec.md` - Smart contract details
- `docs/frontend_spec.md` - Frontend component specs

### External Resources
- [Anchor Documentation](https://book.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [React Documentation](https://reactjs.org/docs/getting-started.html)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)

## Getting Help

1. **Team Members**: Reach out to assigned team members
2. **Technical Questions**: Use team communication channels
3. **External Resources**: Refer to official documentation
4. **Blockers**: Escalate immediately to team lead

This quick start guide should help you begin contributing to HeartLock immediately. Remember to refer to the detailed documentation for specific implementation details.