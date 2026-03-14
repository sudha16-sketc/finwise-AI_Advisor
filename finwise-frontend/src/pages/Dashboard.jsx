import { useState, useEffect, useCallback } from "react";
import {
  TrendingUp,
  Flame,
  Trophy,
  Star,
  Loader2,
  RefreshCw,
  Wallet,
} from "lucide-react";
import { finwiseApi } from "../services/api";
import RiskBadge from "../components/RiskBadge";
import BudgetChart from "../components/BudgetChart";
import SavingsChart from "../components/SavingsChart";
import Txhistory from "../components/TxHistory";
import { getStats } from "../services/stellarPiggy";
import { getConnectedAddress, onKitEvent, KitEventType } from "../services/walletManager";

function StatCard({ Icon, label, value, sub, color }) {
  return (
    <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-5 flex items-center gap-4">
      <div className={`w-12 h-12 rounded-xl ${color} flex items-center justify-center shrink-0`}>
        <Icon className="w-6 h-6 text-white" />
      </div>
      <div>
        <p className="text-xs text-slate-500 font-medium uppercase tracking-wide">{label}</p>
        <p className="text-2xl font-extrabold text-slate-800">{value}</p>
        {sub && <p className="text-xs text-slate-400">{sub}</p>}
      </div>
    </div>
  );
}

const DECIMALS = 7;
function fromContractAmount(value) {
  return Number(value) / 10 ** DECIMALS;
}

