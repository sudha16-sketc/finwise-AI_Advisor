import { useState } from 'react'
import { Loader2, Sparkles, CheckCircle, TrendingUp, DollarSign, AlertCircle } from 'lucide-react'
import { finwiseApi } from '../services/api'
import RiskBadge from '../components/RiskBadge'
import BudgetChart from '../components/BudgetChart'

const USER_ID = 'user_demo_001'

const EXAMPLES = [
  'I earn 8 lakh per year in India. I have 2 kids, one home loan, and I struggle with savings. I want to reduce tax and build wealth.',
  "I'm 32 years old in the USA earning $85,000 annually. I have student loan debt and credit card debt. I want to retire early and build an emergency fund.",
  "I'm a freelancer in the UK earning £45,000 per year. I have no loans but spend too much on wants. I want to invest and buy a house.",
]

export default function Analyze() {
  const [text, setText]       = useState('')
  const [userId, setUserId]   = useState(USER_ID)
  const [loading, setLoading] = useState(false)
  const [result, setResult]   = useState(null)
  const [error, setError]     = useState(null)

  const handleSubmit = async () => {
    if (text.trim().length < 10) {
      setError('Please enter at least 10 characters describing your financial situation.')
      return
    }
    setError(null)
    setLoading(true)
    setResult(null)
    try {
      const response = await finwiseApi.analyze({ user_id: userId, financial_text: text })
      setResult(response)
    } catch (err) {
      setError(err.message || 'Analysis failed. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  const currencySymbol = result?.structured_data?.country === 'India' ? '₹' : '$'

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-sky-50 py-10 px-4">
      <div className="max-w-4xl mx-auto">

        {/* Header */}
        <div className="text-center mb-10 animate-fade-in">
          <h1 className="text-4xl font-extrabold text-slate-800 mb-3">AI Financial Analysis</h1>
          <p className="text-slate-500 text-lg">
            Describe your financial situation in plain English — our AI does the rest.
          </p>
        </div>

        {/* Input Card */}
        <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-8 mb-6 animate-slide-up">
          <div className="mb-4">
            <label className="block text-sm font-semibold text-slate-700 mb-2">
              User ID (for tracking)
            </label>
            <input
              type="text"
              value={userId}
              onChange={(e) => setUserId(e.target.value)}
              className="w-full px-4 py-2.5 rounded-xl border border-slate-200 text-sm focus:outline-none focus:ring-2 focus:ring-sky-300 text-slate-700"
              placeholder="your_user_id"
            />
          </div>

          <label className="block text-sm font-semibold text-slate-700 mb-2">
            Describe your financial situation
          </label>
          <textarea
            value={text}
            onChange={(e) => setText(e.target.value)}
            rows={6}
            placeholder="e.g. I earn 8 lakh per year in India. I have 2 kids, one home loan, and I struggle with savings..."
            className="w-full px-4 py-3 rounded-2xl border border-slate-200 text-slate-700 resize-none focus:outline-none focus:ring-2 focus:ring-sky-300 text-sm leading-relaxed"
          />
          <p className="text-xs text-slate-400 mt-1 text-right">{text.length} characters</p>

          {/* Example prompts */}
          <div className="mt-4">
            <p className="text-xs font-medium text-slate-500 mb-2">Try an example:</p>
            <div className="flex flex-col gap-2">
              {EXAMPLES.map((ex, i) => (
                <button
                  key={i}
                  onClick={() => setText(ex)}
                  className="text-left text-xs text-sky-600 hover:text-sky-800 bg-sky-50 hover:bg-sky-100 px-3 py-2 rounded-lg transition-colors truncate"
                >
                  {ex}
                </button>
              ))}
            </div>
          </div>

          {error && (
            <div className="mt-4 flex items-center gap-2 text-rose-600 bg-rose-50 px-4 py-3 rounded-xl text-sm">
              <AlertCircle className="w-4 h-4 shrink-0" />
              {error}
            </div>
          )}

          <button
            onClick={handleSubmit}
            disabled={loading}
            className="mt-6 w-full py-4 bg-gradient-to-r from-sky-500 to-violet-600 text-white font-bold rounded-2xl shadow-lg hover:shadow-sky-300 hover:scale-[1.02] active:scale-[0.98] transition-all duration-200 disabled:opacity-60 disabled:cursor-not-allowed flex items-center justify-center gap-2 text-base"
          >
            {loading ? (
              <><Loader2 className="w-5 h-5 animate-spin" /> Analyzing with AI...</>
            ) : (
              <><Sparkles className="w-5 h-5" /> Analyze My Finances</>
            )}
          </button>
        </div>

        {/* Results */}
        {result && (
          <div className="space-y-6 animate-slide-up">
            <div className="flex items-center gap-3 bg-emerald-50 border border-emerald-200 rounded-2xl px-5 py-4">
              <CheckCircle className="w-5 h-5 text-emerald-600 shrink-0" />
              <p className="text-emerald-700 text-sm font-medium">
                Analysis complete! Structured data extracted and AI advice generated.
              </p>
            </div>

            {/* Risk + Budget */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
                <h3 className="text-lg font-bold text-slate-800 mb-4">Risk Assessment</h3>
                <RiskBadge level={result.advice.risk_level} size="lg" />
                <div className="mt-6 space-y-3">
                  <div className="flex justify-between text-sm">
                    <span className="text-slate-500">Income</span>
                    <span className="font-semibold text-slate-800">
                      {currencySymbol}{result.structured_data.income.toLocaleString()}
                    </span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-slate-500">Country</span>
                    <span className="font-semibold text-slate-800">{result.structured_data.country}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-slate-500">Family Members</span>
                    <span className="font-semibold text-slate-800">{result.structured_data.family_members}</span>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
                <h3 className="text-lg font-bold text-slate-800 mb-2">Budget Allocation</h3>
                <BudgetChart budgetPlan={result.advice.budget_plan} />
              </div>
            </div>

            {/* Tax Suggestions */}
            <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
              <div className="flex items-center gap-2 mb-4">
                <DollarSign className="w-5 h-5 text-amber-500" />
                <h3 className="text-lg font-bold text-slate-800">Tax Saving Suggestions</h3>
              </div>
              <ul className="space-y-2">
                {result.advice.tax_saving_suggestions.map((s, i) => (
                  <li key={i} className="flex gap-3 text-sm text-slate-700">
                    <span className="w-6 h-6 rounded-full bg-amber-100 text-amber-700 text-xs flex items-center justify-center font-bold shrink-0 mt-0.5">
                      {i + 1}
                    </span>
                    {s}
                  </li>
                ))}
              </ul>
            </div>

            {/* Income Growth */}
            <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
              <div className="flex items-center gap-2 mb-4">
                <TrendingUp className="w-5 h-5 text-sky-500" />
                <h3 className="text-lg font-bold text-slate-800">Income Growth Suggestions</h3>
              </div>
              <ul className="space-y-2">
                {result.advice.income_growth_suggestions.map((s, i) => (
                  <li key={i} className="flex gap-3 text-sm text-slate-700">
                    <span className="w-6 h-6 rounded-full bg-sky-100 text-sky-700 text-xs flex items-center justify-center font-bold shrink-0 mt-0.5">
                      {i + 1}
                    </span>
                    {s}
                  </li>
                ))}
              </ul>
            </div>

            {/* Debt Strategy */}
            {result.advice.debt_strategy && (
              <div className="bg-white rounded-3xl shadow-sm border border-slate-100 p-6">
                <h3 className="text-lg font-bold text-slate-800 mb-3">Debt Strategy</h3>
                <p className="text-slate-600 text-sm leading-relaxed">{result.advice.debt_strategy}</p>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  )
}