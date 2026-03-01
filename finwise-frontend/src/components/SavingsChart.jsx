import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts'

/** Generates a mock savings growth trend from total saved + streak length */
function generateData(totalSaved, streak) {
  const points = Math.max(streak, 7)
  const perDay = points > 0 ? totalSaved / points : 0
  return Array.from({ length: points }, (_, i) => ({
    day: `Day ${i + 1}`,
    saved: Math.round(perDay * (i + 1)),
  }))
}

export default function SavingsChart({ totalSaved, streak }) {
  const data = generateData(totalSaved, streak)

  return (
    <div className="w-full h-56">
      <ResponsiveContainer width="100%" height="100%">
        <AreaChart data={data} margin={{ top: 5, right: 10, left: 0, bottom: 5 }}>
          <defs>
            <linearGradient id="savingsGradient" x1="0" y1="0" x2="0" y2="1">
              <stop offset="5%"  stopColor="#0ea5e9" stopOpacity={0.3} />
              <stop offset="95%" stopColor="#0ea5e9" stopOpacity={0} />
            </linearGradient>
          </defs>
          <CartesianGrid strokeDasharray="3 3" stroke="#f1f5f9" />
          <XAxis
            dataKey="day"
            tick={{ fontSize: 11, fill: '#94a3b8' }}
            tickLine={false}
            axisLine={false}
            interval={Math.floor(data.length / 5)}
          />
          <YAxis
            tick={{ fontSize: 11, fill: '#94a3b8' }}
            tickLine={false}
            axisLine={false}
            tickFormatter={(v) => `₹${v}`}
          />
          <Tooltip
            formatter={(value) => [`₹${value.toLocaleString()}`, 'Total Saved']}
            contentStyle={{ borderRadius: '12px', border: 'none', boxShadow: '0 4px 20px rgba(0,0,0,0.1)' }}
          />
          <Area
            type="monotone"
            dataKey="saved"
            stroke="#0ea5e9"
            strokeWidth={2.5}
            fill="url(#savingsGradient)"
          />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  )
}