export default function Dashboard() {
  const [profile, setProfile]           = useState(null);
  const [loading, setLoading]           = useState(true);
  const [error, setError]               = useState(null);

  // Track wallet address locally — Dashboard is self-sufficient on mobile
  const [walletAddress, setWalletAddress] = useState(null);
  const [walletChecked, setWalletChecked] = useState(false); // true once we know wallet state

  // ── Step 1: resolve wallet address first ──────────────────────────────────
  //
  // On mobile, WalletConnect session restore is async over the network.
  // We must NOT fetch chain data until we know the wallet address (or confirm
  // there is none). This effect runs once and sets walletChecked = true when done.
  //
  useEffect(() => {
    let cancelled = false;

    const resolveWallet = async () => {
      const address = await getConnectedAddress();
      if (!cancelled) {
        setWalletAddress(address);
        setWalletChecked(true);
      }
    };

    resolveWallet();

    // Also subscribe to kit events so Dashboard reacts if the user connects
    // or disconnects WHILE the dashboard is open (e.g. they connect from
    // another tab, or the WalletConnect session expires on mobile).
    const unsub = onKitEvent((event) => {
      if (event.eventType === KitEventType.STATE_UPDATED) {
        const addr = event.payload?.address ?? null;
        setWalletAddress(addr);
        setWalletChecked(true);
      }
      if (event.eventType === KitEventType.DISCONNECT) {
        setWalletAddress(null);
      }
    });

    return () => {
      cancelled = true;
      unsub();
    };
  }, []);

  // ── Step 2: fetch profile only after wallet is resolved ───────────────────
  //
  // useCallback so we can call it from the Retry button too.
  //
  const fetchProfile = useCallback(async (address) => {
    setLoading(true);
    setError(null);

    try {
      const apiData = await finwiseApi.getProfile();

      // Only fetch chain data if we have a wallet address.
      // On mobile this is the key guard — without it, getStats() would be
      // called with no address and fail silently or throw.
      let chainData = { total_saved: 0, current_streak: 0, longest_streak: 0, reward_points: 0 };
      if (address) {
        chainData = (await getStats()) ?? chainData;
      }

      setProfile({
        ...apiData,
        total_saved:     chainData.total_saved,
        current_streak:  chainData.current_streak,
        longest_streak:  chainData.longest_streak,
        reward_points:   chainData.reward_points,
      });
    } catch (err) {
      setError(err.message || "Failed to load dashboard");
    } finally {
      setLoading(false);
    }
  }, []);

  // Trigger fetchProfile once wallet state is known, and again if address changes
  useEffect(() => {
    if (!walletChecked) return; // wait — session restore not done yet
    fetchProfile(walletAddress);
  }, [walletChecked, walletAddress, fetchProfile]);

  // ── Render: wallet not connected ──────────────────────────────────────────
  if (walletChecked && !walletAddress) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-slate-50 to-sky-50 px-4">
        <div className="text-center bg-white rounded-3xl p-10 shadow-sm border border-slate-100 max-w-md">
          <Wallet className="w-12 h-12 text-sky-400 mx-auto mb-4" />
          <p className="text-slate-700 font-semibold mb-2">Wallet not connected</p>
          <p className="text-slate-500 text-sm">
            Please connect your wallet from the home page to view your dashboard stats.
          </p>
        </div>
      </div>
    );
  }

  // ── Render: waiting for wallet session restore (mobile WC is slow) ─────────
  if (!walletChecked || loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-slate-50 to-sky-50">
        <div className="text-center">
          <Loader2 className="w-12 h-12 text-sky-500 animate-spin mx-auto mb-4" />
          <p className="text-slate-500">
            {!walletChecked ? "Restoring wallet session…" : "Loading your dashboard…"}
          </p>
        </div>
      </div>
    );
  }

  // ── Render: error ─────────────────────────────────────────────────────────
  if (error || !profile) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-slate-50 to-sky-50 px-4">
        <div className="text-center bg-white rounded-3xl p-10 shadow-sm border border-slate-100 max-w-md">
          <p className="text-rose-500 font-semibold mb-2">Could not load dashboard</p>
          <p className="text-slate-500 text-sm mb-6">
            {error || "Run a financial analysis first to populate your dashboard."}
          </p>
          <button
            onClick={() => fetchProfile(walletAddress)}
            className="flex items-center gap-2 mx-auto px-6 py-3 bg-sky-500 text-white rounded-xl hover:bg-sky-600 transition-colors"
          >
            <RefreshCw className="w-4 h-4" /> Retry
          </button>
        </div>
      </div>
    );
  }

  // ── Render: dashboard ─────────────────────────────────────────────────────
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-sky-50 py-10 px-4">
      <div className="max-w-5xl mx-auto">

        {/* Header */}
        <div className="flex items-center justify-between mb-8">
          <div>
            <h1 className="text-3xl font-extrabold text-slate-800">Dashboard</h1>
            <p className="text-slate-500 text-sm mt-1">Welcome back, {profile.user_id}</p>
          </div>
          <button
            onClick={() => fetchProfile(walletAddress)}
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
            <h3 className="text-lg font-bold text-slate-800 mb-4">Savings Growth</h3>
            <SavingsChart
              totalSaved={fromContractAmount(profile.total_saved)}
              streak={Number(profile.current_streak)}
            />
          </div>
          {profile.latest_advice && (
            <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-bold text-slate-800">Budget Allocation</h3>
                <RiskBadge level={profile.latest_advice.risk_level} size="sm" />
              </div>
              <BudgetChart budgetPlan={profile.latest_advice.budget_plan} />
            </div>
          )}
        </div>

        {/* Latest Advice */}
        {profile.latest_advice && (
          <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6 animate-slide-up">
            <h3 className="text-lg font-bold text-slate-800 mb-4">Latest AI Recommendations</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <p className="text-xs font-semibold text-slate-500 uppercase tracking-wide mb-2">Tax Savings</p>
                <ul className="space-y-1.5">
                  {profile.latest_advice.tax_saving_suggestions.slice(0, 3).map((s, i) => (
                    <li key={i} className="text-sm text-slate-600 flex gap-2">
                      <span className="text-amber-500 font-bold">•</span> {s}
                    </li>
                  ))}
                </ul>
              </div>
              <div>
                <p className="text-xs font-semibold text-slate-500 uppercase tracking-wide mb-2">Income Growth</p>
                <ul className="space-y-1.5">
                  {profile.latest_advice.income_growth_suggestions.slice(0, 3).map((s, i) => (
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