# FinWise AI – Smart Financial Advisory dApp

> **AI-powered financial advisory platform built on the Stellar blockchain — combining personalised financial guidance with on-chain reward mechanics.**

---

## Live Demo

** [https://finwise-ai-advisor.vercel.app](https://finwise-ai-advisor.vercel.app)**

---

## Demo Video

**Full MVP walkthrough — wallet connect, AI analysis, XLM send, piggy bank streak:**

https://github.com/user-attachments/assets/e0b15652-6c5d-4847-a7f5-971fb8a2b5cf

https://github.com/user-attachments/assets/5c90d0f8-df5a-4c60-a239-2282336029b6

> *The video demonstrates: landing page → wallet connection → Google OAuth → AI financial analysis → XLM send transaction → transaction history → piggy bank savings streak → dashboard stats.*

---

## Table of Contents

- [Overview](#-overview)
- [Features](#-features)
- [Screenshots](#-screenshots)
- [Live Demo](#-live-demo)
- [Demo Video](#-demo-video)
- [Smart Contract](#-smart-contract)
- [Verified Wallet Addresses](#-verified-wallet-addresses)
- [User Feedback](#-user-feedback)
- [Technology Stack](#-technology-stack)
- [Architecture](#-architecture)
- [Project Structure](#-project-structure)
- [Getting Started](#-getting-started)
- [Environment Variables](#-environment-variables)
- [API Reference](#-api-reference)
- [Mobile Responsive View](#-mobile-responsive-view)
- [CI/CD Status](#-cicd-status)
- [Contributing](#-contributing)
- [License](#-license)
- [Advanced Feature](#-Advanced Feature)
- [Data Indexing](#-Data Indexing)
- [Monitoring & Logs](#-Monitoring & Logs)
---


## Overview

FinWise AI is an AI-powered financial advisory platform that helps users manage income, savings, expenses, taxes, and long-term financial planning. By combining **AI-driven recommendations** with **country-specific financial logic**, the platform generates actionable insights to improve budgeting, debt management, and wealth growth.

The platform features a **Blockchain-based Reward System** built on Stellar to incentivize disciplined saving habits through streaks, reward points, and on-chain milestone tracking.

---

## Features

### 1. Personalised Financial Advice
Users input their income, location, family details, and financial challenges to receive:
- **Budget Plans** — Custom-tailored spending limits using the 50/20/30 framework
- **Tax Strategies** — Guidance based on local tax laws and regulations
- **Debt Reduction** — Optimised plans to clear liabilities faster
- **Income Growth** — Recommendations for investment and side income opportunities

### 2. Country-Specific Logic
A built-in rule engine adjusts financial advice based on the specific tax codes and economic regulations of the user's country (currently optimised for India with Section 80C, HRA, and related deductions).

### 3. Token Reward System (Saving Motivation Engine)
To encourage consistency, FinWise AI rewards users for maintaining saving streaks:
- **Commit & Save** — Users set daily or monthly saving goals
- **Earn Tokens** — Consistent streaks trigger reward distributions via Soroban smart contracts
- **Utility** — Use tokens to unlock premium AI advisory features and advanced financial modules

### 4. Wallet Integration
- Connect via **Freighter**, **xBull**, **Albedo**, **Lobstr**, or **WalletConnect** (mobile QR)
- View real-time XLM balance
- Send XLM transactions directly from the dashboard

### 5. Transaction History
- Full on-chain transaction log via the Stellar Horizon API
- Viewable with links to Stellar Expert explorer

### 6. Piggy Bank Streak
- On-chain savings streak tracking via Soroban smart contract
- Reward points accumulated per streak milestone
- Visual streak counter and savings growth chart on dashboard

### 7. Multi-Method Authentication
- Email / password registration and login
- Google OAuth (one-click sign-in)
- Wallet-based identity (Stellar public key)

---

## Screenshots

### Home Page
<img width="1904" height="928" alt="image" src="https://github.com/user-attachments/assets/8ee94745-8e18-44cf-97d5-a1970f12ce0a" />


### Wallet Connection with Google Sign-In Option
<img width="1918" height="929" alt="Wallet Connection" src="https://github.com/user-attachments/assets/149b1ef3-180f-4b11-b68a-eb45b9e8ee44" />

### Google OAuth Sign-In
<img width="1919" height="928" alt="Google OAuth" src="https://github.com/user-attachments/assets/46d0b786-268b-4886-80e4-f7da2d07bf84" />

### Balance & Send XLM
<img width="1919" height="929" alt="image" src="https://github.com/user-attachments/assets/c616c8a1-0ad4-43a4-b3f7-0b41a4532af0" />


### Transaction History
<img width="1919" height="928" alt="image" src="https://github.com/user-attachments/assets/f060f8a2-d239-4e20-a28f-3193a1ab6185" />


### AI Financial Analysis & Advisor
<img width="1902" height="927" alt="AI Analysis" src="https://github.com/user-attachments/assets/67c35e66-1da2-46d4-a48c-37d1b2988180" />

### Profile Dashboard
<img width="1919" height="929" alt="image" src="https://github.com/user-attachments/assets/44aefbc8-8cf0-4d38-af4d-3494e206d72d" />


### Piggy Bank — Savings Streak
<img width="1919" height="926" alt="image" src="https://github.com/user-attachments/assets/1b923b71-1fae-4bf2-b7c7-20c5ffdc9307" />


### About Section
<img width="1919" height="930" alt="image" src="https://github.com/user-attachments/assets/161431b4-9177-49b7-9c0d-586e050c63dc" />

### Matric Dashboard
<img width="1899" height="928" alt="image" src="https://github.com/user-attachments/assets/21ae064a-dc01-4491-8052-ee8b6e126a10" />

---

## Mobile Responsive View

### Home Page
<img width="466" height="817" alt="image" src="https://github.com/user-attachments/assets/a0ffd330-6408-4a6a-ab13-3d2cee42b5b0" />

### Wallet Connection with Google Sign-In Option
<img width="464" height="817" alt="image" src="https://github.com/user-attachments/assets/7ee5dc8e-a75d-4613-a3f7-79658da7648b" />

### About Section
<img width="466" height="825" alt="image" src="https://github.com/user-attachments/assets/f6b2765e-8a5d-4d01-81c1-042fbed4f91e" />

### Menu 
<img width="461" height="823" alt="image" src="https://github.com/user-attachments/assets/592e521d-d640-4f5e-bfd3-6160f9470702" />

### AI Financial Analysis & Advisor
<img width="469" height="817" alt="image" src="https://github.com/user-attachments/assets/bb19be08-0dec-45b9-8ec4-be68dcbb7e04" />

### Balance & Send XLM
<img width="463" height="823" alt="image" src="https://github.com/user-attachments/assets/b135c54c-bb3f-4eb5-ba84-d1d9e277b3f3" />

### Transaction History 
<img width="464" height="822" alt="image" src="https://github.com/user-attachments/assets/4b97c229-f1f1-4fbf-8d73-8f1e0fec9499" />

### Profile Dashboard
<img width="467" height="824" alt="image" src="https://github.com/user-attachments/assets/1429c02c-096d-406f-ad2d-6917653d6088" />

### Piggy Bank — Savings Streak
<img width="465" height="815" alt="image" src="https://github.com/user-attachments/assets/f50d19c5-3ecc-40e6-a6ed-3118f6527a18" />

### Matric Dashboard
<img width="463" height="823" alt="image" src="https://github.com/user-attachments/assets/161c806e-999c-4da7-ad83-b468577ee971" />



---

## ⛓ Smart Contract

| Field | Value |
|---|---|
| **Network** | Stellar Testnet |
| **Contract ID** | `CCQ62QMQSDDXMBD5PFQD4HJODSLMEFHQVCAQEWLEI5YS3T7H7X24YSBK` |
| **Deployment Tx Hash** | `fe0bd61f5fe0b580d9e13147bcb3b870675fd95f198f03e2a31ff38497a830b6` |
| **Language** | Rust (Soroban SDK) |
| **Explorer** | [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCQ62QMQSDDXMBD5PFQD4HJODSLMEFHQVCAQEWLEI5YS3T7H7X24YSBK) |

### Token Details

This project uses native **XLM** on Stellar Testnet. No custom token has been deployed. Reward points and streak data are tracked via the Soroban smart contract state.

---

## Verified Wallet Addresses

The following wallet addresses have interacted with the FinWise platform and are verifiable on the Stellar Testnet Explorer.

| # | Wallet Address | Explorer Link | User Email |
|---|---|---|---|
| 1 | `GCDCQIYZ7N5M4ND625BK7FGGGANTJJDW5KFFUMIWEOOYCIR27WA7ZT6M` | [View](https://stellar.expert/explorer/testnet/account/GCDCQIYZ7N5M4ND625BK7FGGGANTJJDW5KFFUMIWEOOYCIR27WA7ZT6M) | deepakgupta4142434445464748@gmail.com |
| 2 | `GDD5YF3VBRFACXBEOL572U4BOHUK2XAYVQ33H56SBMDSJSQX4ICAUKLO` | [View](https://stellar.expert/explorer/testnet/account/GDD5YF3VBRFACXBEOL572U4BOHUK2XAYVQ33H56SBMDSJSQX4ICAUKLO) | sutarsudarshan621@gmail.com |
| 3 | `GC3U52PVKJ33U424KGFKGCDPVBJGFDOANKAOEFBPT5BUDSMVJXVMXZWG` | [View](https://stellar.expert/explorer/testnet/account/GC3U52PVKJ33U424KGFKGCDPVBJGFDOANKAOEFBPT5BUDSMVJXVMXZWG) | trupti.khandbhor22@gmail.com |
| 4 | `GC6U7V64P7KTUSPFXO3WWU6V7AYBRLRJW6PPRVQAL5YXHZM3HHTX2EP3` | [View](https://stellar.expert/explorer/testnet/account/GC6U7V64P7KTUSPFXO3WWU6V7AYBRLRJW6PPRVQAL5YXHZM3HHTX2EP3) | vg6494391@gmail.com |
| 5 | `GDFNCO4KZN7VZJTCYWPJYQN2WJGKJY46PN2AR6FA2WB35YMU3Q2U6JQE` | [View](https://stellar.expert/explorer/testnet/account/GDFNCO4KZN7VZJTCYWPJYQN2WJGKJY46PN2AR6FA2WB35YMU3Q2U6JQE) | pratikshaspark12@gmail.com |
| 6 | `GACUAJJ5XYAOHFRNASQU472IEZHMU5G37CLNPGKA7HK55MEFZV6ZJQ45` | [View](https://stellar.expert/explorer/testnet/account/GACUAJJ5XYAOHFRNASQU472IEZHMU5G37CLNPGKA7HK55MEFZV6ZJQ45) | vedmalkunaik@gmail.com |
| 7 | `GBFP6DQ6V2TODBDCEFHO6R5NAX3FDPNSREKQBFQUZSY5FWJR3HXVQO6Q` | [View](https://stellar.expert/explorer/testnet/account/GBFP6DQ6V2TODBDCEFHO6R5NAX3FDPNSREKQBFQUZSY5FWJR3HXVQO6Q) | shrirammasalge8@gmail.com |

> All addresses are on **Stellar Testnet**. You can verify transactions at [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet).

---

##  User Feedback

The following feedback was collected from real users who tested the FinWise AI platform during the MVP phase.

---

### Feedback Summary

https://docs.google.com/spreadsheets/d/e/2PACX-1vRhZwDws89-sNGKGNdCNaYpNcOtZtE6CvGjqAAOEAOA5Hvdo2ShF4S3veYncBapYMoq_bMTycM9HAwa/pubhtml

---

### Changes made based on the feedback of the user

| # | User Email                                                      | Feedback                                                                                                                     | Commit ID                                |
| - | --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| 1 | [pratikshaspark12@gmail.com](mailto:pratikshaspark12@gmail.com) | Improve the Send XML page UI; other functionalities are working well.                                                        | 1286e69426c12bcf26f0cdb0b077b8464cf12d77 |
| 2 | [vedmalkunaik@gmail.com](mailto:vedmalkunaik@gmail.com)         | Dashboard failed to load. Google sign-in is not working. Piggy bank and AI fund analysis features are functioning correctly. | c8d28573ac21e518b4798de73257768748751c12 |


## 🛠 Technology Stack

### Frontend
| Technology | Purpose |
|---|---|
| React.js (Vite) | SPA framework |
| Tailwind CSS | UI styling |
| Axios | HTTP client |
| Stellar Wallets Kit | Multi-wallet connector |
| Stellar SDK (JS) | Transaction building |
| React Router | Client-side routing |

### Backend
| Technology | Purpose |
|---|---|
| Rust / Actix-Web | REST API server |
| MongoDB Atlas | Persistent database |
| Groq API | Structured AI JSON analysis |
| Gemini API | Narrative financial advice |
| Google OAuth 2.0 | Social authentication |
| bcrypt | Password hashing |

### Blockchain
| Technology | Purpose |
|---|---|
| Stellar Testnet | Blockchain network |
| Soroban (Rust) | Smart contract platform |
| Stellar SDK | Account and transaction management |
| Horizon API | On-chain data queries |
| WalletConnect | Mobile wallet QR connection |
| Freighter | Browser extension wallet |

### Infrastructure
| Technology | Purpose |
|---|---|
| Vercel | Frontend hosting + CDN |
| Railway | Backend container hosting |
| MongoDB Atlas | Cloud database (M0) |
| GitHub Actions | CI/CD pipeline |

---
## Advanced Feature

### AI Financial Analysis & Advisor: AI-powered daily savings and smart budgeting companion designed for students and young earners to build consistent financial habits.
<img width="469" height="817" alt="image" src="https://github.com/user-attachments/assets/bb19be08-0dec-45b9-8ec4-be68dcbb7e04" />

### Piggy Bank — Savings Streak: Users can deposit daily savings on the blockchain, creating a consistent saving habit.
<img width="465" height="815" alt="image" src="https://github.com/user-attachments/assets/f50d19c5-3ecc-40e6-a6ed-3118f6527a18" />

---
## Data Indexing

### User data indexing
<img width="1806" height="727" alt="Screenshot 2026-03-31 215308" src="https://github.com/user-attachments/assets/fe1639b4-eacf-41d8-b6a8-063c5d5b5ea6" />

### Transaction data indexing
<img width="1806" height="658" alt="Screenshot 2026-03-31 215321" src="https://github.com/user-attachments/assets/47075224-3562-4396-9530-dedbb169ef26" />

### Profiles Data indexing
<img width="1796" height="664" alt="Screenshot 2026-03-31 215336" src="https://github.com/user-attachments/assets/99e2e6f6-259b-4cec-a5b8-c7f2852a8bb9" />

### Analysis data indexing 
<img width="1786" height="660" alt="Screenshot 2026-03-31 215400" src="https://github.com/user-attachments/assets/981e2eb3-3525-422e-b2ab-74f634397c4a" />

---
## Monitoring & Logs

Application monitoring is implemented using Render's built-in logging system.

### Features
- Real-time HTTP request logging
- API usage tracking across endpoints
- Error visibility with status codes
- CORS preflight monitoring (OPTIONS requests)

### Proof
<img width="1514" height="709" alt="image" src="https://github.com/user-attachments/assets/457acce2-8e36-4f0c-8861-2852e9f7cb83" />

<img width="1432" height="707" alt="image" src="https://github.com/user-attachments/assets/2b34630c-5834-43a2-ad7a-49eb07ef095c" />

### Sample Logs
- GET /api/metrics → 200 OK
- GET /api/check-auth → 200 OK
- OPTIONS /api/metrics → 200 OK
- GET /api/test-error → 500 Internal Server Error

### Notes
Logs include timestamps, request origin, response status, and latency, providing full visibility into backend behavior.
---
## 🏗 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    React SPA (Vercel)                       │
│  Hero | Analyze | Dashboard | SendTx | TxHistory | Signin   │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTPS REST
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                 Rust / Actix-Web API (Railway)              │
│  Auth | Profile | Analyze | Stellar | Piggy | Health        │
│                         │                                   │
│         ┌───────────────┼───────────────┐                   │
│         ▼               ▼               ▼                   │
│   MongoDB Atlas     Horizon API      Groq + Gemini          │
│   (users, sessions  (balance, txs,   (AI analysis)          │
│    analyses,         submit XDR)                            │
│    piggy_banks)                                             │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│               Stellar Testnet + Soroban                     │
│         Smart Contract: Piggy Bank Streak Tracker           │
└─────────────────────────────────────────────────────────────┘
```

For the full technical architecture document, see [`docs/architecture.md`](docs/architecture.md).

---

## 📁 Project Structure
```text
STELLAR/
│
├── finwise-frontend/                   # React.js application (Vite)
│   ├── src/
│   │   ├── components/                 # Reusable UI components
│   │   ├── hooks/                      # Custom React hooks
│   │   ├── pages/                      # Route-level page components
│   │   ├── services/                   # API client, wallet manager, Stellar SDK
│   │   ├── App.css                     # Global app styles
│   │   ├── App.jsx                     # Root component, routing, wallet state
│   │   ├── index.css                   # Base CSS / Tailwind imports
│   │   └── main.jsx                    # Vite entry point
│   ├── public/                         # Static assets
│   ├── index.html                      # HTML shell
│   ├── .env                            # Frontend environment variables
│   ├── .gitignore
│   ├── .npmrc
│   ├── eslint.config.js                # ESLint configuration
│   ├── package.json                    # NPM dependencies
│   ├── package-lock.json
│   ├── postcss.config.js               # PostCSS configuration
│   ├── tailwind.config.js              # Tailwind CSS configuration
│   ├── vercel.json                     # Vercel deployment config
│   └── vite.config.js                  # Vite build configuration
│
├── finwise-backend/                    # Rust / Actix-Web API server
│   ├── src/
│   │   ├── db/
│   │   │   └── mod.rs                  # MongoDB connection pool
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── analysis.rs             # Analysis document schema
│   │   │   ├── piggy.rs                # Piggy Bank document schema
│   │   │   ├── profile.rs              # User profile schema
│   │   │   └── user.rs                 # User account schema
│   │   ├── routes/
│   │   │   ├── mod.rs
│   │   │   ├── analyze.rs              # POST /api/analyze — AI analysis
│   │   │   ├── health.rs               # GET /health — health check
│   │   │   ├── piggy.rs                # Piggy Bank deposit + stats
│   │   │   ├── profile.rs              # GET /api/profile
│   │   │   └── routes.rs               # Stellar balance / send / transactions
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── nlp_service.rs          # NLP / AI prompt construction
│   │   │   ├── ollama_service.rs       # Ollama / Groq / Gemini integration
│   │   │   └── piggy_service.rs        # Piggy Bank business logic
│   │   ├── utils/
│   │   │   └── mod.rs                  # Shared utility functions
│   │   ├── config.rs                   # Environment variable loading
│   │   ├── main.rs                     # Server bootstrap, middleware, routes
│   │   └── stellar.rs                  # Stellar SDK — balance, send, history
│   ├── .env                            # Backend environment variables
│   ├── Cargo.lock
│   ├── Cargo.toml                      # Rust dependencies
│   └── rust-toolchain.toml             # Pinned Rust toolchain version
│
├── finwise-contract/                   # Soroban Smart Contract (Rust)
│   ├── src/
│   │   ├── errors.rs                   # Contract error types
│   │   ├── events.rs                   # Contract event definitions
│   │   ├── lib.rs                      # Contract entry point + public functions
│   │   ├── storage.rs                  # On-chain storage read/write helpers
│   │   ├── test.rs                     # Contract unit tests
│   │   ├── types.rs                    # Shared contract data types
│   │   └── utils.rs                    # Contract utility functions
│   ├── Cargo.lock
│   └── Cargo.toml                      # Contract dependencies (Soroban SDK)
│
├── docs/
│   └── architecture.md                 # Full system architecture document
│
├── .gitattributes
├── .gitignore
└── README.md
```
```

---

## 🚀 Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.70+
- MongoDB Atlas account
- Stellar Testnet account (funded via [friendbot](https://friendbot.stellar.org))

### Frontend Setup

```bash
cd finwise-frontend
npm install
npm run dev
```

### Backend Setup

```bash
cd finwise-backend
cargo run
```

### Smart Contract Deployment

```bash
cd finwise-contracts
cargo build --target wasm32-unknown-unknown --release
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/finwise_contracts.wasm \
  --network testnet \
  --source <YOUR_KEYPAIR>
---

## 📡 API Reference

| Method | Endpoint | Description | Auth |
|---|---|---|---|
| `POST` | `/api/signup` | Register new user | — |
| `POST` | `/api/login` | Email/password login | — |
| `POST` | `/api/logout` | End session | ✓ |
| `GET` | `/api/check-auth` | Validate session | — |
| `GET` | `/api/profile` | User profile + analysis history | ✓ |
| `POST` | `/api/analyze` | Submit financial data for AI analysis | ✓ |
| `GET` | `/api/balance/:address` | Fetch XLM balance | ✓ |
| `GET` | `/api/transactions/:address` | Fetch transaction history | ✓ |
| `POST` | `/api/send` | Submit signed XLM transaction | ✓ |
| `POST` | `/api/piggy/deposit` | Record savings deposit | ✓ |
| `GET` | `/api/piggy/stats/:user_id` | Get streak stats | ✓ |
| `GET` | `/auth/google` | Initiate Google OAuth | — |
| `GET` | `/auth/google/callback` | Google OAuth callback | — |
| `GET` | `/health` | Service health check | — |

---

## ⚙️ CI/CD Status

![CI](https://github.com/sudha16-sketc/stellar-journey-to-mastery/actions/workflows/create%20main.yml/badge.svg)

Deployments are triggered automatically on push to `main`:
- **Frontend** → Vercel (automatic Vite build + deploy)
- **Backend** → Railway (automatic Rust build + container deploy)

---

## For detail Architecture
  prefer the architecture.md file in the repo


