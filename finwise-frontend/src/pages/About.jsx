import React from "react";
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
export default function About() {
    
    return (
        <section className="max-w-6xl mt-10 mx-auto px-4 pb-24">
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
              className="bg-white/60 backdrop-blur-lg rounded-2xl p-6 shadow-sm border border-slate-100 hover:shadow-md hover:-translate-y-1 transition-all duration-200 group"
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
    )
}