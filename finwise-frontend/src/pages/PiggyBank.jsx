import { useState, useEffect } from "react";
import {
  PiggyBank,
  Flame,
  Trophy,
  Star,
  Loader2,
  CheckCircle,
  AlertCircle,
} from "lucide-react";
import { deposit, getStats } from "../services/stellarPiggy";
import "./PiggyBank.css";

const USER_ID = "user_demo_001";

function streakEmoji(streak) {
  if (streak >= 30) return "🏆";
  if (streak >= 14) return "🔥";
  if (streak >= 7) return "⚡";
  return "🐷";
}

export default function PiggyBankPage() {
  const [stats, setStats] = useState({
    total_saved: 0,
    current_streak: 0,
    last_deposit_day: 0,
  });
  const [amount, setAmount] = useState("");
  const [loading, setLoading] = useState(false);
  const [fetchLoading, setFetchLoading] = useState(true);
  const [success, setSuccess] = useState(null);
  const [error, setError] = useState(null);
  const [alreadyDeposited, setAlreadyDeposited] = useState(false);

  const fetchStats = async () => {
    setFetchLoading(true);
    try {
      const data = await getStats();
      setStats({
        total_saved: Number(data?.total_saved ?? 0),
        current_streak: Number(data?.current_streak ?? 0),
        longest_streak: Number(data?.longest_streak ?? 0),
        reward_points: Number(data?.reward_points ?? 0),
      });
    } catch (err) {
      console.error("Fetch stats error:", err);
    } finally {
      setFetchLoading(false);
    }
  };

  useEffect(() => {
    fetchStats();
  }, []);

  const handleDeposit = async () => {
    const amt = parseFloat(amount);
    if (!amount || isNaN(amt) || amt <= 0) {
      setError("Please enter a valid deposit amount greater than 0.");
      return;
    }

    setError(null);
    setSuccess(null);
    setLoading(true);

    try {
      await deposit(Number(amount));
      setSuccess("Deposit successful on blockchain!");
      await fetchStats();
      setAmount("");
      setAlreadyDeposited(true);
    } catch (err) {
      console.error("Deposit error:", err);
      const raw = err?.toString() || "";

      if (raw.includes("Error(Contract, #1)")) {
        setAlreadyDeposited(true);
        setError(
          "🐷 You have already deposited today. Please come back tomorrow to continue your streak.",
        );
      } else {
        setError("Deposit failed. Please try again.");
      }
    } finally {
      setLoading(false);
    }
  };

  const milestones = [
    { days: 1, label: "First deposit", pts: "+10 pts", icon: "🐷" },
    { days: 7, label: "7-day streak", pts: "+50 bonus pts", icon: "⚡" },
    { days: 30, label: "30-day streak", pts: "+200 bonus pts", icon: "🏆" },
  ];

  return (
    <div className="piggy-page">
      <div className="piggy-container">
        <div className="piggy-header">
          <h1 className="piggy-header-title">Piggy Bank</h1>
          <p className="piggy-header-subtitle">
            Deposit daily, build your streak, earn reward points!
          </p>
        </div>

        <div className="piggy-deposit-card">
          <h2 className="piggy-deposit-title">Make Today's Deposit</h2>

          {stats && (
            <div className="piggy-nudge">
              {stats.current_streak < 7
                ? `🎯 ${7 - stats.current_streak} more days to 7-day streak (+50 bonus points)!`
                : stats.current_streak < 30
                  ? `🔥 ${30 - stats.current_streak} more days to 30-day streak (+200 bonus points)!`
                  : "🏆 You are a savings champion! Keep going!"}
            </div>
          )}

          {alreadyDeposited ? (
            <div className="piggy-already-deposited">
              <CheckCircle className="piggy-already-deposited-icon" />
              <p className="piggy-already-deposited-text">
                {success || "Deposit already made today! Come back tomorrow."}
              </p>
            </div>
          ) : (
            <>
              <div className="piggy-input-wrapper">
                <label className="piggy-input-label">Deposit Amount (₹)</label>
                <input
                  type="number"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  onKeyDown={(e) => e.key === "Enter" && handleDeposit()}
                  min="1"
                  placeholder="e.g. 200"
                  className="piggy-input"
                />
              </div>

              <div className="piggy-chips">
                {[100, 200, 500, 1000].map((v) => (
                  <button
                    key={v}
                    onClick={() => setAmount(String(v))}
                    className="piggy-chip"
                  >
                    ₹{v}
                  </button>
                ))}
              </div>

              {error && (
                <div className="piggy-error">
                  <AlertCircle className="piggy-error-icon" />
                  {error}
                </div>
              )}

              <button
                onClick={handleDeposit}
                disabled={loading}
                className="piggy-deposit-btn"
              >
                {loading ? (
                  <>
                    <Loader2 className="piggy-spinner" /> Saving...
                  </>
                ) : (
                  <>
                    <PiggyBank className="piggy-deposit-btn-icon" /> Deposit &
                    Feed the Piggy!
                  </>
                )}
              </button>
            </>
          )}
        </div>

        {fetchLoading ? (
          <div className="piggy-stats-loader">
            <Loader2 className="piggy-stats-loader-icon" />
          </div>
        ) : stats ? (
          <>
            <div className="piggy-stat-card piggy-stat-card--saved">
              <PiggyBank className="piggy-stat-icon piggy-stat-icon--pink" />
              <p className="piggy-stat-label">Total Saved</p>
              <p className="piggy-stat-value">
                ₹{((stats?.total_saved ?? 0) / 10 ** 7).toLocaleString()}
              </p>
            </div>

            <div className="piggy-stat-card piggy-stat-card--streak">
              <Flame className="piggy-stat-icon piggy-stat-icon--amber" />
              <p className="piggy-stat-label">Current Streak</p>
              <p className="piggy-stat-value">
                {streakEmoji(stats.current_streak)} {stats.current_streak} days
              </p>
            </div>

            <div className="piggy-stat-card piggy-stat-card--longest">
              <Trophy className="piggy-stat-icon piggy-stat-icon--violet" />
              <p className="piggy-stat-label">Longest Streak</p>
              <p className="piggy-stat-value">{stats.longest_streak} days</p>
            </div>

            <div className="piggy-stat-card piggy-stat-card--points">
              <Star className="piggy-stat-icon piggy-stat-icon--emerald" />
              <p className="piggy-stat-label">Reward Points</p>
              <p className="piggy-stat-value">{stats.reward_points} pts</p>
            </div>
          </>
        ) : (
          <div className="piggy-stats-empty">
            No deposits yet — make your first deposit below!
          </div>
        )}

        <div className="piggy-milestones-card">
          <h3 className="piggy-milestones-title">Reward Milestones</h3>
          <div className="piggy-milestones-list">
            {milestones.map(({ days, label, pts, icon }) => (
              <div key={days} className="piggy-milestone-row">
                <span className="piggy-milestone-emoji">{icon}</span>
                <div className="piggy-milestone-info">
                  <p className="piggy-milestone-label">{label}</p>
                  <p className="piggy-milestone-sublabel">Each deposit</p>
                </div>
                <span className="piggy-milestone-pts">{pts}</span>
                {stats && stats.current_streak >= days && (
                  <CheckCircle className="piggy-milestone-check" />
                )}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
