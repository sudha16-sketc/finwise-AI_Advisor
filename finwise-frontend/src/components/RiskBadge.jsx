

const config = {
  Low:    { bg: 'bg-emerald-100', text: 'text-emerald-700', dot: 'bg-emerald-500', label: 'Low Risk' },
  Medium: { bg: 'bg-amber-100',   text: 'text-amber-700',   dot: 'bg-amber-500',   label: 'Medium Risk' },
  High:   { bg: 'bg-rose-100',    text: 'text-rose-700',    dot: 'bg-rose-500',    label: 'High Risk' },
}

const sizes = {
  sm: 'px-2.5 py-1 text-xs',
  md: 'px-3.5 py-1.5 text-sm',
  lg: 'px-5 py-2 text-base',
}

export default function RiskBadge({ level, size = 'md' }) {
  const c = config[level] ?? config.Medium
  return (
    <span className={`inline-flex items-center gap-1.5 font-semibold rounded-full ${c.bg} ${c.text} ${sizes[size]}`}>
      <span className={`w-2 h-2 rounded-full ${c.dot} animate-pulse`} />
      {c.label}
    </span>
  )
}