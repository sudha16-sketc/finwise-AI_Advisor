# Soroban Contracts

## FinWise Piggy Contract

**Events** (indexed by Horizon):
```
("deposit", user_address) → (amount, streak, points, total_saved)
("withdraw", user_address) → (amount, total_saved)
```

**Backend Integration**:
- Poll `/soroban/events?contract=CA...`
- Parse topics[0]=type, topics[1]=user, data=amount
- Call log_transaction(type, user, amount)

**Deployment**:
```
soroban contract deploy --wasm target/soroban/finwise_contract.wasm --network testnet
```

