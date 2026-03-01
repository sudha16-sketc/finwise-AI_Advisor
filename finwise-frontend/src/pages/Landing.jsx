import { useNavigate } from 'react-router-dom'
import { TrendingUp, Shield, Zap, PiggyBank, BarChart2, Brain } from 'lucide-react'


const features = [
  {
    Icon: Brain,
    title: 'AI-Powered Analysis',
    desc: 'Describe your finances in plain English — our AI extracts structure and generates expert advice.',
    color: 'from-sky-500 to-blue-600',
  },
  {
    Icon: BarChart2,
    title: 'Smart Budget Plans',
    desc: 'Get personalized needs/wants/savings allocation tailored to your income, country, and goals.',
    color: 'from-violet-500 to-purple-600',
  },
  {
    Icon: PiggyBank,
    title: 'Gamified Savings',
    desc: 'Build daily deposit habits with streaks, rewards, and milestone badges to keep you motivated.',
    color: 'from-emerald-500 to-teal-600',
  },
  {
    Icon: Shield,
    title: 'Tax Optimization',
    desc: 'Receive country-specific tax-saving suggestions based on your financial profile.',
    color: 'from-amber-500 to-orange-600',
  },
  {
    Icon: Zap,
    title: 'Instant Results',
    desc: 'Powered by Google Gemini — get actionable advice in seconds, not days.',
    color: 'from-rose-500 to-pink-600',
  },
  {
    Icon: TrendingUp,
    title: 'Wealth Growth',
    desc: 'Track your savings streak, total saved, and visualize your path to financial freedom.',
    color: 'from-indigo-500 to-sky-600',
  },
]

export default function Landing() {
  const navigate = useNavigate()

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 via-sky-50 to-violet-50">
      {/* Hero */}
      <section className="relative overflow-hidden pt-24 pb-20 px-4">
        {/* Background blobs */}
        <div className="absolute inset-0 overflow-hidden pointer-events-none">
          <div className="absolute -top-40 -right-40 w-96 h-96 bg-sky-200 rounded-full opacity-30 blur-3xl" />
          <div className="absolute -bottom-40 -left-40 w-96 h-96 bg-violet-200 rounded-full opacity-30 blur-3xl" />
        </div>

        <div className="relative max-w-4xl mx-auto text-center animate-fade-in">
          <div className="inline-flex items-center gap-2 bg-sky-100 text-sky-700 px-4 py-2 rounded-full text-sm font-medium mb-8 border border-sky-200">
            <Zap className="w-4 h-4" />
            Powered by Google Gemini AI
          </div>

          <h1 className="text-5xl md:text-7xl font-extrabold tracking-tight mb-6">
            <span className="bg-gradient-to-r from-sky-600 via-violet-600 to-sky-700 bg-clip-text text-transparent">
              FinWise AI
            </span>
          </h1>
          <p className="text-xl md:text-2xl text-slate-600 max-w-2xl mx-auto mb-4 leading-relaxed">
            Your personal AI financial advisor. Just describe your situation —
            get expert advice, smart budgets, and gamified savings.
          </p>
          <p className="text-base text-slate-500 mb-10">
            Works for India, USA, UK and more • Tax optimization • Debt strategy
          </p>

          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <button
              onClick={() => navigate('/analyze')}
              className="px-8 py-4 bg-gradient-to-r from-sky-500 to-violet-600 text-white font-semibold rounded-2xl shadow-lg hover:shadow-sky-300 hover:scale-105 transition-all duration-200 text-lg"
            >
              Start Free Analysis →
            </button>
            <button
              onClick={() => navigate('/signin')}
              className="px-8 py-4 bg-white text-slate-700 font-semibold rounded-2xl shadow border border-slate-200 hover:border-sky-300 hover:text-sky-700 transition-all duration-200 text-lg"
            >
              Sign Up 
            </button>
          </div>
        </div>
      </section>

      {/* Features */}
      <section className="max-w-6xl mx-auto px-4 pb-24">
        <h2 className="text-3xl font-bold text-center text-slate-800 mb-4">
          Everything you need to master your finances
        </h2>
        <p className="text-center text-slate-500 mb-12">
          AI analysis, budget planning, and gamified savings — all in one place.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {features.map(({ Icon, title, desc, color }) => (
            <div
              key={title}
              className="bg-white rounded-2xl p-6 shadow-sm border border-slate-100 hover:shadow-md hover:-translate-y-1 transition-all duration-200 group"
            >
              <div className={`w-12 h-12 rounded-xl bg-gradient-to-br ${color} flex items-center justify-center mb-4 shadow-sm group-hover:scale-110 transition-transform`}>
                <Icon className="w-6 h-6 text-white" />
              </div>
              <h3 className="text-lg font-semibold text-slate-800 mb-2">{title}</h3>
              <p className="text-slate-500 text-sm leading-relaxed">{desc}</p>
            </div>
          ))}
        </div>
      </section>

      {/* Bottom CTA */}
      <section className="bg-gradient-to-r from-sky-600 to-violet-700 py-16 px-4">
        <div className="max-w-2xl mx-auto text-center">
          <h2 className="text-3xl font-bold text-white mb-4">
            Ready to transform your finances?
          </h2>
          <p className="text-sky-100 mb-8 text-lg">
            Just describe your financial situation in plain text — no forms, no jargon.
          </p>
          <button
            onClick={() => navigate('/analyze')}
            className="px-10 py-4 bg-white text-sky-700 font-bold rounded-2xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-200 text-lg"
          >
            Analyze My Finances →
          </button>
        </div>
      </section>
    </div>
  )
}