# FinWise AI - Project Summary

## Overview
FinWise AI is a comprehensive Smart Financial Advisory dApp that combines AI-driven financial recommendations with blockchain-based reward incentives to help users manage their finances effectively.

---

## Technology Stack

### Frontend
- **Framework**: React.js (Vite)
- **Language**: JavaScript
- **Styling**: Tailwind CSS
- **3D Graphics**: Three.js (Background3D component)
- **HTTP Client**: Axios
- **Icons**: Lucide React

### Backend
- **Language**: Rust
- **Framework**: Actix-web
- **Database**: MongoDB Atlas
- **Authentication**: Session-based with Google OAuth

### Blockchain
- **Network**: Stellar / Soroban
- **Smart Contracts**: Rust-based Soroban contracts
- **Wallet Integration**: Albedo, xBull

---

## Features Implemented

### 1. User Authentication System
- **Email/Password Signup**: Users can register with username, email, and password
- **Email/Password Login**: Secure login with bcrypt password hashing
- **Google OAuth Integration**: Sign in with Google for seamless authentication
- **Session Management**: Cookie-based session handling with Actix Session
- **Protected Routes**: Dashboard and Analyze pages require authentication

### 2. Wallet Integration
- **Multi-Wallet Support**: Connect with Albedo and xBull wallets
- **Balance Display**: View XLM balance from Stellar network
- **Transaction History**: View past transactions
- **Send Transactions**: Send XLM to other addresses

### 3. AI Financial Analysis
- **Natural Language Input**: Users describe their financial situation in plain English
- **Structured Data Extraction**: Automatically extracts income, country, family details, debts
- **AI-Powered Advice**: Uses Ollama (Llama3) to generate personalized financial advice
- **Country-Specific Logic**: Adjusts recommendations based on tax laws of user's country
- **Analysis Results Include**:
  - Risk assessment (Low/Medium/High)
  - Budget allocation (Needs/Wants/Savings)
  - Tax saving suggestions
  - Income growth suggestions
  - Debt reduction strategy

### 4. Piggy Bank (Savings Goals)
- **Daily Deposits**: Track daily savings
- **Streak Tracking**: Monitor consecutive saving days
- **Goal Commitments**: Set savings goals with duration
- **Reward Points**: Earn points for maintaining streaks
- **Statistics**: View total saved, current streak, longest streak

### 5. Dashboard
- **Balance Overview**: Display wallet balance
- **Transaction History**: List recent transactions
- **Savings Chart**: Visual representation of savings progress
- **Budget Chart**: Visual breakdown of budget allocation

### 6. Blockchain Smart Contract (Soroban)
The smart contract provides on-chain savings tracking with the following functions:

#### Functions:
- `deposit(user, amount)`: Records daily savings deposits, enforces one deposit per 24 hours, tracks streaks and reward points
- `get_user_stats(user)`: Returns total saved, current streak, longest streak, and reward points
- `commit_goal(user, goal_amount, duration_days)`: Creates an on-chain savings goal commitment
- `get_goal(user)`: Returns the user's active savings goal

#### Data Stored On-Chain:
- total_saved: Total amount saved
- current_streak: Consecutive days of saving
- longest_streak: Best streak achieved
- reward_points: Points earned through consistent saving
- last_deposit_timestamp: Time of last deposit
- commitment_data: Goal amount, duration, deposits made, completion status

---

## Project Structure

```
finwise-frontend/
├── src/
│   ├── components/
│   │   ├── Background3D.jsx      # 3D animated background
│   │   ├── BalanceDisplay.jsx    # Wallet balance display
│   │   ├── BudgetChart.jsx       # Budget allocation chart
│   │   ├── Navbar.jsx            # Navigation bar
│   │   ├── RiskBadge.jsx         # Risk level indicator
│   │   ├── SavingsChart.jsx      # Savings progress chart
│   │   ├── SendTransaction.jsx   # Send XLM form
│   │   ├── TransactionStatus.jsx # Transaction status display
│   │   ├── TxHistory.jsx         # Transaction history list
│   │   ├── WalletConnect.jsx     # Wallet connection component
│   │   └── WalletConnect.css
│   ├── hooks/
│   │   └── useAuth.js            # Authentication hook
│   ├── pages/
│   │   ├── About.jsx             # About page
│   │   ├── Analyze.jsx            # AI Financial Analysis page
│   │   ├── Dashboard.jsx          # User dashboard
│   │   ├── Hero.css              # Hero section styling
│   │   ├── Hero.jsx              # Hero section
│   │   ├── Landing.jsx          # Landing page
│   │   ├── PiggyBank.jsx         # Piggy bank page
│   │   ├── Signin.css            # Signin styling
│   │   ├── Signin.jsx            # Signin page
│   │   ├── sorobanService.js     # Soroban integration
│   │   └── stellarService.js     # Stellar integration
│   ├── services/
│   │   ├── api.js                # API client
│   │   ├── walletManager.js      # Wallet management
│   │   └── wallets/
│   │       ├── albedo.js        # Albedo wallet integration
│   │       └── xbull.js          # xBull wallet integration
│   ├── App.css
│   ├── App.jsx
│   ├── index.css
│   └── main.jsx
├── index.html
├── package.json
├── tailwind.config.js
└── vite.config.js

finwise-backend/
├── src/
│   ├── config.rs                 # Configuration management
│   ├── db/
│   │   └── mod.rs                # Database connection & collections
│   ├── main.rs                   # Application entry point & routes
│   ├── models/
│   │   ├── analysis.rs           # Analysis data models
│   │   ├── mod.rs
│   │   ├── piggy.rs              # Piggy bank data models
│   │   ├── profile.rs            # User profile models
│   │   └── user.rs               # User data models
│   ├── routes/
│   │   ├── analyze.rs            # AI analysis route handler
│   │   ├── health.rs             # Health check endpoint
│   │   ├── mod.rs
│   │   ├── piggy.rs              # Piggy bank route handlers
│   │   ├── profile.rs            # Profile route handlers
│   │   └── routes.rs             # Stellar transaction routes
│   ├── services/
│   │   ├── mod.rs
│   │   ├── nlp_service.rs        # NLP processing
│   │   ├── ollama_service.rs    # Ollama AI service integration
│   │   └── piggy_service.rs      # Piggy bank business logic
│   ├── stellar.rs               # Stellar network utilities
│   └── utils/
│       └── mod.rs               # Utility functions
├── Cargo.toml
└── Cargo.lock

finwise-contract/
├── src/
│   ├── contract_test.rs         # Smart contract tests
│   ├── errors.rs                # Custom error types
│   ├── events.rs                # Contract events/emissions
│   ├── lib.rs                   # Contract implementation
│   ├── storage.rs               # Storage types
│   └── utils.rs                 # Utility functions
├── Cargo.toml
└── Cargo.lock
```

