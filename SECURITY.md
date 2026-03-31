# Security Policy

## Overview

FinWise AI is a financial advisory dApp built on the Stellar blockchain. Because it handles
sensitive financial data and on-chain transactions, security is a top priority. This document
outlines our security practices, known considerations, and how to responsibly report vulnerabilities.

---

## Supported Versions

| Version   | Supported          |
|-----------|--------------------|
| `main`    | ✅ Active support  |
| Older tags | ❌ Not supported  |

Only the latest commit on the `main` (or `resubmission`) branch receives security updates.

---

## Reporting a Vulnerability

If you discover a security vulnerability, **do not open a public GitHub issue.**

Please report it privately via email:

**security@finwise.ai** *(or the maintainer's email on the GitHub profile)*

Include the following in your report:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (optional)

You can expect an acknowledgement within **48 hours** and a resolution timeline within **7 days**
for critical issues.

---

## Security Architecture

### Authentication

| Mechanism         | Details                                                                 |
|-------------------|-------------------------------------------------------------------------|
| JWT               | Signed with `HS256`, expiry 1 hour, secret stored in environment variable |
| HttpOnly Cookie   | `Secure=true`, used for Google OAuth redirect flow                      |
| Bearer Token      | JWT sent via `Authorization: Bearer` header for API clients             |
| Google OAuth 2.0  | CSRF state token validated on every callback via short-lived cookie     |
| Password hashing  | `bcrypt` with `DEFAULT_COST` (10 rounds)                                |

### Rate Limiting

| Scope          | Limit                        |
|----------------|------------------------------|
| Global routes  | 20 requests / 60 sec per IP  |
| Auth routes    | 5 requests / 10 sec per IP   |

Implemented via `actix-governor` at the middleware layer.

### CORS

- Restricted to the exact frontend origin: `https://finwise-ai-advisor.vercel.app`
- `supports_credentials()` enabled for cookie-based auth
- Methods limited to `GET`, `POST`, `OPTIONS`

### Security Headers

Applied globally via the `SecurityHeaders` middleware (`app_middleware::security_headers`):

| Header                      | Value                          |
|-----------------------------|--------------------------------|
| `X-Content-Type-Options`    | `nosniff`                      |
| `X-Frame-Options`           | `DENY`                         |
| `X-XSS-Protection`          | `1; mode=block`                |
| `Referrer-Policy`           | `strict-origin-when-cross-origin` |
| `Content-Security-Policy`   | Restricts scripts, frames, objects |

---

## Blockchain Security

### Stellar Transactions
- All transactions are **signed client-side** — the backend never holds private keys
- XDR (transaction envelope) is submitted to the backend only after client signing
- Transactions are broadcast to **Stellar Testnet** only (no Mainnet funds at risk)

### Smart Contract (Soroban)
- Contract ID: `CCQ62QMQSDDXMBD5PFQD4HJODSLMEFHQVCAQEWLEI5YS3T7H7X24YSBK`
- Deployed on **Stellar Testnet** — not Mainnet
- Contract handles only streak/reward state — no token custody
- Contract source is open and auditable in `/finwise-contract/src/`

### Wallet Integration
- Private keys never leave the user's wallet extension (Freighter, xBull, Albedo, etc.)
- WalletConnect QR flow uses the kit's own secure Web Component — no key exposure to React

---

## Data Security

### Sensitive Data Handling
- Passwords are hashed with `bcrypt` before storage — plaintext passwords are never persisted
- Google OAuth users have an empty password field — they cannot log in via email/password
- JWT secret is loaded from environment variables — never hardcoded
- MongoDB credentials stored in environment variables — never in source code

### What Is Stored in MongoDB
| Collection    | Data Stored                                          | Sensitive? |
|---------------|------------------------------------------------------|------------|
| `users`       | username, email, bcrypt hash, wallet address         | ⚠️ Yes     |
| `analyses`    | financial input data, AI-generated advice            | ⚠️ Yes     |
| `piggy_banks` | savings streak, deposit history, reward points       | Low        |

### What Is Never Stored
- Raw passwords
- Private keys or seed phrases
- Google access/refresh tokens
- Full credit card or bank account numbers

---

## Known Security Considerations

### Cross-Domain Cookies (SameSite)
The `auth_token` cookie uses `SameSite=None; Secure` to support cross-domain requests between
`vercel.app` (frontend) and `onrender.com` (backend). This is required for the Google OAuth
redirect flow. API clients use Bearer tokens instead, which are not subject to SameSite
restrictions.

### Free-Tier Hosting
The backend is hosted on Render's free tier. Cold starts mean the server may be unresponsive
for up to 30 seconds after inactivity. This does not affect security but is noted for availability.

### Testnet Only
All blockchain activity occurs on **Stellar Testnet**. No real funds are at risk. Before any
Mainnet deployment, a full security audit of the Soroban contract and transaction flow is required.

### Input Validation
All auth inputs are validated server-side using dedicated validators:
- `validate_username` — length and character checks
- `validate_email` — format validation
- `validate_password` — minimum strength requirements
- `validate_wallet` — Stellar public key format check

---

## Dependency Security

### Backend (Rust)
Rust's ownership model prevents entire classes of memory vulnerabilities (buffer overflows,
use-after-free, data races). Dependencies are pinned in `Cargo.lock`.

Run audit with:
```bash
cargo install cargo-audit
cargo audit
```

### Frontend (Node.js)
```bash
npm audit
npm audit fix
```

Dependencies are pinned in `package-lock.json`.

---

## CI/CD Security

- Deployments are triggered only on push to `main` via GitHub Actions
- Environment variables (`JWT_SECRET`, `MONGODB_URI`, `GOOGLE_CLIENT_SECRET`, etc.)
  are stored in Vercel and Railway/Render secret stores — never in the repository
- `.env` files are listed in `.gitignore` and must never be committed

### Required Environment Variables (never commit these)

**Backend:**
```
MONGODB_URI
JWT_SECRET
GOOGLE_CLIENT_ID
GOOGLE_CLIENT_SECRET
STELLAR_NETWORK
PORT
```

**Frontend:**
```
VITE_API_URL
```

---

## Security Checklist for Contributors

Before opening a pull request, verify:

- [ ] No secrets, API keys, or `.env` files are committed
- [ ] All new API endpoints require `AuthUser` extractor if they access user data
- [ ] Input validation is applied to all new request DTOs
- [ ] No `unwrap()` on user-controlled data (use proper error handling)
- [ ] No new `allowed_origin("*")` in CORS config
- [ ] Smart contract changes include updated unit tests in `test.rs`

---

## License

This security policy applies to the FinWise AI codebase licensed under the MIT License.
See [LICENSE](./LICENSE) for details.