# Plan de Implementación — Aplicación de Aproximación de π con Análisis Comparativo

Aplicación para la solución del método de aproximación de π con los siguientes algoritmos, además del análisis de comparativa de rendimiento de cada solución.

## Algoritmos a implementar

1. **Algoritmos Probabilísticos y Estocásticos (Simulación)**
   - Algoritmo de Montecarlo Estándar
   - Algoritmo de Aguja de Buffon

2. **Algoritmos Geométricos e Iterativos**
   - Método de Polígonos de Arquímedes
   - Algoritmo de Gauss-Legendre
   - Algoritmos de Borwein

3. **Algoritmos de Series Infinitas y Análisis Numérico**
   - Algoritmos de Ramanujan
   - Algoritmo de Chudnovsky

4. **Algoritmos de Extracción Directa de Dígitos (Spigot)**
   - Algoritmo BBP (Bailey-Borwein-Plouffe)

---

## Pila tecnológica

| Capa | Tecnología | Motivo |
|------|-----------|--------|
| Frontend | Vite + React + TypeScript | SPA moderna, componentes y buena DX |
| Gráficas | Recharts (React-native) | Bar, Line y Scatter charts |
| Tema visual | Light / académico | Interfaz clara, lectura académica de resultados |
| Cómputo híbrido | **Rust → WebAssembly** via `wasm-pack` + `wasm-bindgen` | Cálculo nativo con precisión arbitraria |
| Big numbers | `dashu` (pure-Rust, sin dependencias C, compila a wasm32) | Precisión arbitraria configurada por el usuario |

---

## 1. Instalación de herramientas (pre-requisitos)

```bash
# Instalar Rust (rustup)
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
& "$env:TEMP\rustup-init.exe" -y

# Añadir target wasm32
rustup target add wasm32-unknown-unknown

# Instalar wasm-pack
cargo install wasm-pack

# Inicializar proyecto Vite + React + TypeScript
npm create vite@latest . -- --template react-ts
```

---

## 2. Estructura del proyecto

```
Pi_Simulacion/
├── rust-pi/                  # Cráter Rust → wasm
│   ├── Cargo.toml            # dashu, wasm-bindgen, serde, serde-wasm-bindgen, console_error_panic_hook
│   └── src/
│       ├── lib.rs            # Punto de entrada wasm-bindgen, re-exports
│       ├── montecarlo.rs     # Algoritmo 1a: Montecarlo Estándar
│       ├── buffon.rs         # Algoritmo 1b: Aguja de Buffon
│       ├── archimedes.rs     # Algoritmo 2a: Polígonos de Arquímedes
│       ├── gauss_legendre.rs # Algoritmo 2b: Gauss-Legendre
│       ├── borwein.rs        # Algoritmo 2c: Borwein (quartic)
│       ├── ramanujan.rs      # Algoritmo 3a: Ramanujan (1/π)
│       ├── chudnovsky.rs     # Algoritmo 3b: Chudnovsky
│       ├── bbp.rs            # Algoritmo 4: BBP (dígito hexadecimal)
│       └── types.rs          # PiResult, ConvergencePoint (serde)
├── src/                      # React app
│   ├── main.tsx
│   ├── App.tsx
│   ├── components/
│   │   ├── Header.tsx
│   │   ├── ControlPanel.tsx   # Selector de algoritmo, precisión, muestras, botón "Calcular"
│   │   ├── ResultTable.tsx    # Tabla comparativa: tiempo, dígitos correctos, iteraciones
│   │   ├── Charts.tsx         # Recharts: bar (tiempo), line (convergencia), scatter (Monte Carlo)
│   │   └── AlgorithmCard.tsx  # Card individual con resultado y métricas
│   ├── hooks/
│   │   └── usePiWasm.ts       # Hook que carga el módulo wasm, expone funciones
│   ├── utils/
│   │   ├── referencePi.ts     # π decimal conocido (~10k dígitos) para verificar dígitos correctos
│   │   └── hexPi.ts           # π en hexadecimal para BBP
│   └── types/
│       └── index.ts           # PiResult, AlgorithmId, ConvergencePoint (TS types)
├── public/
│   ├── wasm/                  # Artefactos del build de wasm
│   │   ├── rust_pi_bg.wasm
│   │   └── worker.ts          # Web Worker para no bloquear la UI
├── index.html
├── vite.config.ts             # Integración del plugin vite-plugin-wasm
├── tsconfig.json
└── package.json
```

---

## 3. Módulo Rust (`rust-pi/`)

### 3a. `Cargo.toml`

```toml
[package]
name = "rust-pi"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
dashu = { version = "0.6", features = ["std"] }
wasm-bindgen = "0.2"
serde = { version = "1", features = ["derive"] }
serde-wasm-bindgen = "0.6"
console_error_panic_hook = "0.1"
js-sys = "0.3"
```

