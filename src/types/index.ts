export interface PiResult {
  algorithm: string
  digits: string
  time_ms: number
  iterations: number
  correct_digits: number
  params: string
}

export interface ConvergencePoint {
  iteration: number
  correct_digits: number
  time_ms: number
}

export type AlgorithmId =
  | 'montecarlo'
  | 'buffon'
  | 'archimedes'
  | 'gauss_legendre'
  | 'borwein'
  | 'ramanujan'
  | 'chudnovsky'
  | 'bbp'

export type AlgorithmCategory =
  | 'Estocástico'
  | 'Iterativo geométrico'
  | 'Serie (aritmética de alta exactitud)'
  | 'Spigot (dígito esporádico)'

export interface AlgorithmMeta {
  id: AlgorithmId
  name: string
  category: AlgorithmCategory
  origin: string
  description: string
  convergenceLabel: string
  runsOnPrecision: boolean
  runsOnSamples: boolean
  runsOnNeedles: boolean
  runsOnHex: boolean
  needsSeed: boolean
  hasSeries: boolean
  defaultPrecision: number
  defaultSamples: number
  defaultNeedles: number
  defaultHexDigits: number
  maxPrecision: number
  maxSamples: number
  maxNeedles: number
  maxHexDigits: number
  estimatedPrecision: string
}