import { useCallback, useEffect, useRef, useState } from 'react'
import type { AlgorithmId, ConvergencePoint, PiResult } from '../types'
import { ALGORITHMS, RUN_FN, SERIES_FN } from '../types/algorithms'
import { callWorker, warmUpEngine } from '../utils/piEngine'

export interface RunConfig {
  precision: number
  samples: number
  needles: number
  hexDigits: number
  seed: number
}

export function runArgsFor(algorithm: AlgorithmId, config: RunConfig): number[] {
  switch (algorithm) {
    case 'montecarlo':
      return [config.samples, config.seed]
    case 'buffon':
      return [config.needles, 0, config.seed]
    case 'bbp':
      return [config.hexDigits, 0]
    default:
      return [0, config.precision]
  }
}

export function seriesArgsFor(algorithm: AlgorithmId, config: RunConfig): number[] {
  switch (algorithm) {
    case 'montecarlo':
      return [config.samples, config.seed]
    case 'buffon':
      return [config.needles, 40, config.seed]
    default:
      return [0, config.precision]
  }
}

export interface UsePi {
  results: Partial<Record<AlgorithmId, PiResult>>
  series: Partial<Record<AlgorithmId, ConvergencePoint[]>>
  busy: Partial<Record<AlgorithmId, boolean>>
  runningAll: boolean
  error: string | null
  compute: (algorithm: AlgorithmId, config: RunConfig) => Promise<void>
  computeSeries: (algorithm: AlgorithmId, config: RunConfig) => Promise<void>
  computeAll: (config: RunConfig) => Promise<void>
  clear: () => void
}

export function usePi(): UsePi {
  const [results, setResults] = useState<Partial<Record<AlgorithmId, PiResult>>>({})
  const [series, setSeries] = useState<Partial<Record<AlgorithmId, ConvergencePoint[]>>>({})
  const [busy, setBusy] = useState<Partial<Record<AlgorithmId, boolean>>>({})
  const [runningAll, setRunningAll] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const cancelledRef = useRef(false)

  useEffect(() => {
    warmUpEngine()
  }, [])

  const compute = useCallback(async (algorithm: AlgorithmId, config: RunConfig) => {
    setError(null)
    setBusy((prev) => ({ ...prev, [algorithm]: true }))
    try {
      const data = await callWorker(RUN_FN[algorithm], runArgsFor(algorithm, config))
      setResults((prev) => ({ ...prev, [algorithm]: data as PiResult }))
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setBusy((prev) => ({ ...prev, [algorithm]: false }))
    }
  }, [])

  const computeSeries = useCallback(async (algorithm: AlgorithmId, config: RunConfig) => {
    const fn = SERIES_FN[algorithm]
    if (!fn) return
    setError(null)
    setBusy((prev) => ({ ...prev, [algorithm]: true }))
    try {
      const data = await callWorker(fn, seriesArgsFor(algorithm, config))
      setSeries((prev) => ({ ...prev, [algorithm]: data as ConvergencePoint[] }))
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setBusy((prev) => ({ ...prev, [algorithm]: false }))
    }
  }, [])

  const computeAll = useCallback(async (config: RunConfig) => {
    setError(null)
    setRunningAll(true)
    cancelledRef.current = false
    for (const meta of ALGORITHMS) {
      if (cancelledRef.current) break
      setBusy((prev) => ({ ...prev, [meta.id]: true }))
      try {
        const data = await callWorker(RUN_FN[meta.id], runArgsFor(meta.id, config))
        setResults((prev) => ({ ...prev, [meta.id]: data as PiResult }))
      } catch (e) {
        setError((prev) =>
          prev ? `${prev}\n${meta.id}: ${e instanceof Error ? e.message : e}` : `${meta.id}: ${e instanceof Error ? e.message : e}`,
        )
      } finally {
        setBusy((prev) => ({ ...prev, [meta.id]: false }))
      }
    }
    setRunningAll(false)
  }, [])

  const clear = useCallback(() => {
    cancelledRef.current = true
    setResults({})
    setSeries({})
    setError(null)
  }, [])

  return { results, series, busy, runningAll, error, compute, computeSeries, computeAll, clear }
}