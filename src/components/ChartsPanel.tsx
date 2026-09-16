import { useState } from 'react'
import type { AlgorithmId, ConvergencePoint } from '../types'
import { ALGORITHMS, SERIES_FN, ALGORITHM_BY_ID } from '../types/algorithms'
import { ConvergenceChart } from './ConvergenceChart'

interface ChartsPanelProps {
  series: Partial<Record<AlgorithmId, ConvergencePoint[]>>
}

export function ChartsPanel({ series }: ChartsPanelProps) {
  const available = ALGORITHMS.filter((a) => SERIES_FN[a.id] && (series[a.id]?.length ?? 0) > 0)
  const [selected, setSelected] = useState<AlgorithmId | ''>('')
  if (available.length === 0) return null
  const active = (selected || available[0].id) as AlgorithmId
  if (!available.some((a) => a.id === active)) setSelected(available[0].id)
  const points = series[active]
  const meta = ALGORITHM_BY_ID[active]

  return (
    <section className="panel">
      <h2>Análisis de convergencia</h2>
      <p className="note">
        Selecciona un algoritmo y pulsa «Ver convergencia» en su tarjeta para generar la serie. Las series
        deterministas alcanzan la precisión pedida; las estocásticas muestran la fluctuación estadística limitada a ~f64.
      </p>
      <div className="chart-controls">
        <label>
          Algoritmo
          <select value={active} onChange={(e) => setSelected(e.target.value as AlgorithmId)}>
            {available.map((a) => (
              <option key={a.id} value={a.id}>
                {a.name}
              </option>
            ))}
          </select>
        </label>
        <span className="chart-range">
          {points && points.length > 0 && (
            <>último punto: {points[points.length - 1].correct_digits} dígitos a los {points[points.length - 1].iteration.toLocaleString('es')} pasos</>
          )}
        </span>
      </div>
      {points && <ConvergenceChart points={points} algorithmName={meta.name} mode="digits" logX />}
      {points && points.length > 1 && <ConvergenceChart points={points} algorithmName={meta.name} mode="time" logX />}
    </section>
  )
}