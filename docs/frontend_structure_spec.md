# Frontend Application Structure Specification

This document outlines the file structure and core components for the HeartLock React frontend application.

## Overall Directory Structure

```
app/
├── public/                    # Static assets
│   ├── favicon.ico           # Favicon
│   ├── index.html            # HTML template
│   └── icons/                # Icon files
├── src/                      # Source code
│   ├── components/           # React components
│   │   ├── common/           # Shared components
│   │   ├── layout/           # Layout components
│   │   ├── commitment/       # Commitment-related components
│   │   ├── deposit/          # Deposit-related components
│   │   ├── dispute/          # Dispute-related components
│   │   └── withdrawal/       # Withdrawal-related components
│   ├── hooks/                # Custom React hooks
│   ├── contexts/             # React contexts
│   ├── utils/                # Utility functions
│   ├── services/             # API/services integration
│   ├── pages/                # Page components
│   ├── types/                # TypeScript types
│   ├── styles/               # Global styles
│   ├── assets/               # Application assets
│   ├── App.tsx               # Main app component
│   └── main.tsx              # Application entry point
├── tests/                    # Test files
│   ├── components/           # Component tests
│   ├── hooks/                # Hook tests
│   └── integration/          # Integration tests
├── docs/                     # Frontend documentation
├── .gitignore                # Git ignore rules
├── tsconfig.json             # TypeScript configuration
├── tsconfig.node.json        # TypeScript node configuration
├── vite.config.ts            # Vite configuration
├── tailwind.config.js        # Tailwind CSS configuration
├── postcss.config.js         # PostCSS configuration
└── package.json              # Package configuration
```

## Core Files

### 1. Entry Point (src/main.tsx)
```tsx
import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './App'
import './styles/index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>,
)
```

### 2. Main App Component (src/App.tsx)
```tsx
import React from 'react'
import { WalletProvider } from './contexts/WalletContext'
import { CommitmentProvider } from './contexts/CommitmentContext'
import AppRouter from './components/layout/AppRouter'
import Navbar from './components/layout/Navbar'
import Footer from './components/layout/Footer'
import NotificationContainer from './components/common/NotificationContainer'

const App: React.FC = () => {
  return (
    <WalletProvider>
      <CommitmentProvider>
        <div className="min-h-screen flex flex-col bg-gray-50">
          <Navbar />
          <main className="flex-grow">
            <AppRouter />
          </main>
          <Footer />
          <NotificationContainer />
        </div>
      </CommitmentProvider>
    </WalletProvider>
  )
}

export default App
```

### 3. Router Component (src/components/layout/AppRouter.tsx)
```tsx
import React from 'react'
import { Routes, Route } from 'react-router-dom'
import HomePage from '../../pages/HomePage'
import CreateCommitmentPage from '../../pages/CreateCommitmentPage'
import CommitmentDetailPage from '../../pages/CommitmentDetailPage'
import MyCommitmentsPage from '../../pages/MyCommitmentsPage'
import ArbiterDashboardPage from '../../pages/ArbiterDashboardPage'
import NotFoundPage from '../../pages/NotFoundPage'

const AppRouter: React.FC = () => {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/create" element={<CreateCommitmentPage />} />
      <Route path="/commitment/:id" element={<CommitmentDetailPage />} />
      <Route path="/my-commitments" element={<MyCommitmentsPage />} />
      <Route path="/arbiter-dashboard" element={<ArbiterDashboardPage />} />
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  )
}

export default AppRouter
```

## Context Providers

### 1. Wallet Context (src/contexts/WalletContext.tsx)
```tsx
import React, { createContext, useContext, useEffect, useState } from 'react'
import { useWallet, WalletProvider as SolanaWalletProvider } from '@solana/wallet-adapter-react'
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui'

interface WalletContextType {
  connected: boolean
  publicKey: string | null
  connect: () => void
  disconnect: () => void
}

const WalletContext = createContext<WalletContextType | undefined>(undefined)

export const WalletProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  // Implementation for wallet connection and state management
  return (
    <SolanaWalletProvider /* ... */>
      <WalletModalProvider>
        {/* Context provider implementation */}
        {children}
      </WalletModalProvider>
    </SolanaWalletProvider>
  )
}

export const useWalletContext = () => {
  const context = useContext(WalletContext)
  if (context === undefined) {
    throw new Error('useWalletContext must be used within a WalletProvider')
  }
  return context
}
```

