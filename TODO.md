# FinWise Backend Compilation Fix Plan
## Approved Steps (Step-by-step execution)

### Step 1: Fix models/mod.rs (add metrics re-export)
- ✅ Add `pub use metrics::*;`

### Step 2: Fix db/mod.rs (add Clone derive + indexes)
- ✅ Add `#[derive(Clone)]`
- ✅ Add index creation in Database::new()

### Step 3: Fix main.rs 
- ✅ Complete User struct in google_callback()
- ✅ Fix event listener spawn
- ✅ Clean unused imports

### Step 4: Fix services/user_service.rs
- ✅ Use bson::DateTime 
- ✅ Fix find_one_and_update API (remove options param)

### Step 5: Fix models/metrics.rs (ensure Serialize)
- ✅ Add Serialize derive

### Step 6: Test compilation
- `cd finwise-backend && cargo check`
- `cd finwise-backend && cargo build --release`

### Step 7: Docker rebuild test
- `docker build -t finwise-backend .`

---

**Current Progress: 0/7 steps complete**
**Next Action: Execute Step 1**

