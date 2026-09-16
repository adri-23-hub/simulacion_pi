import type { ConvergencePoint, PiResult } from './index'

export interface WorkerRequest {
  id: number
  fn: string
  args: number[]
}

export type WorkerResponse =
  | { id: number; ok: true; data: PiResult | ConvergencePoint[] }
  | { id: number; ok: false; error: string }