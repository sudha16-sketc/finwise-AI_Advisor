# Architecture

```
Wallet Connect ──→ API /track-user ──→ track_user() ──→ Mongo users
    │
    ↓
Soroban Contract (deposit/withdraw) ──→ Events ──→ Event Listener ──→ log_transaction() ──→ Mongo transactions
    │
    ↓
API /metrics ──→ Aggregations ──→ React Dashboard (Recharts)
```

**Data Flow**:
1. Wallet connects → track_user upserts user, logs "connect" tx
2. Contract emits deposit/withdraw → Poller ingests → log_transaction updates user activity
3. Dashboard polls /metrics → Mongo aggregations → Charts

**Scalability**:
- Mongo indexes for O(log n) lookups
- Event polling (scale with workers)
- Cached aggregations possible

