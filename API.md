# FinWise API Documentation

## Endpoints

### Authentication
`POST /api/signup`
```
{
  "username": "string",
  "email": "string",
  "password": "string",
  "walletAddress": "optional Stellar pubkey"
}
```

`POST /api/login`
```
{
  "email": "string",
  "password": "string"
}
```

### Metrics (NEW)
`GET /api/metrics`
```
{
  "totalUsers": 123,
  "activeUsers24h": 45,
  "activeUsers7d": 89,
  "totalTransactions": 567,
  "totalDeposits": 234,
  "totalWithdrawals": 333
}
```

`POST /api/track-user`
```
{
  "wallet_address": "G..."
}
```

### Stellar Operations
`GET /api/balance/{address}`

`GET /api/transactions/{address}`

`POST /api/send`

### Piggy Bank
`POST /api/piggy/deposit`

`GET /api/piggy/stats/{user_id}`

### AI Analysis
`POST /api/analyze`

`GET /api/profile`

## Error Codes
- 400: Invalid input
- 401: Unauthorized
- 500: Server error

## Environment
```
MONGODB_URI=...
SESSION_SECRET=...
CONTRACT_ADDRESS=... (Soroban contract)
HORIZON_URL=... (Stellar RPC)
```

