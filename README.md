# FinWise AI – Smart Financial Advisory dApp

> **AI-powered financial advisory platform built on the Stellar blockchain — combining personalised financial guidance with on-chain reward mechanics.**

---

## Live Demo

** [https://stellar-journey-to-mastery.vercel.app](https://stellar-journey-to-mastery.vercel.app)**

---

## Demo Video

**Full MVP walkthrough — wallet connect, AI analysis, XLM send, piggy bank streak:**
https://github.com/user-attachments/assets/b80716df-a6a6-4ff2-9cca-1af8eb5dcd0a
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
<img width="1902" height="933" alt="Home Page" src="https://github.com/user-attachments/assets/0c5c8ffd-06a0-46ff-a273-3337ed55db52" />

### Wallet Connection with Google Sign-In Option
<img width="1918" height="929" alt="Wallet Connection" src="https://github.com/user-attachments/assets/149b1ef3-180f-4b11-b68a-eb45b9e8ee44" />

### Google OAuth Sign-In
<img width="1919" height="928" alt="Google OAuth" src="https://github.com/user-attachments/assets/46d0b786-268b-4886-80e4-f7da2d07bf84" />

### Balance & Send XLM
<img width="1919" height="929" alt="Balance and Send" src="https://github.com/user-attachments/assets/c32226a7-2110-4c77-ac31-c2706492890a" />

### Transaction History
<img width="1919" height="932" alt="Transaction History" src="https://github.com/user-attachments/assets/07f15af3-3142-4e5b-8f71-d94f6eff7fb5" />

### AI Financial Analysis & Advisor
<img width="1902" height="927" alt="AI Analysis" src="https://github.com/user-attachments/assets/67c35e66-1da2-46d4-a48c-37d1b2988180" />

### Dashboard
<img width="1896" height="931" alt="Dashboard" src="https://github.com/user-attachments/assets/08d0166d-7478-44ac-9861-0bdb2207a5c2" />

### Piggy Bank — Savings Streak
<img width="1900" height="923" alt="Piggy Bank" src="https://github.com/user-attachments/assets/b174f393-496e-42b1-a634-8543e31c29c3" />

### About Section
<img width="1919" height="928" alt="About Section" src="https://github.com/user-attachments/assets/3f77b8d1-3f16-4100-acc5-863be33fa4ca" />

---

## Mobile Responsive View

<img width="754" height="932" alt="Mobile 1" src="https://github.com/user-attachments/assets/f0bc2a5e-1dd8-4173-9589-897506bda4f0" />
<img width="795" height="932" alt="Mobile 2" src="https://github.com/user-attachments/assets/963e6423-f13c-4ce7-8f8a-2cd8b6749b08" />
<img width="790" height="931" alt="Mobile 3" src="https://github.com/user-attachments/assets/404c74b6-d942-4814-91db-d3dc18678fb2" />
<img width="841" height="934" alt="Mobile 4" src="https://github.com/user-attachments/assets/56fec22a-cd32-4858-984b-00dbed29b4b5" />

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

| # | Wallet Address | Explorer Link | Role |
|---|---|---|---|
| 1 | `GA3WKZPAEMGMMMB5PJKWPITIFD54SECIID3V4QKNB3ARROYQNCKHBPI2` | [View](https://stellar.expert/explorer/testnet/account/GA3WKZPAEMGMMMB5PJKWPITIFD54SECIID3V4QKNB3ARROYQNCKHBPI2) | sudhakarsutar101@gmail.com |
| 2 | `GBBCIA2SJ4ZCNWPB5O447KCNO4NIHMLF3XMVK3DZOMQD4RIJ3HCDMXE5` | [View](https://stellar.expert/explorer/testnet/account/GBBCIA2SJ4ZCNWPB5O447KCNO4NIHMLF3XMVK3DZOMQD4RIJ3HCDMXE5) | mrameen330@gmail.com |
| 3 | `GBROW5BI5VDRZ4ZKO432LAPTTDODYQCJQXTCUXAEBWGTPG7JIGLVB5M3` | [View](https://stellar.expert/explorer/testnet/account/GBROW5BI5VDRZ4ZKO432LAPTTDODYQCJQXTCUXAEBWGTPG7JIGLVB5M3) | blockchainerjainparam@gmail.com |
| 4 | `GCPB676PALIONHBTQUEQ3FOAXYAAQ4ADMBMDLGVUGZNWXPYZROOX4TP4` | [View](https://stellar.expert/explorer/testnet/account/GCPB676PALIONHBTQUEQ3FOAXYAAQ4ADMBMDLGVUGZNWXPYZROOX4TP4) | deepakgupta4142434445464748@gmail.com |
| 5 | `GCXF754WQZ5ELFJMQTWJDOTUR6MQQXHDVEFEYDQBIQICQ6H7XLRM4VRZ` | [View](https://stellar.expert/explorer/testnet/account/GCXF754WQZ5ELFJMQTWJDOTUR6MQQXHDVEFEYDQBIQICQ6H7XLRM4VRZ) | kalbhorsppu12@gmail.com |

> All addresses are on **Stellar Testnet**. You can verify transactions at [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet).

---

##  User Feedback

The following feedback was collected from real users who tested the FinWise AI platform during the MVP phase.

---

### Feedback Summary

| User Email | Key Feedback |
|------------|--------------|
| kalbhorsppu12@gmail.com | Sign-in works on desktop browsers but fails on mobile browsers. |
| deepakgupta4142434445464748@gmail.com | UI is clean, but wallet connection could be faster. |
| blockchainerjainparam@gmail.com | Transaction history feature is very useful. |
| mrameen330@gmail.com | AI financial analysis feature is interesting and helpful. |
| sudhakarsutar101@gmail.com | There is an issue with wallet connection inside the dashboard. |

---


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


