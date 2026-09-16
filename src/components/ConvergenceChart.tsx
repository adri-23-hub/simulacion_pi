import {
  CartesianGrid,
  Legend,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts'
import type { ConvergencePoint } from '../types'

export interface ConvergenceChartProps {
  points: ConvergencePoint[]
  algorithmName: string
  mode: 'digits' | 'time'
  logX: boolean
}

export function ConvergenceChart({ points, algorithmName, mode, logX }: ConvergenceChartProps) {
  if (points.length === 0) return null
  const data = points.map((p) => ({ ...p }))
  const yLabel = mode === 'digits' ? 'Dígitos correctos' : 'Tiempo acumulado (ms)'
  const dataKey = mode === 'digits' ? 'correct_digits' : 'time_ms'
  return (
    <div className="chart">
      <h4>
        Convergencia de {algorithmName} <span className="chart-mode">{mode === 'digits' ? '· dígitos' : '· tiempo'}</span>
      </h4>
      <ResponsiveContainer width="100%" height={260}>
        <LineChart data={data} margin={{ top: 8, right: 16, bottom: 4, left: 0 }}>
          <CartesianGrid strokeDasharray="3 3" stroke="rgba(255, 255, 255, 0.1)" />
          <XAxis
            dataKey="iteration"
            type="number"
            scale={logX ? 'log' : 'linear'}
            domain={['auto', 'auto']}
            label={{ value: 'Iteración', position: 'insideBottom', offset: -2 }}
            tickFormatter={(v: number) => (v >= 1000 ? `${(v / 1000).toFixed(0)}K` : String(v))}
            stroke="rgba(255, 255, 255, 0.5)"
          />
          <YAxis label={{ value: yLabel, angle: -90, position: 'insideLeft' }} stroke="rgba(255, 255, 255, 0.5)" />
          <Tooltip
            formatter={(value) => [
              mode === 'digits' ? `${value} dígitos` : `${Number(value ?? 0).toFixed(2)} ms`,
              algorithmName,
            ]}
            labelFormatter={(v) => `iteración ${v}`}
            contentStyle={{ backgroundColor: 'rgba(30, 41, 59, 0.9)', borderColor: 'rgba(255, 255, 255, 0.1)' }}
          />
          <Legend />
          <Line
            type="monotone"
            dataKey={dataKey}
            name={mode === 'digits' ? 'Dígitos correctos' : 'Tiempo acumulado'}
            stroke={mode === 'digits' ? '#38bdf8' : '#fbbf24'}
            strokeWidth={3}
            dot={false}
            isAnimationActive={true}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  )
}