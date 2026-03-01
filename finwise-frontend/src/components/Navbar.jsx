import { Link, useLocation } from 'react-router-dom'
import { TrendingUp, PiggyBank, LayoutDashboard, Sparkles,ArrowLeftRight,History } from 'lucide-react'

const navItems = [
  { path: '/',          label: 'Home',      Icon: Sparkles },
  { path: '/analyze',   label: 'Analyze',   Icon: TrendingUp },
  { path: '/dashboard', label: 'Dashboard', Icon: LayoutDashboard },
  { path: '/piggy',     label: 'Piggy Bank',Icon: PiggyBank },
  { path: '/sendtransaction',     label: 'send XLM',Icon:ArrowLeftRight },
  { path: '/txhistory', label: 'Transaction History', Icon: History }
]

export default function Navbar() {
  const location = useLocation()

  return (
    <nav className="sticky top-0 z-50 bg-white/80 backdrop-blur-md border-b border-slate-200 shadow-sm">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">

          {/* Logo */}
          <Link to="/" className="flex items-center gap-2 group">
            <div className="w-8 h-8 bg-gradient-to-br from-sky-500 to-violet-600 rounded-lg flex items-center justify-center shadow-md group-hover:shadow-sky-300 transition-shadow">
              <TrendingUp className="w-4 h-4 text-white" />
            </div>
            <span className="text-xl font-bold bg-gradient-to-r from-sky-600 to-violet-600 bg-clip-text text-transparent">
              FinWise AI
            </span>
          </Link>

          {/* Desktop links */}
          <div className="hidden md:flex items-center gap-1">
            {navItems.map(({ path, label, Icon }) => {
              const active = location.pathname === path
              return (
                <Link
                  key={path}
                  to={path}
                  className={`flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                    active
                      ? 'bg-sky-50 text-sky-700'
                      : 'text-slate-600 hover:bg-slate-100 hover:text-slate-900'
                  }`}
                >
                  <Icon className="w-4 h-4" />
                  {label}
                </Link>
              )
            })}
          </div>

          {/* Mobile icon links */}
          <div className="flex md:hidden items-center gap-1">
            {navItems.map(({ path, Icon }) => {
              const active = location.pathname === path
              return (
                <Link
                  key={path}
                  to={path}
                  className={`p-2 rounded-lg transition-colors ${
                    active ? 'text-sky-600 bg-sky-50' : 'text-slate-500 hover:text-slate-700'
                  }`}
                >
                  <Icon className="w-5 h-5" />
                </Link>
              )
            })}
          </div>
        </div>
      </div>
    </nav>
  )
}