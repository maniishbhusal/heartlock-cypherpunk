# Frontend Package Specification

This document outlines the package configuration for the HeartLock React frontend application.

## Package.json Structure

```json
{
  "name": "heartlock-frontend",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "test": "vitest",
    "test:ui": "vitest --ui",
    "coverage": "vitest run --coverage",
    "lint": "eslint src --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "lint:fix": "eslint src --ext ts,tsx --report-unused-disable-directives --max-warnings 0 --fix",
    "preview-build": "vite preview --port 5173"
  },
  "dependencies": {
    "@solana/wallet-adapter-base": "^0.9.23",
    "@solana/wallet-adapter-react": "^0.15.35",
    "@solana/wallet-adapter-react-ui": "^0.9.35",
    "@solana/wallet-adapter-wallets": "^0.19.32",
    "@solana/web3.js": "^1.95.3",
    "@coral-xyz/anchor": "^0.31.1",
    "react": "^18.3.1",
    "react-dom": "^18.3.1",
    "react-router-dom": "^6.26.2",
    "tailwindcss": "^3.4.13",
    "autoprefixer": "^10.4.20",
    "postcss": "^8.4.47"
  },
  "devDependencies": {
    "@types/react": "^18.3.11",
    "@types/react-dom": "^18.3.0",
    "@typescript-eslint/eslint-plugin": "^8.8.0",
    "@typescript-eslint/parser": "^8.8.0",
    "@vitejs/plugin-react": "^4.3.2",
    "eslint": "^8.57.0",
    "eslint-plugin-react-hooks": "^4.6.2",
    "eslint-plugin-react-refresh": "^0.4.12",
    "typescript": "^5.6.2",
    "vite": "^5.4.8",
    "vitest": "^2.1.1",
    "jsdom": "^25.0.1",
    "@vitest/coverage-v8": "^2.1.1",
    "@vitest/ui": "^2.1.1"
  }
}
```

## Key Dependencies Explained

### Production Dependencies

1. **@solana/wallet-adapter-base, -react, -react-ui, -wallets**
   - Core libraries for Solana wallet integration
   - Provides components and hooks for wallet connection
   - Supports multiple wallet providers (Phantom, Solflare, etc.)

2. **@solana/web3.js**
   - Official Solana JavaScript SDK
   - Used for interacting with the Solana blockchain
   - Required for sending transactions and querying account data

3. **@coral-xyz/anchor**
   - Anchor client library for TypeScript
   - Simplifies interaction with Anchor-based smart contracts
   - Provides type-safe program client generation

4. **react, react-dom**
   - Core React libraries for building the UI
   - Using version 18+ for modern features

5. **react-router-dom**
   - Client-side routing for the single-page application
   - Enables navigation between different views

6. **tailwindcss, autoprefixer, postcss**
   - Utility-first CSS framework for styling
   - PostCSS plugins for processing CSS
   - Enables responsive and consistent design

### Development Dependencies

1. **@types/react, @types/react-dom**
   - TypeScript type definitions for React
   - Enables type checking in TypeScript files

2. **@typescript-eslint/eslint-plugin, @typescript-eslint/parser**
   - ESLint plugin and parser for TypeScript
   - Enforces code quality and consistency

3. **@vitejs/plugin-react**
   - Vite plugin for React fast refresh
   - Improves development experience

4. **eslint, eslint-plugin-react-hooks, eslint-plugin-react-refresh**
   - Code linting tools
   - Enforce React best practices

5. **typescript**
   - TypeScript compiler
   - Enables static typing in JavaScript

6. **vite**
   - Fast build tool and development server
   - Replaces Create React App for better performance

7. **vitest, jsdom, @vitest/coverage-v8, @vitest/ui**
   - Testing framework and utilities
   - Unit and integration testing
   - Code coverage reporting
   - Visual testing UI

## Scripts Explanation

1. **dev**: Starts the development server with hot module replacement
2. **build**: Compiles TypeScript and builds the production bundle
3. **preview**: Locally previews the production build
4. **test**: Runs unit tests in watch mode
5. **test:ui**: Runs tests with visual UI
6. **coverage**: Generates code coverage report
7. **lint**: Checks code for linting errors
8. **lint:fix**: Automatically fixes fixable linting errors
9. **preview-build**: Previews the production build on port 5173

## Installation Commands

To set up the frontend development environment:

```bash
# Navigate to the app directory
cd app

# Install dependencies
yarn install

# Start development server
yarn dev
```

## Build Process

The build process involves:
1. TypeScript compilation
2. Bundling with Vite
3. Optimization for production
4. Output to `dist/` directory

To build for production:
```bash
yarn build
```

The resulting build can be deployed to any static hosting service.