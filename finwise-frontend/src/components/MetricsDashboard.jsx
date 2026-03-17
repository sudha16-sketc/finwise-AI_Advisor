import React, { useState, useEffect } from 'react';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import api from '../services/api.js';

const MetricsDashboard = () => {
  const [metrics, setMetrics] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchMetrics = async () => {
      try {
        const response = await api.get('/metrics');
        setMetrics(response.data);
      } catch (err) {
        setError('Failed to fetch metrics');
      } finally {
        setLoading(false);
      }
    };
    fetchMetrics();
    const interval = setInterval(fetchMetrics, 30000); // Refresh every 30s
    return () => clearInterval(interval);
  }, []);

  if (loading) return <div className="text-center py-8">Loading metrics...</div>;
  if (error) return <div className="text-red-500 text-center py-8">{error}</div>;

  const chartData = [
    { name: 'Deposits', value: metrics.total_deposits },
    { name: 'Withdrawals', value: metrics.total_withdrawals },
  ];

  return (
    <div className="bg-white/80 backdrop-blur-xl rounded-2xl p-8 shadow-2xl border border-white/20 max-w-4xl mx-auto">
      <h2 className="text-3xl font-bold text-gray-900 mb-8 text-center">📊 Platform Metrics</h2>
      
      <div className="grid md:grid-cols-2 gap-8 mb-8">
        <div className="bg-gradient-to-br from-blue-500 to-purple-600 text-white p-6 rounded-xl">
          <h3 className="text-lg font-semibold opacity-90">Total Users</h3>
          <div className="text-4xl font-black mt-2">{metrics.total_users}</div>
        </div>
        <div className="bg-gradient-to-br from-emerald-500 to-teal-600 text-white p-6 rounded-xl">
          <h3 className="text-lg font-semibold opacity-90">Active 24h</h3>
          <div className="text-4xl font-black mt-2">{metrics.active_users_24h}</div>
        </div>
      </div>

      <div className="grid md:grid-cols-2 gap-8 mb-8">
        <div className="bg-gradient-to-br from-orange-500 to-red-600 text-white p-6 rounded-xl">
          <h3 className="text-lg font-semibold opacity-90">Active 7d</h3>
          <div className="text-4xl font-black mt-2">{metrics.active_users_7d}</div>
        </div>
        <div className="bg-gradient-to-br from-indigo-500 to-violet-600 text-white p-6 rounded-xl">
          <h3 className="text-lg font-semibold opacity-90">Total Tx</h3>
          <div className="text-4xl font-black mt-2">{metrics.total_transactions}</div>
        </div>
      </div>

      <div className="bg-gradient-to-r from-gray-50 to-gray-100 p-6 rounded-2xl">
        <h3 className="text-xl font-bold text-gray-900 mb-4">💰 Deposits vs Withdrawals</h3>
        <ResponsiveContainer width="100%" height={400}>
          <BarChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" strokeOpacity={0.3} />
            <XAxis dataKey="name" />
            <YAxis />
            <Tooltip />
            <Legend />
            <Bar dataKey="value" fill="#10b981" radius={[4, 4, 0, 0]} />
          </BarChart>
        </ResponsiveContainer>
      </div>

      <div className="mt-8 p-4 bg-blue-50 rounded-xl text-sm text-blue-800">
        <p>🔄 Auto-refreshing every 30 seconds • Data powered by Soroban events + wallet connects</p>
      </div>
    </div>
  );
};

export default MetricsDashboard;

