import type { AlgorithmId, PiResult } from '../types'
import { ALGORITHMS } from '../types/algorithms'

interface ComparisonTableProps {
  results: Partial<Record<AlgorithmId, PiResult>>
}

export function ComparisonTable({ results }: ComparisonTableProps) {
  const rows = ALGORITHMS.map((meta) => ({
    meta,
    result: results[meta.id],
  }))
    .filter((row): row is { meta: (typeof ALGORITHMS)[number]; result: PiResult } => Boolean(row.result))
    .sort((a, b) => {
      if (a.result.correct_digits !== b.result.correct_digits) {
        return b.result.correct_digits - a.result.correct_digits
      }
      return a.result.time_ms - b.result.time_ms
    })

  if (rows.length === 0) {
    return (
      <section className="panel">
        <h2>Comparativa de resultados</h2>
        <p className="card-pending">
          Calcula al menos un algoritmo para ver la tabla comparativa de rendimiento aquí.
        </p>
      </section>
    )
  }

  return (
    <section className="panel">
      <h2>Comparativa de resultados</h2>
      <div className="table-wrap">
        <table className="result-table">
          <thead>
            <tr>
              <th>Algoritmo</th>
              <th>Categoría</th>
              <th className="num">Dígitos correctos</th>
              <th className="num">Iteraciones</th>
              <th className="num">Tiempo (ms)</th>
              <th className="num">Díg. / segundo</th>
              <th>Resultado</th>
            </tr>
          </thead>
          <tbody>
            {rows.map(({ meta, result }) => {
              const perSecond = result.time_ms > 0 ? result.correct_digits / (result.time_ms / 1000) : 0
              const shown = Math.min(result.correct_digits, result.digits.length)
              return (
                <tr key={meta.id}>
                  <td className="algo-name">{meta.name}</td>
                  <td>{meta.category}</td>
                  <td className="num">{result.correct_digits}</td>
                  <td className="num">{result.iterations.toLocaleString('es')}</td>
                  <td className="num">{result.time_ms.toFixed(2)}</td>
                  <td className="num">{perSecond.toFixed(1)}</td>
                  <td className="cell-digits">
                    <code className="digits-preview small">
                      <span className="digits-ok">{result.digits.slice(0, shown)}</span>
                      <span className="digits-rest">{result.digits.slice(shown, shown + 10)}</span>
                    </code>
                  </td>
                </tr>
              )
            })}
          </tbody>
        </table>
      </div>
      <p className="note">
        Los algoritmos estocásticos (Montecarlo, Buffon) quedan limitados a los dígitos estadísticamente
        significativos (~f64); los deterministas alcanzan la precisión solicitada.
      </p>
    </section>
  )
}