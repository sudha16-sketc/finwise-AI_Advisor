import { PieChart, Pie, Cell, Legend, Tooltip, ResponsiveContainer } from 'recharts'

const COLORS = ['#0ea5e9', '#8b5cf6', '#10b981']

export default function BudgetChart({ budgetPlan }) {
  const data = [
    { name: 'Needs',   value: budgetPlan.needs },
    { name: 'Wants',   value: budgetPlan.wants },
    { name: 'Savings', value: budgetPlan.savings },
  ]

  return (
    <div className="w-full h-64">
      <ResponsiveContainer width="100%" height="100%">
        <PieChart>
          <Pie
            data={data}
            cx="50%"
            cy="50%"
            innerRadius={55}
            outerRadius={85}
            paddingAngle={4}
            dataKey="value"
          >
            {data.map((_, index) => (
              <Cell key={`cell-${index}`} fill={COLORS[index]} strokeWidth={0} />
            ))}
          </Pie>
          <Tooltip
            formatter={(value) => [`${value}%`, '']}
            contentStyle={{ borderRadius: '12px', border: 'none', boxShadow: '0 4px 20px rgba(0,0,0,0.1)' }}
          />
          <Legend
            formatter={(value) => <span className="text-sm text-slate-600">{value}</span>}
          />
        </PieChart>
      </ResponsiveContainer>
    </div>
  )
}