import { useState, type ReactNode } from 'react'
import type { AlgorithmMeta, ConvergencePoint, PiResult } from '../types'
import type { RunConfig } from '../hooks/usePi'

function DigitsPreview({ digits, correct }: { digits: string; correct: number }) {
  const matched = digits.slice(0, correct)
  const rest = digits.slice(correct, correct + 12)
  return (
    <code className="digits-preview">
      <span className="digits-ok">{matched}</span>
      <span className="digits-rest">{rest}</span>
      {digits.length > correct + 12 && <span className="digits-rest">…</span>}
    </code>
  )
}

function Stat({ label, value }: { label: string; value: ReactNode }) {
  return (
    <div className="stat">
      <span className="stat-label">{label}</span>
      <span className="stat-value">{value}</span>
    </div>
  )
}

export interface AlgorithmCardProps {
  meta: AlgorithmMeta
  result?: PiResult
  series?: ConvergencePoint[]
  busy: boolean
  hasSeriesData: boolean
  onCompute: (config: RunConfig) => void
  onSeries: (config: RunConfig) => void
  error?: string
}

export function AlgorithmCard({
  meta,
  result,
  series,
  busy,
  hasSeriesData,
  onCompute,
  onSeries,
  error,
}: AlgorithmCardProps) {
  const [config, setConfig] = useState<RunConfig>({
    precision: meta.defaultPrecision || 100,
    samples: meta.defaultSamples || 10_000_000,
    needles: meta.defaultNeedles || 1_000_000,
    hexDigits: meta.defaultHexDigits || 100,
    seed: Math.floor(Math.random() * 100000),
  })

  let inputElement = null
  if (meta.runsOnPrecision) {
    inputElement = (
      <div className="card-param">
        <span className="param-label">Precisión (dígitos)</span>
        <input type="number" min={10} max={meta.maxPrecision} step={1} value={config.precision} onChange={e => setConfig({...config, precision: Number(e.target.value)})} />
      </div>
    )
  } else if (meta.runsOnSamples) {
    inputElement = (
      <div className="card-param">
        <span className="param-label">Muestras</span>
        <input type="number" min={1000} max={meta.maxSamples} step={1000} value={config.samples} onChange={e => setConfig({...config, samples: Number(e.target.value)})} />
      </div>
    )
  } else if (meta.runsOnNeedles) {
    inputElement = (
      <div className="card-param">
        <span className="param-label">Agujas de Buffon</span>
        <input type="number" min={1000} max={meta.maxNeedles} step={1000} value={config.needles} onChange={e => setConfig({...config, needles: Number(e.target.value)})} />
      </div>
    )
  } else if (meta.runsOnHex) {
    inputElement = (
      <div className="card-param">
        <span className="param-label">Dígitos Hexadecimales</span>
        <input type="number" min={10} max={meta.maxHexDigits} step={1} value={config.hexDigits} onChange={e => setConfig({...config, hexDigits: Number(e.target.value)})} />
      </div>
    )
  }

  let warningMessage = null;
  if (meta.runsOnPrecision && config.precision > meta.maxPrecision) {
    warningMessage = `⚠️ Superar los ${meta.maxPrecision.toLocaleString('es')} dígitos puede demorar mucho y colgar la app.`;
  } else if (meta.runsOnSamples && config.samples > meta.maxSamples) {
    warningMessage = `⚠️ Superar ${meta.maxSamples.toLocaleString('es')} muestras puede demorar.`;
  } else if (meta.runsOnNeedles && config.needles > meta.maxNeedles) {
    warningMessage = `⚠️ Superar ${meta.maxNeedles.toLocaleString('es')} agujas puede demorar.`;
  } else if (meta.runsOnHex && config.hexDigits > meta.maxHexDigits) {
    warningMessage = `⚠️ Superar ${meta.maxHexDigits.toLocaleString('es')} dígitos hex puede demorar.`;
  }

  return (
    <article className="card algorithm-card">
      <div className="card-head">
        <h3>{meta.name}</h3>
        <span className={`badge badge-${meta.category === 'Estocástico' ? 'stoch' : 'det'}`}>{meta.category}</span>
      </div>
      <p className="card-origin">{meta.origin}</p>
      <p className="card-desc">{meta.description}</p>
      <p className="card-convergence">
        <strong>Convergencia:</strong> {meta.convergenceLabel}
      </p>

      {result ? (
        <div className="result-block">
          <div className="stat-row">
            <Stat label="Dígitos correctos" value={result.correct_digits} />
            <Stat label="Iteraciones" value={result.iterations.toLocaleString('es')} />
            <Stat label="Tiempo" value={`${result.time_ms.toFixed(2)} ms`} />
          </div>
          <DigitsPreview digits={result.digits} correct={Math.min(result.correct_digits, result.digits.length)} />
          {series && series.length > 0 && hasSeriesData ? (
            <span className="series-note">{series[series.length - 1].correct_digits} dígitos al final de la serie</span>
          ) : null}
        </div>
      ) : error ? (
        <p className="card-error">{error}</p>
      ) : (
        <p className="card-pending">Sin calcular todavía.</p>
      )}

      <div className="card-controls">
        {inputElement}
        {warningMessage && <div className="card-warning">{warningMessage}</div>}
      </div>

      <div className="card-actions">
        <button type="button" className="btn btn-outline" onClick={() => {
          let currentConfig = { ...config };
          if (meta.needsSeed) {
            currentConfig.seed = Math.floor(Math.random() * 100000);
          }
          setConfig(currentConfig);
          onCompute(currentConfig);
        }} disabled={busy}>
          {busy ? 'Calculando…' : 'Calcular'}
        </button>
        {meta.hasSeries && (
          <button type="button" className="btn btn-ghost" onClick={() => {
             onSeries(config);
          }} disabled={busy}>
            {hasSeriesData ? 'Actualizar convergencia' : 'Ver convergencia'}
          </button>
        )}
      </div>
    </article>
  )
}