---

## API Endpoints

### Authentication
- `POST /api/signup` - Register new user
- `POST /api/login` - User login
- `POST /api/logout` - User logout
- `GET /api/check-auth` - Check authentication status
- `GET /auth/google` - Google OAuth login
- `GET /auth/google/callback` - Google OAuth callback

### Stellar Operations
- `GET /api/balance/{address}` - Get wallet balance
- `GET /api/transactions/{address}` - Get transaction history
- `POST /api/send` - Send XLM transaction

### Financial Features
- `GET /api/profile` - Get user profile
- `POST /api/analyze` - AI financial analysis
- `POST /api/piggy/deposit` - Record savings deposit
- `GET /api/piggy/stats/{user_id}` - Get piggy bank statistics

### System
- `GET /health` - Health check endpoint

---

## Key Implementation Details

### Authentication Flow
1. User signs up with email/password or Google OAuth
2. Password is hashed using bcrypt (DEFAULT_COST)
3. Session is created with user_id stored in cookie
4. Protected routes check session for authentication

### AI Analysis Flow
1. User inputs financial situation as free text
2. Backend extracts structured data (income, country, family, debts)
3. Ollama service generates personalized financial advice
4. Results stored in MongoDB for history
5. User profile updated with latest advice

### Piggy Bank Flow
1. User sets a savings goal (amount and duration)
2. User makes deposits (one per 24 hours allowed)
3. Contract tracks streaks and awards points
4. Goal progress is recorded on-chain

### Blockchain Integration
1. Smart contract deployed on Stellar Testnet
2. Frontend connects to wallet (Albedo/xBull)
3. Transactions signed by user's wallet
4. On-chain data tracks savings and rewards

---

## Security Features
- Password hashing with bcrypt
- Session-based authentication with secure cookies
- User authorization required for smart contract calls
- Input validation on all endpoints
- CORS configuration
- Environment variable management with dotenv

---

## External Services & APIs
- **MongoDB Atlas**: User data and analysis storage
- **Stellar**: Blockchain network for transactions
- **Ollama**: Local AI model (Llama3) for financial advice
- **Google OAuth**: Third-party authentication

---

## Getting Started

### Prerequisites
- Node.js 18+
- Rust 1.70+
- MongoDB (local or Atlas)
- Ollama with Llama3 model

### Environment Variables Required
- `MONGODB_URI`: MongoDB connection string
- `SESSION_SECRET`: Secret for session encryption
- `STELLAR_NETWORK`: TESTNET or PUBLIC
- `GOOGLE_CLIENT_ID`: Google OAuth client ID
- `GOOGLE_CLIENT_SECRET`: Google OAuth client secret

### Running the Project

1. **Start Backend**:
   
```
bash
   cd finwise-backend
   cargo run
   
```

2. **Start Frontend**:
   
```
bash
   cd finwise-frontend
   npm install
   npm run dev
   
```

3. **Deploy Smart Contract**:
   
```
bash
   cd finwise-contract
   soroban contract deploy ...
   
```

---

## Summary

FinWise AI is a full-stack dApp that successfully combines:
- Modern web technologies (React, Tailwind CSS)
- Rust-based backend with Actix-web
- MongoDB for data persistence
- AI-powered financial advisory using Ollama
- Blockchain technology (Stellar/Soroban) for reward system
- Multi-wallet support for blockchain interactions
- Google OAuth for streamlined authentication

The platform provides users with personalized financial advice, tracks their savings goals, and incentivizes disciplined saving through blockchain-based reward points.
