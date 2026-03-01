import { useState, useEffect } from 'react'
import { PiggyBank, Flame, Trophy, Star, Loader2, CheckCircle, AlertCircle } from 'lucide-react'
import { finwiseApi } from '../services/api'

const USER_ID = 'user_demo_001'

function streakEmoji(streak) {
  if (streak >= 30) return '🏆'
  if (streak >= 14) return '🔥'
  if (streak >= 7)  return '⚡'
  return '🐷'
}

export default function PiggyBankPage() {
  const [stats, setStats]                   = useState(null)
  const [amount, setAmount]                 = useState('')
  const [loading, setLoading]               = useState(false)
  const [fetchLoading, setFetchLoading]     = useState(true)
  const [success, setSuccess]               = useState(null)
  const [error, setError]                   = useState(null)
  const [alreadyDeposited, setAlreadyDeposited] = useState(false)

  const fetchStats = async () => {
    setFetchLoading(true)
    try {
      const data = await finwiseApi.getPiggyStats(USER_ID)
      setStats(data)
    } catch {
      // No stats yet — first time user
    } finally {
      setFetchLoading(false)
    }
  }

  useEffect(() => { fetchStats() }, [])

  const handleDeposit = async () => {
    const amt = parseFloat(amount)
    if (!amount || isNaN(amt) || amt <= 0) {
      setError('Please enter a valid deposit amount greater than 0.')
      return
    }
    setError(null)
    setSuccess(null)
    setLoading(true)
    try {
      const res = await finwiseApi.deposit({ user_id: USER_ID, amount: amt })
      setSuccess(res.message)
      setStats({
        user_id:        USER_ID,
        total_saved:    res.total_saved,
        current_streak: res.current_streak,
        longest_streak: res.longest_streak,
        reward_points:  res.reward_points,
      })
      setAmount('')
      setAlreadyDeposited(true)
    } catch (err) {
      const msg = err.message || 'Deposit failed'
      if (msg.toLowerCase().includes('already')) {
        setAlreadyDeposited(true)
        setError('You have already deposited today! Come back tomorrow to keep your streak.')
      } else {
        setError(msg)
      }
    } finally {
      setLoading(false)
    }
  }

  const milestones = [
    { days: 1,  label: 'First deposit',  pts: '+10 pts',       icon: '🐷' },
    { days: 7,  label: '7-day streak',   pts: '+50 bonus pts', icon: '⚡' },
    { days: 30, label: '30-day streak',  pts: '+200 bonus pts',icon: '🏆' },
  ]

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 via-pink-50 to-violet-50 py-10 px-4">
      <div className="max-w-2xl mx-auto">

        {/* Header */}
        <div className="text-center mb-10 animate-fade-in">
          <div className="text-6xl mb-4">🐷</div>
          <h1 className="text-4xl font-extrabold text-slate-800 mb-2">Piggy Bank</h1>
          <p className="text-slate-500">Deposit daily, build your streak, earn reward points!</p>
        </div>

        {/* Stats */}
        {fetchLoading ? (
          <div className="flex justify-center py-8">
            <Loader2 className="w-8 h-8 text-sky-500 animate-spin" />
          </div>
        ) : stats ? (
          <div className="grid grid-cols-2 gap-4 mb-8 animate-slide-up">
            <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-5 text-center">
              <PiggyBank className="w-6 h-6 text-pink-500 mx-auto mb-1" />
              <p className="text-xs text-slate-500 uppercase tracking-wide font-medium">Total Saved</p>
              <p className="text-2xl font-extrabold text-slate-800">₹{stats.total_saved.toLocaleString()}</p>
            </div>
            <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-5 text-center">
              <Flame className="w-6 h-6 text-amber-500 mx-auto mb-1" />
              <p className="text-xs text-slate-500 uppercase tracking-wide font-medium">Current Streak</p>
              <p className="text-2xl font-extrabold text-slate-800">
                {streakEmoji(stats.current_streak)} {stats.current_streak} days
              </p>
            </div>
            <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-5 text-center">
              <Trophy className="w-6 h-6 text-violet-500 mx-auto mb-1" />
              <p className="text-xs text-slate-500 uppercase tracking-wide font-medium">Longest Streak</p>
              <p className="text-2xl font-extrabold text-slate-800">{stats.longest_streak} days</p>
            </div>
            <div className="bg-white rounded-2xl shadow-sm border border-slate-100 p-5 text-center">
              <Star className="w-6 h-6 text-emerald-500 mx-auto mb-1" />
              <p className="text-xs text-slate-500 uppercase tracking-wide font-medium">Reward Points</p>
              <p className="text-2xl font-extrabold text-slate-800">{stats.reward_points} pts</p>
            </div>
          </div>
        ) : (
          <div className="text-center py-6 text-slate-400 text-sm mb-8">
            No deposits yet — make your first deposit below!
          </div>
        )}

        {/* Deposit Card */}
        <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-8 animate-slide-up">
          <h2 className="text-xl font-bold text-slate-800 mb-6">Make Today's Deposit</h2>

          {/* Progress nudge */}
          {stats && (
            <div className="bg-amber-50 border border-amber-100 rounded-xl px-4 py-3 mb-6 text-sm text-amber-700">
              {stats.current_streak < 7
                ? `🎯 ${7 - stats.current_streak} more days to 7-day streak (+50 bonus points)!`
                : stats.current_streak < 30
                ? `🔥 ${30 - stats.current_streak} more days to 30-day streak (+200 bonus points)!`
                : '🏆 You are a savings champion! Keep going!'}
            </div>
          )}

          {alreadyDeposited ? (
            <div className="flex items-center gap-3 bg-emerald-50 border border-emerald-200 rounded-2xl px-5 py-4">
              <CheckCircle className="w-5 h-5 text-emerald-600 shrink-0" />
              <p className="text-emerald-700 text-sm font-medium">
                {success || 'Deposit already made today! Come back tomorrow.'}
              </p>
            </div>
          ) : (
            <>
              <div className="mb-5">
                <label className="block text-sm font-semibold text-slate-700 mb-2">
                  Deposit Amount (₹)
                </label>
                <input
                  type="number"
                  value={amount}
                  onChange={(e) => setAmount(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleDeposit()}
                  min="1"
                  placeholder="e.g. 200"
                  className="w-full px-4 py-3 rounded-xl border border-slate-200 text-slate-800 text-lg font-semibold focus:outline-none focus:ring-2 focus:ring-pink-300"
                />
              </div>

              {/* Quick amount chips */}
              <div className="flex gap-2 flex-wrap mb-5">
                {[100, 200, 500, 1000].map((v) => (
                  <button
                    key={v}
                    onClick={() => setAmount(String(v))}
                    className="px-4 py-1.5 bg-slate-100 hover:bg-pink-100 hover:text-pink-700 text-slate-600 rounded-full text-sm font-medium transition-colors"
                  >
                    ₹{v}
                  </button>
                ))}
              </div>

              {error && (
                <div className="flex items-center gap-2 text-rose-600 bg-rose-50 px-4 py-3 rounded-xl text-sm mb-4">
                  <AlertCircle className="w-4 h-4 shrink-0" />
                  {error}
                </div>
              )}

              <button
                onClick={handleDeposit}
                disabled={loading}
                className="w-full py-4 bg-gradient-to-r from-pink-500 to-violet-600 text-white font-bold rounded-2xl shadow-lg hover:shadow-pink-300 hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 disabled:opacity-60 disabled:cursor-not-allowed flex items-center justify-center gap-2"
              >
                {loading ? (
                  <><Loader2 className="w-5 h-5 animate-spin" /> Saving...</>
                ) : (
                  <><PiggyBank className="w-5 h-5" /> Deposit & Feed the Piggy!</>
                )}
              </button>
            </>
          )}
        </div>

        {/* Milestones */}
        <div className="mt-6 bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
          <h3 className="font-bold text-slate-800 mb-4">Reward Milestones</h3>
          <div className="space-y-3">
            {milestones.map(({ days, label, pts, icon }) => (
              <div key={days} className="flex items-center gap-4">
                <span className="text-2xl w-8">{icon}</span>
                <div className="flex-1">
                  <p className="text-sm font-medium text-slate-700">{label}</p>
                  <p className="text-xs text-slate-400">Each deposit</p>
                </div>
                <span className="text-sm font-semibold text-emerald-600">{pts}</span>
                {stats && stats.current_streak >= days && (
                  <CheckCircle className="w-4 h-4 text-emerald-500" />
                )}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}