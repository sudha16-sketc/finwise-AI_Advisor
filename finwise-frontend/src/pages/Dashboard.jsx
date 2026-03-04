import { useState, useEffect } from "react";
import {
  TrendingUp,
  Flame,
  Trophy,
  Star,
  Loader2,
  RefreshCw,
} from "lucide-react";
import { finwiseApi } from "../services/api";
import RiskBadge from "../components/RiskBadge";
import BudgetChart from "../components/BudgetChart";
import SavingsChart from "../components/SavingsChart";
import Txhistory from "../components/Txhistory";
import { getStats } from "../services/stellarPiggy";

function StatCard({ Icon, label, value, sub, color }) {
  return (
    <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-5 flex items-center gap-4">
      <div
        className={`w-12 h-12 rounded-xl ${color} flex items-center justify-center shrink-0`}
      >
        <Icon className="w-6 h-6 text-white" />
      </div>
      <div>
        <p className="text-xs text-slate-500 font-medium uppercase tracking-wide">
          {label}
        </p>
        <p className="text-2xl font-extrabold text-slate-800">{value}</p>
        {sub && <p className="text-xs text-slate-400">{sub}</p>}
      </div>
    </div>
  );
}

export default function Dashboard() {
  const [profile, setProfile] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const DECIMALS = 7;

  function fromContractAmount(value) {
    return Number(value) / 10 ** DECIMALS;
  }
  const fetchProfile = async () => {
    setLoading(true);
    setError(null);

    try {
      // 1️⃣ Get backend data
      const apiData = await finwiseApi.getProfile();

      // 2️⃣ Get blockchain data
      const chainData = await getStats();

      // 3️⃣ Merge both
      setProfile({
        ...apiData,
        total_saved: chainData?.total_saved ?? 0,
        current_streak: chainData?.current_streak ?? 0,
        longest_streak: chainData?.longest_streak ?? 0,
        reward_points: chainData?.reward_points ?? 0,
      });
    } catch (err) {
      setError(err.message || "Failed to load dashboard");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchProfile();
  }, []);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-slate-50 to-sky-50">
        <div className="text-center">
          <Loader2 className="w-12 h-12 text-sky-500 animate-spin mx-auto mb-4" />
          <p className="text-slate-500">Loading your dashboard...</p>
        </div>
      </div>
    );
  }

  if (error || !profile) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-slate-50 to-sky-50 px-4">
        <div className="text-center bg-white rounded-3xl p-10 shadow-sm border border-slate-100 max-w-md">
          <p className="text-rose-500 font-semibold mb-2">
            Could not load dashboard
          </p>
          <p className="text-slate-500 text-sm mb-6">
            {error ||
              "Run a financial analysis first to populate your dashboard."}
          </p>
          <button
            onClick={fetchProfile}
            className="flex items-center gap-2 mx-auto px-6 py-3 bg-sky-500 text-white rounded-xl hover:bg-sky-600 transition-colors"
          >
            <RefreshCw className="w-4 h-4" /> Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-sky-50 py-10 px-4">
      <div className="max-w-5xl mx-auto">
        {/* Header */}
        <div className="flex items-center justify-between mb-8">
          <div>
            <h1 className="text-3xl font-extrabold text-slate-800">
              Dashboard
            </h1>
            <p className="text-slate-500 text-sm mt-1">
              Welcome back, {profile.user_id}
            </p>
          </div>
          <button
            onClick={fetchProfile}
            className="p-2.5 rounded-xl bg-white border border-slate-200 text-slate-500 hover:text-sky-600 hover:border-sky-300 transition-all"
          >
            <RefreshCw className="w-5 h-5" />
          </button>
        </div>

        {/* Stats Grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8 animate-slide-up">
          <StatCard
            Icon={TrendingUp}
            label="Total Saved"
            value={`₹${fromContractAmount(profile.total_saved).toLocaleString()}`}
            color="bg-gradient-to-br from-sky-400 to-sky-600"
          />
          <StatCard
            Icon={Flame}
            label="Current Streak"
            value={`${profile.current_streak} days`}
            color="bg-gradient-to-br from-amber-400 to-orange-500"
            sub="Keep it going!"
          />
          <StatCard
            Icon={Trophy}
            label="Longest Streak"
            value={`${profile.longest_streak} days`}
            color="bg-gradient-to-br from-violet-400 to-purple-600"
          />
          <StatCard
            Icon={Star}
            label="Reward Points"
            value={profile.reward_points}
            color="bg-gradient-to-br from-emerald-400 to-teal-600"
            sub={`${profile.total_analyses} analyses done`}
          />
        </div>

        {/* Charts */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
          <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
            <h3 className="text-lg font-bold text-slate-800 mb-4">
              Savings Growth
            </h3>
            <SavingsChart
              totalSaved={fromContractAmount(profile.total_saved)}
              streak={Number(profile.current_streak)}
            />
          </div>

          {profile.latest_advice && (
            <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-bold text-slate-800">
                  Budget Allocation
                </h3>
                <RiskBadge level={profile.latest_advice.risk_level} size="sm" />
              </div>
              <BudgetChart budgetPlan={profile.latest_advice.budget_plan} />
            </div>
          )}
        </div>

        {/* Latest Advice */}
        {profile.latest_advice && (
          <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6 animate-slide-up">
            <h3 className="text-lg font-bold text-slate-800 mb-4">
              Latest AI Recommendations
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <p className="text-xs font-semibold text-slate-500 uppercase tracking-wide mb-2">
                  Tax Savings
                </p>
                <ul className="space-y-1.5">
                  {profile.latest_advice.tax_saving_suggestions
                    .slice(0, 3)
                    .map((s, i) => (
                      <li key={i} className="text-sm text-slate-600 flex gap-2">
                        <span className="text-amber-500 font-bold">•</span> {s}
                      </li>
                    ))}
                </ul>
              </div>
              <div>
                <p className="text-xs font-semibold text-slate-500 uppercase tracking-wide mb-2">
                  Income Growth
                </p>
                <ul className="space-y-1.5">
                  {profile.latest_advice.income_growth_suggestions
                    .slice(0, 3)
                    .map((s, i) => (
                      <li key={i} className="text-sm text-slate-600 flex gap-2">
                        <span className="text-sky-500 font-bold">•</span> {s}
                      </li>
                    ))}
                </ul>
              </div>
            </div>
          </div>
        )}

        <div>
          <Txhistory />
        </div>
      </div>
    </div>
  );
}
