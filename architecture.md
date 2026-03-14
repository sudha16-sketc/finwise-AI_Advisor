# FinWise — System Architecture Document

> **Document Path:** `docs/architecture.md`  
> **Version:** 1.0.0  
> **Last Updated:** March 2026  
> **Status:** Production

---

## Table of Contents

1. [System Overview](#1-system-overview)
2. [High-Level Architecture](#2-high-level-architecture)
3. [Core System Components](#3-core-system-components)
4. [Authentication and Wallet Integration](#4-authentication-and-wallet-integration)
5. [AI Financial Analysis Flow](#5-ai-financial-analysis-flow)
6. [Transaction and Blockchain Interaction Flow](#6-transaction-and-blockchain-interaction-flow)
7. [Dashboard Functional Architecture](#7-dashboard-functional-architecture)
8. [Data Flow Diagram](#8-data-flow-diagram)
9. [Deployment Architecture](#9-deployment-architecture)
10. [Security Considerations](#10-security-considerations)
11. [Scalability Considerations](#11-scalability-considerations)
12. [Future Architecture Improvements](#12-future-architecture-improvements)

---

## 1. System Overview

**FinWise** is a Web3 financial assistant and savings dashboard built on the Stellar blockchain network. It combines traditional web authentication with decentralised wallet connectivity, AI-powered financial analysis, and on-chain transaction capabilities — all delivered through a responsive single-page application.

### Core Capabilities

| Capability | Description |
|---|---|
| Wallet Integration | Connect via Stellar WalletConnect Kit, Freighter, or other Stellar wallets |
| Authentication | Email/password signup, Google OAuth, and wallet-based identity |
| AI Financial Analysis | Chat interface backed by an AI API for personalised financial advice |
| XLM Transactions | Send XLM tokens directly from the dashboard via the Stellar Testnet |
| Transaction History | View on-chain transaction records via the Horizon API |
| Piggy Streak Savings | Gamified savings feature with streak tracking and reward points |
| Financial Dashboard | Unified view of wallet balance, analyses, and savings stats |

### Design Principles

- **Separation of Concerns** — Frontend, backend, blockchain, and AI services are independently deployable layers.
- **Stateless Backend** — The API server holds no long-lived in-memory state; sessions are persisted in MongoDB.
- **Progressive Enhancement** — The application degrades gracefully when wallet or blockchain services are unavailable.
- **Cross-Origin Compatibility** — Authentication is designed to work across separate frontend and backend domains.

---

## 2. High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          CLIENT LAYER                                   │
│                                                                         │
│   ┌──────────────────────────────────────────────────────────────────┐  │
│   │                    React SPA (Vercel)                            │  │
│   │                                                                  │  │
│   │  ┌──────────┐  ┌──────────┐  ┌───────────┐  ┌───────────────┐  │  │
│   │  │  Hero /  │  │ Analyze  │  │ Dashboard │  │  Send / Tx    │  │  │
│   │  │  About   │  │  Chat    │  │  Page     │  │  History      │  │  │
│   │  └──────────┘  └──────────┘  └───────────┘  └───────────────┘  │  │
│   │                                                                  │  │
│   │  ┌───────────────────────┐   ┌──────────────────────────────┐  │  │
│   │  │  Stellar Wallets Kit  │   │  axios / fetch API Service   │  │  │
│   │  │  (WalletConnect +     │   │  (finwiseApi, credentials)   │  │  │
│   │  │   Freighter + xBull)  │   └──────────────────────────────┘  │  │
│   │  └───────────────────────┘                                      │  │
│   └──────────────────────────────────────────────────────────────┘  │  │
└─────────────────────────────────────────────────────────────────────────┘
         │  HTTPS REST                              │  Wallet SDK
         ▼                                          ▼
┌─────────────────────────┐            ┌────────────────────────────┐
│     BACKEND LAYER       │            │    BLOCKCHAIN LAYER        │
│                         │            │                            │
│  Rust / Actix-Web API   │            │  Stellar Testnet           │
│  (Railway)              │◄──────────►│  Horizon API               │
│                         │            │  Stellar SDK               │
│  ┌─────────────────┐    │            │                            │
│  │  Auth Routes    │    │            │  ┌──────────────────────┐  │
│  │  Profile Routes │    │            │  │ Account / Balance    │  │
│  │  Analyze Routes │    │            │  │ Transaction Submit   │  │
│  │  Stellar Routes │    │            │  │ Transaction History  │  │
│  │  Piggy Routes   │    │            │  └──────────────────────┘  │
│  └─────────────────┘    │            └────────────────────────────┘
│                         │
│  ┌─────────────────┐    │            ┌────────────────────────────┐
│  │    MongoDB      │    │            │   EXTERNAL AI SERVICES     │
│  │    Atlas        │    │            │                            │
│  │  - users        │    │◄──────────►│  Groq API (JSON analysis)  │
│  │  - sessions     │    │            │  Gemini API (advice text)  │
│  │  - analyses     │    │            │                            │
│  │  - piggy_banks  │    │            └────────────────────────────┘
│  └─────────────────┘    │
└─────────────────────────┘
```

---

## 3. Core System Components

### 3.1 Frontend (React SPA)

The frontend is a single-page application built with React and deployed on Vercel.

**Pages**

| Page | Route | Description |
|---|---|---|
| Hero / About | `/` | Landing page with CTA buttons and platform overview |
| Analyze | `/analyze` | AI-powered chat interface for financial questions |
| Dashboard | `/dashboard` | Main user hub — stats, charts, advice, wallet actions |
| Send Transaction | `/sendtransaction` | XLM transfer form with real-time balance |
| Transaction History | `/txhistory` | On-chain transaction log |
| Sign In | `/signin` | Email/password login, Google OAuth, wallet connect |
| Piggy Bank | `/piggy` | Streak-based savings tracker |

**Key Services (Frontend)**

```
src/
├── services/
│   ├── api.js            # axios instance with JWT/credentials interceptor
│   ├── walletManager.js  # Stellar Wallets Kit wrapper (connect, disconnect, events)
│   └── stellarPiggy.js   # Piggy Bank on-chain interactions
├── pages/
│   └── stellarService.js # Stellar SDK — balance, send, transaction history
└── components/
    ├── Navbar.jsx
    ├── SendTransaction.jsx
    ├── TxHistory.jsx
    └── WalletConnect.jsx
```

**State Management**

Global wallet state (`publicKey`, `isConnected`) is managed in `App.jsx` and passed as props to all child components. This single-source-of-truth pattern ensures all pages react consistently to wallet connect/disconnect events, including asynchronous WalletConnect session restores on mobile.

---

### 3.2 Backend (Rust / Actix-Web)

The backend is a REST API server built in Rust using the Actix-Web framework, deployed on Railway.

**Module Structure**

```
src/
├── main.rs               # Server bootstrap, middleware, route registration
├── db.rs                 # MongoDB Atlas connection pool
├── session_store.rs      # MongoDB-backed session store
├── models/
│   └── user.rs           # User document schema
├── routes/
│   ├── analyze.rs        # AI analysis endpoint
│   ├── profile.rs        # User profile endpoint
│   ├── routes.rs         # Stellar balance / send / transactions
│   ├── piggy.rs          # Piggy Bank deposit and stats
│   └── health.rs         # Health check
├── stellar.rs            # Stellar SDK integration
├── config.rs             # Environment variable loading
├── services/             # Business logic layer
└── utils/                # Shared utilities
```

**API Endpoints**

| Method | Endpoint | Description | Auth Required |
|---|---|---|---|
| `POST` | `/api/signup` | Register new user | No |
| `POST` | `/api/login` | Email/password login | No |
| `POST` | `/api/logout` | Invalidate session | Yes |
| `GET` | `/api/check-auth` | Validate current session | No |
| `GET` | `/api/profile` | Fetch user profile and analysis history | Yes |
| `POST` | `/api/analyze` | Submit financial data for AI analysis | Yes |
| `GET` | `/api/balance/:address` | Fetch XLM balance from Stellar | Yes |
| `GET` | `/api/transactions/:address` | Fetch transaction history | Yes |
| `POST` | `/api/send` | Submit signed XLM transaction | Yes |
| `POST` | `/api/piggy/deposit` | Record a savings deposit | Yes |
| `GET` | `/api/piggy/stats/:user_id` | Get savings streak stats | Yes |
| `GET` | `/auth/google` | Initiate Google OAuth flow | No |
| `GET` | `/auth/google/callback` | Handle Google OAuth callback | No |
| `GET` | `/health` | Service health check | No |

---

### 3.3 Database (MongoDB Atlas)

MongoDB Atlas serves as the primary data store. All collections reside in the `finwise` database.

**Collections**

```
finwise/
├── users          # User accounts (email, hashed password, wallet address)
├── sessions       # Server-side session store (session_id → user_id + expiry)
├── analyses       # AI analysis results linked to users
├── profiles       # Aggregated user financial profile and advice history
└── piggy_banks    # Savings streak records per user
```

**Session Document Schema**

```json
{
  "session_id": "uuid-v4-string",
  "data": {
    "user_id": "mongodb-objectid-hex"
  },
  "expires_at": "2026-03-21T00:00:00Z"
}
```

A TTL index on `expires_at` ensures expired sessions are automatically removed by MongoDB.

---

## 4. Authentication and Wallet Integration

FinWise supports three independent authentication methods. Wallet connection is separate from account authentication and can be used alongside any auth method.

### 4.1 Authentication Flow Diagram

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Browser   │     │  React SPA  │     │  Rust API   │     │   MongoDB   │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │                   │
       │   Visit /signin   │                   │                   │
       │──────────────────►│                   │                   │
       │                   │                   │                   │
       │  Submit email +   │                   │                   │
       │    password        │                   │                   │
       │──────────────────►│                   │                   │
       │                   │  POST /api/login  │                   │
       │                   │──────────────────►│                   │
       │                   │                   │  find user by     │
       │                   │                   │  email            │
       │                   │                   │──────────────────►│
       │                   │                   │◄──────────────────│
       │                   │                   │  bcrypt verify    │
       │                   │                   │  create session   │
       │                   │                   │──────────────────►│
       │                   │  Set-Cookie:      │                   │
       │                   │  session_id       │                   │
       │                   │◄──────────────────│                   │
       │  Redirect to      │                   │                   │
       │  /dashboard       │                   │                   │
       │◄──────────────────│                   │                   │
```

### 4.2 Email / Password Authentication

1. User submits credentials via the login form in `WalletConnect.jsx`.
2. Frontend sends `POST /api/login` with `{ email, password }`.
3. Backend fetches user from MongoDB by email.
4. Password is verified using `bcrypt`.
5. On success, a UUID session is created and saved to the `sessions` collection.
6. The session ID is returned as an `HttpOnly`, `Secure`, `SameSite=None` cookie.
7. All subsequent authenticated requests include this cookie automatically.

### 4.3 Google OAuth Authentication

```
Browser ──► GET /auth/google ──► Google OAuth Consent Screen
                                          │
                                          ▼
                              GET /auth/google/callback
                                          │
                              Exchange code for access token
                                          │
                              Fetch user email from Google
                                          │
                              Upsert user in MongoDB
                                          │
                              Create session in MongoDB
                                          │
                              Redirect to /dashboard
```

1. User clicks "Sign in with Google".
2. Browser redirects to `GET /auth/google` on the backend.
3. Backend constructs the Google OAuth URL and redirects to Google.
4. Google redirects to `/auth/google/callback` with an authorization code.
5. Backend exchanges the code for an access token, fetches user info from Google.
6. User is upserted in MongoDB (created if new, fetched if existing).
7. A new MongoDB session is created and session cookie is set.
8. User is redirected to the frontend dashboard.

### 4.4 Wallet Connection (Stellar Wallets Kit)

Wallet connection is managed client-side and is independent of backend authentication.

```
User clicks Connect Wallet
          │
          ▼
Stellar Wallets Kit modal opens
          │
    ┌─────┴──────┐
    │            │
    ▼            ▼
Freighter    WalletConnect
Extension    (QR Code — mobile)
    │            │
    └─────┬──────┘
          │
          ▼
  kit.STATE_UPDATED event fires
          │
          ▼
App.syncWalletState(address)
  - setPublicKey(address)
  - setIsConnected(true)
  - localStorage.setItem("publicKey", address)
          │
          ▼
publicKey prop propagates to
Dashboard, SendTx, TxHistory
```

**Supported Wallets:** Freighter, Albedo, xBull, Lobstr, WalletConnect (mobile QR), RABET

**Mobile Session Restore:** On page reload, `App.jsx` calls `getConnectedAddress()` asynchronously. All child components guard against `publicKey = null` during this restore window by returning a loading state rather than an error screen.

---

## 5. AI Financial Analysis Flow

### 5.1 Chat Interface Flow

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│   User   │    │  React   │    │ Rust API │    │   Groq   │    │  Gemini  │
│          │    │  SPA     │    │          │    │   API    │    │   API    │
└────┬─────┘    └────┬─────┘    └────┬─────┘    └────┬─────┘    └────┬─────┘
     │               │               │               │               │
     │ Submit        │               │               │               │
     │ financial     │               │               │               │
     │ question      │               │               │               │
     │──────────────►│               │               │               │
     │               │ POST          │               │               │
     │               │ /api/analyze  │               │               │
     │               │──────────────►│               │               │
     │               │               │ Forward to    │               │
     │               │               │ Groq for      │               │
     │               │               │ structured    │               │
     │               │               │ JSON output   │               │
     │               │               │──────────────►│               │
     │               │               │               │ Returns JSON  │
     │               │               │               │ risk_level,   │
     │               │               │               │ budget_plan,  │
     │               │               │               │ tax_tips, etc │
     │               │               │◄──────────────│               │
     │               │               │ Forward to    │               │
     │               │               │ Gemini for    │               │
     │               │               │ narrative     │               │
     │               │               │ advice text   │               │
     │               │               │──────────────────────────────►│
     │               │               │◄──────────────────────────────│
     │               │               │ Merge JSON +  │               │
     │               │               │ advice text   │               │
     │               │               │ Save to DB    │               │
     │               │ 200 OK        │               │               │
     │               │ { analysis }  │               │               │
     │◄──────────────│◄──────────────│               │               │
     │ Renders       │               │               │               │
     │ results in    │               │               │               │
     │ chat UI       │               │               │               │
```

### 5.2 AI Response Schema

The Groq API is prompted to return a strict JSON structure:

```json
{
  "risk_level": "Medium",
  "tax_saving_suggestions": [
    "Utilize deductions under Section 80C",
    "Invest in tax-saving mutual funds"
  ],
  "budget_plan": {
    "needs": 50,
    "wants": 20,
    "savings": 30
  },
  "income_growth_suggestions": [
    "Invest in dividend-paying stocks",
    "Start a side business"
  ],
  "debt_strategy": "Prioritize home loan repayment"
}
```

This structured response is then enriched with a narrative explanation from the Gemini API before being returned to the frontend and persisted in MongoDB.

---

## 6. Transaction and Blockchain Interaction Flow

### 6.1 Sending XLM

```
┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
│   User   │   │  React   │   │ Stellar  │   │ Rust API │   │ Stellar  │
│          │   │  SPA     │   │  SDK     │   │          │   │ Testnet  │
└────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘
     │              │              │               │               │
     │ Enter dest   │              │               │               │
     │ + amount     │              │               │               │
     │─────────────►│              │               │               │
     │              │ loadAccount  │               │               │
     │              │─────────────►│               │               │
     │              │              │ GET /accounts │               │
     │              │              │──────────────────────────────►│
     │              │              │◄──────────────────────────────│
     │              │◄─────────────│               │               │
     │              │ Build        │               │               │
     │              │ Transaction  │               │               │
     │              │ (XDR format) │               │               │
     │              │─────────────►│               │               │
     │              │◄─────────────│               │               │
     │ Wallet signs │              │               │               │
     │ prompt       │              │               │               │
     │◄─────────────│              │               │               │
     │ Approve      │              │               │               │
     │─────────────►│              │               │               │
     │              │ signedXDR    │               │               │
     │              │─────────────►│               │               │
     │              │◄─────────────│               │               │
     │              │ POST /api/send               │               │
     │              │ { xdr: signedXDR }           │               │
     │              │──────────────────────────────►               │
     │              │              │               │ Submit XDR    │
     │              │              │               │──────────────►│
     │              │              │               │◄──────────────│
     │              │              │               │ { hash,       │
     │              │              │               │   ledger }    │
     │              │ 200 OK       │               │               │
     │              │ { hash, ledger, fee }         │               │
     │◄─────────────│◄─────────────────────────────│               │
     │ Shows        │              │               │               │
     │ success +    │              │               │               │
     │ refreshes    │              │               │               │
     │ balance      │              │               │               │
```

### 6.2 Fetching Balance and Transaction History

- **Balance:** `GET /api/balance/:address` → Backend calls Horizon API → Returns XLM balance.
- **Transactions:** `GET /api/transactions/:address` → Backend calls Horizon API → Returns last 10 transactions.
- **Fallback:** If the backend is unreachable, `stellarService.js` calls the Horizon API directly from the browser as a fallback.

---

## 7. Dashboard Functional Architecture

### 7.1 Component Hierarchy

```
Dashboard (page)
├── Props: { publicKey }
├── State: { profile, loading, error }
│
├── StatCard × 4
│   ├── Total Saved (from Stellar contract via getStats())
│   ├── Current Streak (from Stellar contract)
│   ├── Longest Streak (from Stellar contract)
│   └── Reward Points (from Stellar contract)
│
├── SavingsChart
│   └── Renders savings growth based on total_saved + streak
│
├── BudgetChart
│   └── Renders pie chart of needs / wants / savings from latest AI advice
│
├── RiskBadge
│   └── Displays risk level from latest AI analysis
│
├── AI Recommendations Panel
│   ├── Tax saving suggestions (from MongoDB profile)
│   └── Income growth suggestions (from MongoDB profile)
│
└── TxHistory (component)
    └── Props: { publicKey }
        └── Fetches last 10 transactions from Horizon via backend
```

### 7.2 Data Sources per Dashboard Section

| Section | Data Source | API Call |
|---|---|---|
| Stats (streak, rewards) | Stellar smart contract | `getStats()` via `stellarPiggy.js` |
| Total Saved | Stellar smart contract | `getStats()` via `stellarPiggy.js` |
| Budget Chart | MongoDB (`analyses` collection) | `GET /api/profile` |
| Risk Badge | MongoDB (`analyses` collection) | `GET /api/profile` |
| AI Recommendations | MongoDB (`analyses` collection) | `GET /api/profile` |
| Transaction History | Stellar Horizon API | `GET /api/transactions/:address` |
| Wallet Balance | Stellar Horizon API | `GET /api/balance/:address` |

---

## 8. Data Flow Diagram

```
                              FinWise Data Flow
                              =================

  ┌─────────────────────────────────────────────────────────────────────┐
  │                        BROWSER (React SPA)                         │
  │                                                                     │
  │  User Input ──► WalletConnect Kit ──► publicKey (localStorage)     │
  │                       │                     │                      │
  │                       ▼                     ▼                      │
  │             Wallet Events             App.jsx state                 │
  │           (connect/disconnect)    (publicKey, isConnected)          │
  │                                         │                          │
  │              ┌──────────────────────────┤                          │
  │              │          │               │          │               │
  │              ▼          ▼               ▼          ▼               │
  │          Dashboard  SendTx          TxHistory   Analyze            │
  │              │          │               │          │               │
  └──────────────┼──────────┼───────────────┼──────────┼───────────────┘
                 │          │               │          │
       ┌─────────┘    ┌─────┘       ┌──────┘    ┌─────┘
       │              │             │           │
       ▼              ▼             ▼           ▼
  GET /api/      POST /api/    GET /api/   POST /api/
  profile        send          transactions  analyze
       │              │             │           │
  ┌────┴──────────────┴─────────────┴───────────┴──────┐
  │                  RUST API (Railway)                 │
  │                                                     │
  │  Auth Middleware ──► Session Lookup ──► MongoDB     │
  │                                                     │
  │       │                   │                │        │
  │       ▼                   ▼                ▼        │
  │  MongoDB CRUD      Horizon API        AI APIs       │
  │  (users, sessions, (balance, txs,  (Groq + Gemini)  │
  │   analyses,         submit XDR)                     │
  │   piggy_banks)                                      │
  └─────────────────────────────────────────────────────┘
                 │                   │
                 ▼                   ▼
          MongoDB Atlas        Stellar Testnet
          (Persistent          (Horizon API —
           Storage)             Live chain data)
```

---

## 9. Deployment Architecture

### 9.1 Infrastructure Overview

```
┌─────────────────────┐         ┌─────────────────────────┐
│   Vercel (CDN)      │         │   Railway (PaaS)        │
│                     │         │                         │
│  React SPA          │  HTTPS  │  Rust / Actix-Web       │
│  Static Build       │◄───────►│  Docker Container       │
│  Global Edge CDN    │         │  Port 8080              │
│                     │         │  Region: europe-west4   │
│  Domain:            │         │  Replicas: 1            │
│  stellar-journey-   │         │                         │
│  to-mastery         │         │  Domain:                │
│  .vercel.app        │         │  finwise-backend        │
│                     │         │  .up.railway.app        │
└─────────────────────┘         └────────────┬────────────┘
                                              │
                         ┌────────────────────┼──────────────────┐
                         │                    │                  │
                         ▼                    ▼                  ▼
               ┌──────────────────┐  ┌───────────────┐  ┌──────────────┐
               │  MongoDB Atlas   │  │  Groq API     │  │ Stellar      │
               │  (M0 Free Tier)  │  │  (AI JSON)    │  │ Testnet      │
               │  Region: AWS     │  └───────────────┘  │ Horizon API  │
               │  eu-west-1       │                      └──────────────┘
               │                  │  ┌───────────────┐
               │  Collections:    │  │  Gemini API   │
               │  users           │  │  (AI Advice)  │
               │  sessions        │  └───────────────┘
               │  analyses        │
               │  piggy_banks     │  ┌───────────────┐
               └──────────────────┘  │  Google OAuth │
                                     │  (Auth)       │
                                     └───────────────┘
```

### 9.2 Build and Deploy Pipeline

**Frontend (Vercel)**
- Git push to `main` branch triggers automatic Vercel deployment.
- Build command: `npm run build`
- Output directory: `dist/`
- Environment variables configured in Vercel dashboard.

**Backend (Railway)**
- Git push to `main` branch triggers automatic Railway deployment.
- Railway detects Rust project via `Cargo.toml` and builds using `cargo build --release`.
- Container starts with `cargo run --release`.
- Environment variables set in Railway Variables tab.
- Runtime: `rust@1.88.0`, Region: `europe-west4-drams3a`

### 9.3 Environment Variables

**Backend (Railway)**

| Variable | Description |
|---|---|
| `MONGODB_URI` | MongoDB Atlas connection string |
| `SESSION_SECRET` | Secret key for session signing (min 64 bytes) |
| `GOOGLE_CLIENT_ID` | Google OAuth client ID |
| `GOOGLE_CLIENT_SECRET` | Google OAuth client secret |
| `GROQ_API_KEY` | Groq AI API key |
| `GEMINI_API_KEY` | Google Gemini API key |
| `STELLAR_NETWORK` | `TESTNET` or `MAINNET` |
| `PORT` | Injected by Railway (default 8080) |

**Frontend (Vercel)**

| Variable | Description |
|---|---|
| `VITE_API_URL` | Backend API base URL |

---

## 10. Security Considerations

### 10.1 Authentication Security

- **Password Hashing:** All passwords are hashed using `bcrypt` with `DEFAULT_COST` before storage. Plaintext passwords are never persisted.
- **Session Management:** Sessions are stored server-side in MongoDB, not in cookies. The cookie carries only a random UUID session ID.
- **Session Expiry:** Sessions have a 7-day TTL enforced at both the application and database layer (MongoDB TTL index).
- **Cookie Flags:** Session cookies are set with `HttpOnly`, `Secure`, and `SameSite=None` to prevent XSS and enable cross-origin requests.
- **Google OAuth Users:** Google-authenticated users have an empty password field. Password login is blocked for these accounts.

### 10.2 API Security

- **CORS:** The backend explicitly allows only the production frontend origin (`stellar-journey-to-mastery.vercel.app`). All other origins are rejected.
- **Input Validation:** JSON payloads are validated using Rust's type system and `serde`. Malformed requests return `400 Bad Request` with structured error messages.
- **No `.unwrap()` in Production Paths:** All critical code paths use proper error handling with `match` or `?` — panics do not propagate to callers.
- **Rate Limiting:** To be implemented — see Section 12.

### 10.3 Blockchain Security

- **Client-Side Signing:** Private keys never leave the user's wallet. Transactions are built on the backend but signed client-side using the Stellar Wallets Kit.
- **XDR Validation:** The backend submits only signed XDR payloads received from the client. It does not construct or sign transactions on behalf of users.
- **Testnet Isolation:** The application currently operates exclusively on Stellar Testnet. Mainnet migration requires explicit environment variable change and additional audit.

### 10.4 Data Security

- **MongoDB Atlas:** Database access is restricted by IP allowlist and requires credentials. Connection string is stored as an environment variable, never committed to source control.
- **Secrets Management:** All API keys and secrets are stored as environment variables in Railway and Vercel. No secrets are present in the codebase.
- **Sensitive Data Exposure:** Wallet private keys, seed phrases, and raw passwords are never logged or transmitted to the backend.

---

## 11. Scalability Considerations

### 11.1 Current Constraints

| Component | Current Limit | Bottleneck |
|---|---|---|
| Railway (Free Tier) | 1 replica, shared CPU | Concurrent request handling |
| MongoDB Atlas M0 | 512 MB storage, ~500 connections | Connection pool under high load |
| Groq API | Rate-limited by plan tier | AI analysis throughput |
| Stellar Testnet | Public shared testnet | Not suitable for production load |

### 11.2 Horizontal Scaling Path

**Backend Workers**
- Actix-Web is configured with 2 workers by default.
- Sessions are stored in MongoDB (not in-memory), so adding workers or replicas does not cause session loss.
- Scaling to N replicas on Railway requires no code changes.

**Database Scaling**
- Upgrade MongoDB Atlas cluster from M0 (free) to M10+ for production workloads.
- Add connection pooling limits in the MongoDB driver to prevent Atlas M0 connection exhaustion.

**Frontend**
- Vercel CDN scales automatically. No action required for frontend scaling.

**Caching**
- Balance and transaction history responses can be cached (Redis or in-memory) to reduce Horizon API calls under load.
- AI analysis results are already persisted in MongoDB — repeated requests for the same data hit the database, not the AI API.

---

## 12. Future Architecture Improvements

### 12.1 Short Term

- **JWT Authentication:** Migrate from cookie-based sessions to JWT tokens stored in `localStorage`. This eliminates cross-origin cookie issues on mobile Safari and simplifies stateless horizontal scaling.
- **Rate Limiting:** Add per-IP rate limiting middleware to all public API endpoints to prevent abuse.
- **Request Logging:** Integrate structured JSON logging with request IDs for distributed tracing.
- **Input Sanitisation:** Add explicit input sanitisation middleware beyond Rust's type-level validation.

### 12.2 Medium Term

- **Redis Session Store:** Replace MongoDB session store with Redis for lower-latency session lookups.
- **WebSocket Support:** Add WebSocket connection for real-time transaction notifications and live balance updates.
- **Caching Layer:** Introduce response caching for balance and transaction endpoints (TTL: 30 seconds) to reduce Horizon API dependency.
- **CI/CD Pipeline:** Add GitHub Actions workflows for automated testing, linting, and deployment on pull requests.
- **Stellar Mainnet Migration:** Environment-flagged mainnet support with additional security audit before release.

### 12.3 Long Term

- **Microservices Decomposition:** Split the monolithic Rust API into discrete services — Auth Service, Stellar Service, AI Service — each independently deployable and scalable.
- **Smart Contract Expansion:** Expand on-chain logic for the Piggy Bank feature using Soroban (Stellar's smart contract platform) for transparent, trustless savings rules.
- **Multi-Chain Support:** Abstract the blockchain layer to support additional networks (e.g., Ethereum, Solana) through a unified wallet interface.
- **Mobile Application:** React Native port of the frontend for native iOS and Android experience, reusing the same backend API.
- **AI Model Fine-Tuning:** Fine-tune the financial analysis model on domain-specific Indian financial data for improved relevance of tax and investment advice.

---

## Appendix: Technology Stack Summary

| Layer | Technology | Version | Purpose |
|---|---|---|---|
| Frontend Framework | React | 18.x | SPA rendering |
| Frontend Hosting | Vercel | — | CDN + CI/CD |
| Backend Language | Rust | 1.88.0 | API server |
| Backend Framework | Actix-Web | 4.x | HTTP server |
| Backend Hosting | Railway | — | PaaS container |
| Database | MongoDB Atlas | 7.x | Persistent storage |
| Blockchain | Stellar Testnet | — | XLM transactions |
| Blockchain API | Horizon API | — | Chain data access |
| Blockchain SDK | Stellar SDK (JS) | — | Frontend tx building |
| Wallet Integration | Stellar Wallets Kit | — | Multi-wallet support |
| Wallet (Extension) | Freighter | — | Browser extension wallet |
| Wallet (Mobile) | WalletConnect | — | Mobile QR wallet |
| AI Service 1 | Groq API | — | Structured JSON analysis |
| AI Service 2 | Gemini API | — | Narrative advice text |
| OAuth Provider | Google OAuth 2.0 | — | Social authentication |
| HTTP Client | Axios | — | Frontend API calls |
| CSS Framework | Tailwind CSS | 3.x | UI styling |

---