### 2. Commitment Context (src/contexts/CommitmentContext.tsx)
```tsx
import React, { createContext, useContext, useState } from 'react'
import { CommitmentAccount } from '../types/commitment'

interface CommitmentContextType {
  commitments: CommitmentAccount[]
  fetchCommitments: () => Promise<void>
  createCommitment: (data: any) => Promise<string>
  refreshCommitment: (id: string) => Promise<void>
}

const CommitmentContext = createContext<CommitmentContextType | undefined>(undefined)

export const CommitmentProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [commitments, setCommitments] = useState<CommitmentAccount[]>([])
  
  // Implementation for commitment state management
  return (
    <CommitmentContext.Provider value={{
      commitments,
      fetchCommitments: async () => {},
      createCommitment: async () => '',
      refreshCommitment: async () => {}
    }}>
      {children}
    </CommitmentContext.Provider>
  )
}

export const useCommitmentContext = () => {
  const context = useContext(CommitmentContext)
  if (context === undefined) {
    throw new Error('useCommitmentContext must be used within a CommitmentProvider')
  }
  return context
}
```

## Key Pages

### 1. Home Page (src/pages/HomePage.tsx)
```tsx
import React from 'react'
import HeroSection from '../components/common/HeroSection'
import FeaturesSection from '../components/common/FeaturesSection'
import CtaSection from '../components/common/CtaSection'

const HomePage: React.FC = () => {
  return (
    <div>
      <HeroSection />
      <FeaturesSection />
      <CtaSection />
    </div>
  )
}

export default HomePage
```

### 2. Create Commitment Page (src/pages/CreateCommitmentPage.tsx)
```tsx
import React from 'react'
import CreateCommitmentForm from '../components/commitment/CreateCommitmentForm'
import PageHeader from '../components/layout/PageHeader'

const CreateCommitmentPage: React.FC = () => {
  return (
    <div className="container mx-auto px-4 py-8">
      <PageHeader title="Create New Commitment" />
      <CreateCommitmentForm />
    </div>
  )
}

export default CreateCommitmentPage
```

### 3. Commitment Detail Page (src/pages/CommitmentDetailPage.tsx)
```tsx
import React from 'react'
import { useParams } from 'react-router-dom'
import CommitmentDashboard from '../components/commitment/CommitmentDashboard'
import DepositInterface from '../components/deposit/DepositInterface'
import DisputeReporting from '../components/dispute/DisputeReporting'

const CommitmentDetailPage: React.FC = () => {
  const { id } = useParams<{ id: string }>()
  
  return (
    <div className="container mx-auto px-4 py-8">
      <CommitmentDashboard commitmentId={id || ''} />
      <DepositInterface commitmentId={id || ''} />
      <DisputeReporting commitmentId={id || ''} />
    </div>
  )
}

export default CommitmentDetailPage
```

## Configuration Files

### 1. TypeScript Configuration (tsconfig.json)
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

### 2. Vite Configuration (vite.config.ts)
```ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  optimizeDeps: {
    exclude: ['@solana/wallet-adapter-react-ui']
  }
})
```

### 3. Tailwind Configuration (tailwind.config.js)
```js
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eff6ff',
          500: '#3b82f6',
          900: '#1e3a8a',
        },
        secondary: {
          500: '#8b5cf6',
        }
      }
    },
  },
  plugins: [],
}
```

## Asset Organization

### Public Assets
```
public/
├── favicon.ico
├── icons/
│   ├── logo.svg
│   ├── commitment.svg
│   ├── deposit.svg
│   ├── dispute.svg
│   └── withdrawal.svg
└── manifest.json
```

### Source Assets
```
src/assets/
├── images/
│   ├── hero-bg.jpg
│   ├── feature-icons/
│   └── illustrations/
└── icons/
    ├── custom-icons.tsx
    └── icon-components/
```

## Styles Organization

### Global Styles (src/styles/index.css)
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply text-gray-900;
  }
  
  h1, h2, h3, h4, h5, h6 {
    @apply font-bold;
  }
}

@layer components {
  .btn-primary {
    @apply bg-primary-500 hover:bg-primary-600 text-white font-medium py-2 px-4 rounded transition-colors;
  }
  
  .btn-secondary {
    @apply bg-secondary-500 hover:bg-secondary-600 text-white font-medium py-2 px-4 rounded transition-colors;
  }
  
  .card {
    @apply bg-white rounded-lg shadow-md p-6;
  }
}

@layer utilities {
  .text-balance {
    text-wrap: balance;
  }
}
```

## Test Structure

### Unit Tests
```
tests/
├── components/
│   ├── common/
│   ├── layout/
│   ├── commitment/
│   ├── deposit/
│   ├── dispute/
│   └── withdrawal/
├── hooks/
└── utils/
```

### Integration Tests
```
tests/integration/
├── wallet.integration.test.ts
├── commitment.integration.test.ts
└── transaction.integration.test.ts
```

## Documentation

Each major component directory should include a README.md file explaining:
1. Purpose of components in that directory
2. Props interface for each component
3. Usage examples
4. Dependencies and requirements

Example for commitment components:
```
src/components/commitment/README.md
```

This structure provides a scalable foundation for the HeartLock frontend application while maintaining clear separation of concerns and promoting maintainability.