### 3b. `types.rs` — Estructuras de datos compartidas

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PiResult {
    pub digits: String,           // π decimal calculado
    pub time_ms: f64,
    pub iterations: u64,
    pub correct_digits: usize,    // dígitos correctos vs referencia
    pub algorithm: String,
    pub params: String,           // JSON con parámetros usados
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConvergencePoint {
    pub iteration: u64,
    pub correct_digits: usize,
    pub time_ms: f64,
}
```

### 3c. Algoritmos implementados

| Algoritmo | Tipo | Fórmula clave | Convergencia |
|-----------|------|---------------|-------------|
| Montecarlo Estándar | Estocástico | `π ≈ 4 × (hits/total)` con puntos aleatorios en cuadrante unitario | O(1/√N) — ~1 dígito por cada factor 10× de muestras |
| Aguja de Buffon | Estocástico | `π ≈ (2 × L × N) / (H × hits)` donde L=aguja, H=distancia entre líneas | O(1/√N) — muy lento |
| Arquímedes | Iterativo geométrico | Límite de perímetros de polígonos inscrito/circunscrito duplicando lados | Aprox. 1 dígito por duplicación de lados |
| Gauss-Legendre | Iterativo geométrico (AGM) | Inicializa a₀=1, b₀=1/√2, t₀=1/4; iteraciones AGM | Cuadrática — ~2 dígitos por iteración |
| Borwein (quartic) | Iterativo geométrico | Recurrencia con yₙ = (1-√⁴(1-yₙ²)) / (1+√⁴(1-yₙ²)) | Cuártica — ~4 dígitos por iteración |
| Ramanujan (1/π) | Serie infinita | `1/π = (2√2/9801) Σₖ ((4k)!(1103+26390k)) / ((k!)⁴ 396⁴ᵏ)` | ~8 dígitos cada ~ ... términos |
| Chudnovsky | Serie infinita | `1/π = 12 Σₖ ((-1)ᵏ (6k)!(13591409+545140134k)) / ((3k)!(k!)³ 640320³ᵏ⁺³/²)` | ~14 dígitos por término |
| BBP (spigot) | Extracción directa hex | `π = Σₖ (4/(8k+1) - 2/(8k+4) - 1/(8k+5) - 1/(8k+6)) × 16⁻ᵏ` | Extrae el n-ésimo dígito hexadecimal sin calcular los anteriores |

**Estrategia para estocásticos (Montecarlo, Buffon):**
- Usar `dashu::float::DBig` (base-10) para el ratio de conteos.
- El resultado real solo tiene ~log₁₀(√N) dígitos correctos; eso es parte del análisis comparativo (demuestran que estos métodos no sirven para alta precisión).
- Semilla de PRNG proporcionada desde JS para reproducibilidad.

**Estrategia para BBP:**
- Implementar el spigot BBP completo para extraer N dígitos hexadecimales secuenciales.
- Convertir el π decimal de referencia (Chudnovsky) a hexadecimal para comparar.
- Mostrar resultados en hexadecimal (formato nativo de BBP).

### 3d. API expuesta via wasm-bindgen

```rust
// Cada función devuelve el PiResult serializado (JsValue)
#[wasm_bindgen]
pub fn run_montecarlo(samples: u64, precision: usize) -> JsValue;
pub fn run_buffon(needles: u64, precision: usize) -> JsValue;
pub fn run_archimedes(max_sides: u32, precision: usize) -> JsValue;
pub fn run_gauss_legendre(iterations: u32, precision: usize) -> JsValue;
pub fn run_borwein(iterations: u32, precision: usize) -> JsValue;
pub fn run_ramanujan(terms: u32, precision: usize) -> JsValue;
pub fn run_chudnovsky(terms: u32, precision: usize) -> JsValue;
pub fn run_bbp(digits: u32) -> JsValue;

// Funciones de convergencia (para gráficas de línea)
pub fn convergence_montecarlo(samples: u64, checkpoints: u32) -> JsValue; // Vec<ConvergencePoint>
pub fn convergence_archimedes(max_sides: u32) -> JsValue;
pub fn convergence_gauss_legendre(iterations: u32) -> JsValue;
// ... resto de algoritmos

// Benchmark completo
pub fn run_all(precision: usize, max_iterations: u32) -> JsValue;
```

Flujo interno de cada función:
1. Crear `Context<HalfEven>` con la precisión pedida + 10 dígitos de guarda.
2. Ejecutar el algoritmo.
3. Redondear a la precisión final.
4. Medir tiempo con `js_sys::Date::now()` (antes/después).
5. Contar dígitos correctos contra referencia interna (Chudnovsky a precisión extra).
6. Serializar `PiResult` a `JsValue` via `serde-wasm-bindgen`.

### 3e. Build

```bash
cd rust-pi
wasm-pack build --target web --release
# Copiar artefactos de pkg/ a public/wasm/
```

---

## 4. Frontend React (`src/`)

### 4a. `hooks/usePiWasm.ts`
- Carga dinámica del módulo wasm al montar la app (`init()` de wasm-bindgen).
- Expone funciones `runMontecarlo(samples, precision)` → `PiResult`, etc.
- Estado: `loading`, `error`, `results: PiResult[]`, `convergenceData`.

### 4b. `ControlPanel.tsx`
- Dropdown para seleccionar algoritmo (o "Benchmark completo").
- Input numérico: precisión (dígitos, rango 10–5000).
- Input numérico: iteraciones/muestras (según algoritmo).
- Slider para checkpoints de convergencia (datos para las gráficas).
- Botón "Calcular" → invoca fue el wasm en un Web Worker para no bloquear la UI.

### 4c. `ResultTable.tsx`
Tabla comparativa con columnas:

| Algoritmo | Tiempo (ms) | Dígitos correctos | Iteraciones | Tasa (dígitos/seg) | Categoría |
|-----------|-------------|-------------------|-------------|--------------------|-----------|

- La fila más rápida para la precisión actual queda resaltada.

### 4d. `Charts.tsx` (Recharts)
- **BarChart**: tiempo de ejecución por algoritmo (eje Y común para comparar).
- **LineChart**: convergencia — dígitos correctos vs iteración/parámetro.
- **ScatterChart**: Montecarlo — puntos (X=muestra, Y=dígito correcto) para ver el ruido estocástico.

### 4e. `AlgorithmCard.tsx`
Card expandible por algoritmo con:
- Valor calculado de π (primeros ~50 dígitos visibles).
- Métricas: tiempo, iteraciones, dígitos correctos.
- Sparkline de convergencia (cuando aplique).

### 4f. Referencia π
- Archivos con π en decimal (10,000 dígitos) y hexadecimal (para BBP).
- Función `countCorrectDigits(computed, reference)` que compara posición por posición, ignorando el punto decimal.

---

## 5. Ejecución en Web Worker

Para no bloquear la UI durante cálculos pesados, el wasm se ejecuta en un Web Worker:

- `public/wasm/worker.ts` — recibe mensajes `{ algorithm, params }`, ejecuta la función wasm y devuelve el resultado al hilo principal.
- El hook `usePiWasm` despacha mensajes al worker y recibe los resultados.

Criticidad: Chudnovsky a 5000 dígitos puede tomar cientos de ms; el worker evita congelar la interfaz.

---

## 6. Tema visual light / académico

- Paleta: fondo `#FAFAFA`, cards `#FFFFFF` con `box-shadow` suave, acento azul `#1976D2`.
- Tipografía: Inter (Google Fonts) o system-ui.
- Layout: header fijo + 2 columnas (panel de control izquierda / resultados y charts derecha), responsive.
- Tablas con bordes sutiles y zebra striping.
- Charts con color por categoría:
  - Probabilísticos = naranja
  - Geométricos = azul
  - Series infinitas = verde
  - Spigot = morado

---

## 7. Orden de implementación

| Paso | Archivos | Descripción |
|------|----------|-------------|
| 1 | — | Instalar Rust + wasm-pack + crear proyecto Vite React TS |
| 2 | `rust-pi/` | Crear crate, `types.rs`, `lib.rs` (stub) |
| 3 | Rust | `chudnovsky.rs` (primero, sirve de referencia), `montecarlo.rs`, `buffon.rs` |
| 4 | Rust | `archimedes.rs`, `gauss_legendre.rs`, `borwein.rs`, `ramanujan.rs`, `bbp.rs` |
| 5 | Rust | `lib.rs` — exponer todas las funciones + `run_all` benchmark |
| 6 | — | Build wasm: `wasm-pack build --target web --release` |
| 7 | React | `usePiWasm.ts`, tipos TS |
| 8 | UI | `Header`, `ControlPanel`, `AlgorithmCard` |
| 9 | UI | `ResultTable.tsx` |
| 10 | UI | `Charts.tsx` con Recharts |
| 11 | — | `worker.ts` + integración |
| 12 | — | `referencePi.ts` + lógica de dígitos correctos |
| 13 | — | Testing visual de cada algoritmo con distintas precisiones |

---

## 8. Dependencias npm

```json
{
  "dependencies": {
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "recharts": "^2.15.0"
  },
  "devDependencies": {
    "@types/react": "^19.0.0",
    "@types/react-dom": "^19.0.0",
    "@vitejs/plugin-react": "^4.0.0",
    "typescript": "^5.6.0",
    "vite": "^6.0.0",
    "vite-plugin-wasm": "^3.0.0",
    "top-level-await": "^2.2.0"
  }
}
```

---

## 9. Limitaciones conocidas

- **Montecarlo / Buffon** no pueden producir alta precisión (convergencia O(1/√N)); esto es parte del análisis comparativo (por qué no sirven para miles de dígitos).
- **BBP** produce dígitos hexadecimales; la comparación se realiza en base 16.
- wasm-bindgen no soporta threads directamente; se usan Web Workers individuales.
- `dashu` no es el backend más rápido el mundo (vs GMP), pero es la mejor opción para wasm sin dependencias C.