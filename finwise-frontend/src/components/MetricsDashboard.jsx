// src/components/MetricsDashboard.jsx
import React, { useState, useEffect } from 'react';
import {
  BarChart, Bar, XAxis, YAxis, CartesianGrid,
  Tooltip, Legend, ResponsiveContainer,
} from 'recharts';
import { finwiseApi } from '../services/api.js'; // ✅ named export, not default

const MetricsDashboard = () => {
  const [metrics, setMetrics] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError]     = useState(null);

  useEffect(() => {
    const fetchMetrics = async () => {
      try {
        const data = await finwiseApi.getMetrics(); // ✅ uses existing helper
        setMetrics(data);
        setError(null);
      } catch (err) {
        setError(err.message || 'Failed to fetch metrics');
      } finally {
        setLoading(false);
      }
    };

    fetchMetrics();
    const interval = setInterval(fetchMetrics, 30000);
    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return (
      <div className="text-center py-8 text-slate-500 text-sm animate-pulse">
        Loading metrics…
      </div>
    );
  }

  // Fail silently — metrics are supplementary, don't break the dashboard
  if (error || !metrics) return null;

  // Defensive field resolution (snake_case from Rust backend)
  const totalUsers        = metrics.total_users          ?? 0;
  const activeUsers24h    = metrics.active_users_24h     ?? metrics.active_today ?? 0;
  const activeUsers7d     = metrics.active_users_7d      ?? 0;
  const totalTransactions = metrics.total_transactions   ?? 0;
  const totalDeposits     = metrics.total_deposits       ?? 0;
  const totalWithdrawals  = metrics.total_withdrawals    ?? 0;
  const totalAnalyses     = metrics.total_analyses       ?? 0;
  const avgActions        = metrics.avg_actions_per_user ?? 0;

  const chartData = [
    { name: 'Deposits',     value: totalDeposits },
    { name: 'Withdrawals',  value: totalWithdrawals },
    { name: 'Analyses',     value: totalAnalyses },
    { name: 'Transactions', value: totalTransactions },
  ];

  return (
    <div className="bg-white/80 backdrop-blur-xl rounded-2xl p-8 shadow-sm border border-slate-100 max-w-4xl mx-auto mt-6">
      <h2 className="text-2xl font-extrabold text-slate-800 mb-8 text-center">
        📊 Platform Metrics
      </h2>

      {/* Row 1 — 4 cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
        <div className="bg-gradient-to-br from-sky-400 to-cyan-500 text-white p-5 rounded-2xl">
          <h3 className="text-xs font-semibold opacity-80 uppercase tracking-widest">Total Users</h3>
          <div className="text-3xl font-black mt-2">{totalUsers.toLocaleString()}</div>
        </div>
        <div className="bg-gradient-to-br from-emerald-400 to-teal-500 text-white p-5 rounded-2xl">
          <h3 className="text-xs font-semibold opacity-80 uppercase tracking-widest">Active 24h</h3>
          <div className="text-3xl font-black mt-2">{activeUsers24h.toLocaleString()}</div>
        </div>
        <div className="bg-gradient-to-br from-amber-400 to-orange-500 text-white p-5 rounded-2xl">
          <h3 className="text-xs font-semibold opacity-80 uppercase tracking-widest">Active 7d</h3>
          <div className="text-3xl font-black mt-2">{activeUsers7d.toLocaleString()}</div>
        </div>
        <div className="bg-gradient-to-br from-violet-400 to-purple-500 text-white p-5 rounded-2xl">
          <h3 className="text-xs font-semibold opacity-80 uppercase tracking-widest">Total Tx</h3>
          <div className="text-3xl font-black mt-2">{totalTransactions.toLocaleString()}</div>
        </div>
      </div>

      {/* Row 2 — 2 cards */}
      <div className="grid grid-cols-2 gap-4 mb-8">
        <div className="bg-gradient-to-br from-rose-400 to-pink-500 text-white p-5 rounded-2xl">
          <h3 className="text-xs font-semibold opacity-80 uppercase tracking-widest">Analyses Run</h3>
          <div className="text-3xl font-black mt-2">{totalAnalyses.toLocaleString()}</div>
        </div>
        <div className="bg-gradient-to-br from-indigo-400 to-blue-500 text-white p-5 rounded-2xl">
          <h3 className="text-xs font-semibold opacity-80 uppercase tracking-widest">Avg Actions / User</h3>
          <div className="text-3xl font-black mt-2">
            {typeof avgActions === 'number' ? avgActions.toFixed(1) : avgActions}
          </div>
        </div>
      </div>

      {/* Bar chart */}
      <div className="bg-slate-50 p-6 rounded-2xl">
        <h3 className="text-base font-bold text-slate-700 mb-4">💰 Activity Breakdown</h3>
        <ResponsiveContainer width="100%" height={280}>
          <BarChart data={chartData} margin={{ top: 4, right: 8, left: 0, bottom: 0 }}>
            <CartesianGrid strokeDasharray="3 3" strokeOpacity={0.25} />
            <XAxis dataKey="name" tick={{ fontSize: 12, fill: '#64748b' }} />
            <YAxis tick={{ fontSize: 12, fill: '#64748b' }} />
            <Tooltip
              contentStyle={{
                borderRadius: '12px',
                border: 'none',
                boxShadow: '0 4px 20px rgba(0,0,0,0.1)',
                fontSize: '13px',
              }}
            />
            <Legend wrapperStyle={{ fontSize: '12px' }} />
            <Bar dataKey="value" name="Count" fill="#38bdf8" radius={[6, 6, 0, 0]} />
          </BarChart>
        </ResponsiveContainer>
      </div>

      {/* Top users — only rendered if backend returns top_users array */}
      {Array.isArray(metrics.top_users) && metrics.top_users.length > 0 && (
        <div className="mt-6">
          <h3 className="text-base font-bold text-slate-700 mb-3">🏆 Most Active Users</h3>
          <div className="space-y-2">
            {metrics.top_users.slice(0, 5).map((u, i) => {
              const actions = u.total_actions ?? u.actions ?? 0;
              const maxA    = Math.max(...metrics.top_users.map(x => x.total_actions ?? x.actions ?? 0));
              const pct     = maxA > 0 ? (actions / maxA) * 100 : 0;
              const colors  = ['bg-sky-400','bg-violet-400','bg-amber-400','bg-emerald-400','bg-rose-400'];
              return (
                <div key={i} className="flex items-center gap-3">
                  <span className="w-28 text-xs text-slate-500 truncate shrink-0">
                    {u.username ?? u.email ?? `User ${i + 1}`}
                  </span>
                  <div className="flex-1 h-2 bg-slate-100 rounded-full overflow-hidden">
                    <div
                      className={`h-full rounded-full ${colors[i % colors.length]} transition-all duration-700`}
                      style={{ width: `${pct}%` }}
                    />
                  </div>
                  <span className="w-8 text-right text-xs font-semibold text-slate-600">{actions}</span>
                </div>
              );
            })}
          </div>
        </div>
      )}

      <div className="mt-6 px-4 py-3 bg-sky-50 rounded-xl text-xs text-sky-700">
        🔄 Auto-refreshing every 30 seconds · Data powered by Soroban events + wallet connects
      </div>
    </div>
  );
};

export default MetricsDashboard;