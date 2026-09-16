import { REFERENCE_PI } from '../utils/referencePi'

export function Header() {
  return (
    <header className="app-header">
      <div className="header-title">
        <span className="pi-symbol">π</span>
        <div>
          <h1>Simulación de Algoritmos para π</h1>
          <p className="subtitle">Comparativa de convergencia y rendimiento · WebAssembly + Rust (dashu)</p>
        </div>
      </div>
      <div className="reference-box">
        <span className="reference-label">π de referencia</span>
        <code className="reference-value">{REFERENCE_PI}</code>
      </div>
    </header>
  )
}