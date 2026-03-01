/// Returns true if more than 24 hours have elapsed since the last deposit timestamp.
pub fn is_new_day(last_ts: u64, now: u64) -> bool {
    const SECONDS_IN_DAY: u64 = 86_400;
    now.saturating_sub(last_ts) >= SECONDS_IN_DAY
}

/// Returns true if the streak was broken (gap > 25 hours = 24h + 1h grace)
pub fn streak_broken(last_ts: u64, now: u64) -> bool {
    const GRACE_PERIOD: u64 = 90_000; // 25 hours
    now.saturating_sub(last_ts) > GRACE_PERIOD
}

/// Calculates reward points for a deposit based on streak length.
/// Base: +10 points per deposit
/// 7-day streak: +50 bonus
/// 30-day streak: +200 bonus
pub fn calculate_reward_points(current_streak: u32) -> u32 {
    let mut pts: u32 = 10;
    if current_streak == 7 {
        pts = pts.saturating_add(50);
    }
    if current_streak == 30 {
        pts = pts.saturating_add(200);
    }
    pts
}