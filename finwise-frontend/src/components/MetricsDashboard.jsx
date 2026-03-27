// src/components/MetricsDashboard.jsx
import React, { useState, useEffect } from "react";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import "./MetricsDashboard.css";
import { finwiseApi } from "../services/api.js";
const MetricsDashboard = () => {
  const [metrics, setMetrics] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchMetrics = async () => {
      try {
        const data = await finwiseApi.getMetrics();
        setMetrics(data);
        setError(null);
      } catch (err) {
        setError(err.message || "Failed to fetch metrics");
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
        {" "}
        Loading metrics…{" "}
      </div>
    );
  } 
  if (error || !metrics) return null;
  const totalUsers = metrics.total_users ?? 0;
  const activeUsers24h = metrics.active_users_24h ?? 0;
  const activeUsers7d = metrics.active_users_7d ?? 0;
  const totalTransactions = metrics.total_transactions ?? 0;
  const totalAnalyses = metrics.total_analyses ?? 0;
  const totalConnects = metrics.total_connects ?? 0;
  const avgActions = metrics.avg_actions_per_user ?? 0; 
  const chartData = [
    { name: "Analyses", value: totalAnalyses },
    { name: "Connects", value: totalConnects },
    { name: "Transactions", value: totalTransactions },
  ];
  const maxActions = Math.max(
  ...(metrics.top_users?.map((u) => u.total_actions ?? u.actions ?? 0) || [0])
);
  return (
    <div className="metrics-page">
      <div className="metrics-dashboard">
        <div className="metrics-grid">
          {/* ── div1: Bar Chart (col 1–3, row 1–3) ── */}
          <div className="metrics-cell metrics-chart-cell metrics-div1">
            <p className="metrics-chart-title">📈 Activity Breakdown</p>
            <div className="metrics-chart-area">
              <ResponsiveContainer width="100%" height="100%">
                <BarChart
                  data={chartData}
                  margin={{ top: 4, right: 8, left: -16, bottom: 0 }}
                >
                  <CartesianGrid
                    strokeDasharray="3 3"
                    strokeOpacity={0.12}
                    stroke="#94a3b8"
                  />
                  <XAxis
                    dataKey="name"
                    tick={{ fontSize: 11, fill: "#64748b" }}
                    axisLine={false}
                    tickLine={false}
                  />
                  <YAxis
                    tick={{ fontSize: 11, fill: "#64748b" }}
                    axisLine={false}
                    tickLine={false}
                  />
                  <Tooltip
                    contentStyle={{
                      borderRadius: "10px",
                      border: "none",
                      background: "#1e293b",
                      color: "#f1f5f9",
                      boxShadow: "0 4px 20px rgba(0,0,0,0.4)",
                      fontSize: "12px",
                    }}
                    cursor={{ fill: "rgba(255,255,255,0.04)" }}
                  />
                  <Bar
                    dataKey="value"
                    name="Count"
                    fill="#38bdf8"
                    radius={[6, 6, 0, 0]}
                  />
                </BarChart>
              </ResponsiveContainer>
            </div>
          </div>

          {/* ── div2: Total Users (col 1, rows 4–5) ── */}
          <div className="metrics-cell metrics-stat-cell metrics-stat-cell--users metrics-div2">
            <p className="metrics-stat-label">Total Users</p>
            <p className="metrics-stat-value">{totalUsers.toLocaleString()}</p>
            <span className="metrics-stat-icon">👥</span>
          </div>

          {/* ── div3: Active 24h (col 2, rows 4–5) ── */}
          <div className="metrics-cell metrics-stat-cell metrics-stat-cell--24h metrics-div3">
            <p className="metrics-stat-label">Active 24h</p>
            <p className="metrics-stat-value">
              {activeUsers24h.toLocaleString()}
            </p>
            <span className="metrics-stat-icon">⚡</span>
          </div>

          {/* ── div4: Active 7d (col 3, rows 4–5) ── */}
          <div className="metrics-cell metrics-stat-cell metrics-stat-cell--7d metrics-div4">
            <p className="metrics-stat-label">Active 7d</p>
            <p className="metrics-stat-value">
              {activeUsers7d.toLocaleString()}
            </p>
            <span className="metrics-stat-icon">📅</span>
          </div>

          {/* ── div8: Total Transactions (col 4, rows 1–2) ── */}
          <div className="metrics-cell metrics-stat-cell metrics-stat-cell--tx metrics-div8">
            <p className="metrics-stat-label">Total Tx</p>
            <p className="metrics-stat-value">
              {totalTransactions.toLocaleString()}
            </p>
            <span className="metrics-stat-icon">💳</span>
          </div>

          {/* ── div9: Analyses Run (col 5, rows 1–2) ── */}
          <div className="metrics-cell metrics-stat-cell metrics-stat-cell--analyses metrics-div9">
            <p className="metrics-stat-label">Analyses</p>
            <p className="metrics-stat-value">
              {totalAnalyses.toLocaleString()}
            </p>
            <span className="metrics-stat-icon">🔬</span>
          </div>

          {/* ── div10: Top Users list (col 4–5, rows 3–5) ── */}
          <div className="metrics-cell metrics-users-cell metrics-div10">
            <p className="metrics-users-title">🏆 Most Active Users</p>
            <div className="metrics-users-list">
              {Array.isArray(metrics.top_users) &&
                metrics.top_users.slice(0, 5).map((u, i) => {
                  const actions = u.total_actions ?? u.actions ?? 0;
                  const pct = maxActions > 0 ? (actions / maxActions) * 100 : 0;
                  return (
                    <div key={i} className="metrics-user-row">
                      <span className="metrics-user-rank">#{i + 1}</span>
                      <span className="metrics-user-name">
                        {u.username ?? u.email ?? `User ${i + 1}`}
                      </span>
                      <div className="metrics-user-bar-track">
                        <div
                          className={`metrics-user-bar-fill metrics-user-bar-fill--${i}`}
                          style={{ width: `${pct}%` }}
                        />
                      </div>
                      <span className="metrics-user-actions">{actions}</span>
                    </div>
                  );
                })}
            </div>
          </div>
        </div>
        {/* end .metrics-grid */}

        {/* Footer */}
        <div className="metrics-footer">
          🔄 Auto-refreshing every 30 seconds · Data powered by wallet connects
          + AI analyses
        </div>
      </div>
    </div>
  );
};

export default MetricsDashboard;
