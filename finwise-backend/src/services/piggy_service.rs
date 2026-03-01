use chrono::Utc;
use mongodb::bson::doc;

use crate::db::{Database, collections};
use crate::models::piggy::PiggyBankDocument;
use crate::utils::{AppError, AppResult};

const SECONDS_IN_DAY: i64 = 86_400;
const STREAK_GRACE_PERIOD: i64 = SECONDS_IN_DAY + 3600; // 25 hours grace

pub struct PiggyService;

impl PiggyService {
    /// Processes a deposit for a user. Enforces one-per-day, updates streak.
    pub async fn deposit(
        db: &Database,
        user_id: &str,
        amount: f64,
    ) -> AppResult<PiggyBankDocument> {
        if amount <= 0.0 {
            return Err(AppError::Validation("Deposit amount must be greater than 0".to_string()));
        }

        let collection = db.collection::<PiggyBankDocument>(collections::PIGGY_BANKS);
        let now = Utc::now().timestamp();

        // Fetch existing record or create default
        let existing = collection
            .find_one(doc! { "user_id": user_id })
            .await?;

        let mut piggy = existing.unwrap_or_else(|| PiggyBankDocument {
            id: None,
            user_id: user_id.to_string(),
            total_saved: 0.0,
            current_streak: 0,
            longest_streak: 0,
            last_deposit_ts: None,
            reward_points: 0,
        });

        // Check if already deposited today
        if let Some(last_ts) = piggy.last_deposit_ts {
            let elapsed = now - last_ts;
            if elapsed < SECONDS_IN_DAY {
                return Err(AppError::AlreadyDepositedToday);
            }
            // Check if streak should continue or reset
            if elapsed > STREAK_GRACE_PERIOD {
                // Missed a day — reset streak
                piggy.current_streak = 0;
            }
        }

        // Update stats
        piggy.total_saved += amount;
        piggy.current_streak += 1;
        piggy.last_deposit_ts = Some(now);

        if piggy.current_streak > piggy.longest_streak {
            piggy.longest_streak = piggy.current_streak;
        }

        // Reward points: base + streak bonuses
        piggy.reward_points += 10;
        if piggy.current_streak == 7 {
            piggy.reward_points += 50;
            log::info!("🎉 User {} hit 7-day streak! +50 bonus points", user_id);
        }
        if piggy.current_streak == 30 {
            piggy.reward_points += 200;
            log::info!("🏆 User {} hit 30-day streak! +200 bonus points", user_id);
        }

        // Upsert into MongoDB — mongodb 3.x takes (filter, update) as positional args
        let filter = doc! { "user_id": user_id };
        let update = doc! {
            "$set": {
                "user_id": &piggy.user_id,
                "total_saved": piggy.total_saved,
                "current_streak": piggy.current_streak as i32,
                "longest_streak": piggy.longest_streak as i32,
                "last_deposit_ts": piggy.last_deposit_ts,
                "reward_points": piggy.reward_points as i32,
            }
        };

        collection
            .update_one(filter, update)
            .upsert(true)
            .await?;

        Ok(piggy)
    }

    /// Fetches piggy bank stats for a user
    pub async fn get_stats(db: &Database, user_id: &str) -> AppResult<PiggyBankDocument> {
        let collection = db.collection::<PiggyBankDocument>(collections::PIGGY_BANKS);

        collection
            .find_one(doc! { "user_id": user_id })
            .await?
            .ok_or_else(|| AppError::NotFound(format!("No piggy bank found for user {}", user_id)))
    }
}