import { useCallback } from 'react'
import { Header } from './components/Header'
import { AlgorithmCard } from './components/AlgorithmCard'
import { ComparisonTable } from './components/ComparisonTable'
import { ChartsPanel } from './components/ChartsPanel'
import { usePi } from './hooks/usePi'
import { ALGORITHMS } from './types/algorithms'

export default function App() {
  const { results, series, busy, runningAll, error, compute, computeSeries, computeAll, clear } = usePi()

  const handleRunAll = useCallback(() => {
    void computeAll({
      precision: 100,
      samples: 1_000_000,
      needles: 1_000_000,
      hexDigits: 100,
      seed: Math.floor(Math.random() * 100000),
    })
  }, [computeAll])

  const algorithmsByCategory = ALGORITHMS.reduce((acc, meta) => {
    if (!acc[meta.category]) acc[meta.category] = []
    acc[meta.category].push(meta)
    return acc
  }, {} as Record<string, typeof ALGORITHMS>)

  return (
    <div className="app">
      <Header />
      {error && (
        <div className="banner-error">
          <strong>Error:</strong> {error}
        </div>
      )}
      <ComparisonTable results={results} />
      <section className="panel">
        <div className="panel-head">
          <h2>Algoritmos por Categoría</h2>
          <div style={{ display: 'flex', gap: '12px' }}>
            <button type="button" className="btn btn-primary" onClick={handleRunAll} disabled={runningAll || Object.values(busy).some(Boolean)}>
              {runningAll ? 'Calculando...' : 'Calcular todos (Rápido)'}
            </button>
            <button type="button" className="btn btn-ghost" onClick={clear}>
              Limpiar resultados
            </button>
          </div>
        </div>
        
        {Object.entries(algorithmsByCategory).map(([category, algos]) => (
          <div key={category} className="category-section" style={{ marginBottom: '2rem' }}>
            <h3 style={{ borderBottom: '1px solid #eee', paddingBottom: '0.5rem', marginBottom: '1rem', color: '#555' }}>
              {category}
            </h3>
            <div className="cards-grid">
              {algos.map((meta) => (
                <AlgorithmCard
                  key={meta.id}
                  meta={meta}
                  result={results[meta.id]}
                  series={series[meta.id]}
                  busy={Boolean(busy[meta.id])}
                  hasSeriesData={(series[meta.id]?.length ?? 0) > 0}
                  onCompute={(localConfig) => void compute(meta.id, localConfig)}
                  onSeries={(localConfig) => void computeSeries(meta.id, localConfig)}
                />
              ))}
            </div>
          </div>
        ))}
      </section>
      <ChartsPanel series={series} />
      <footer className="app-footer">
        <span>π · Simulación de algoritmos</span>
        <span>Cálculo en Rust (dashu) compilado a WebAssembly, ejecutado en un Web Worker.</span>
      </footer>
    </div>
  )
}