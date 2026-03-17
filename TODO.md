# Verified Active Users + Metrics Dashboard Implementation Plan
Current Working Directory: d:/stellar

## Approved Plan Summary
- Extend MongoDB User model (add last_active, total_actions)
- New Transactions model/collection
- Services: user_service (track_user), metrics_service, event_listener
- Routes: /api/metrics, /api/track-user
- Frontend: MetricsDashboard.jsx + updates
- Docs: Update README + new files
- Use existing MongoDB/Soroban events/Actix structure

## Step-by-Step Implementation (Breakdown)

### Phase 1: Models & DB (3 steps)
- [x] 1. Edit `finwise-backend/src/models/user.rs`: Add `last_active: Option<DateTime<Utc>>`, `total_actions: u64`
- [x] 2. Create `finwise-backend/src/models/transactions.rs`: New Transaction model
- [ ] 3. Create `finwise-backend/src/db/migrations.rs` or doc: Mongo indexes commands

### Phase 2: Services (4 steps)
- [ ] 4. Create `finwise-backend/src/services/user_service.rs`: track_user upsert fn
- [ ] 5. Create `finwise-backend/src/services/metrics_service.rs`: Aggregations for /metrics
- [ ] 6. Create `finwise-backend/src/services/event_listener.rs`: Horizon polling for deposit/withdraw
- [ ] 7. Update `finwise-backend/src/services/mod.rs`: Export new services

### Phase 3: Routes & Main (3 steps)
- [ ] 8. Create `finwise-backend/src/routes/metrics.rs`: GET /api/metrics
- [ ] 9. Create `finwise-backend/src/routes/user.rs`: POST /api/track-user
- [ ] 10. Edit `finwise-backend/src/main.rs`: Add routes; spawn event_listener task

### Phase 4: Frontend (3 steps)
- [ ] 11. Create `finwise-frontend/src/components/MetricsDashboard.jsx`
- [ ] 12. Edit `finwise-frontend/src/pages/Dashboard.jsx`: Import/add MetricsDashboard
- [ ] 13. Edit `finwise-frontend/src/services/api.js`: Add metrics fetch

### Phase 5: Integration & Docs (3 steps)
- [ ] 14. Update main.rs signup/piggy: Call track_user on wallet actions
- [ ] 15. Update root docs: README.md + new INSTALL.md/API.md/ARCHITECTURE.md/CONTRACTS.md
- [ ] 16. Test: cargo check/build, npm run dev, manual metrics curl

## Progress Tracking
- Completed: 0/16
- In Progress: Planning
- Next: Phase 1

*This TODO will be updated after each phase.*

