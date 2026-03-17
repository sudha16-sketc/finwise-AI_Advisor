# Installation & Setup

## Backend (Rust)

1. `cd finwise-backend`
2. `cp .env.example .env` (add MONGODB_URI, CONTRACT_ADDRESS, SESSION_SECRET)
3. `cargo build`
4. `cargo run`

Mongo indexes (run in Mongo shell):
```
db.users.createIndex({"wallet_address": 1}, {unique: true})
db.users.createIndex({"last_active": -1})
db.transactions.createIndex({"wallet_address": 1, "created_at": -1})
db.transactions.createIndex({"tx_type": 1, "created_at": -1})
```

## Frontend
1. `cd finwise-frontend`
2. `npm install`
3. `npm run dev`

## Contract (Soroban)
1. `cd finwise-contract`
2. `soroban contract build`
3. `soroban contract deploy --network testnet`

## Test
- `curl http://localhost:8080/api/metrics`
- Visit `localhost:5173/dashboard` (connect wallet)

