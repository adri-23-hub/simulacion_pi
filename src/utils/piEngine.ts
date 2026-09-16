import type { WorkerRequest, WorkerResponse } from '../types/worker'
import type { ConvergencePoint, PiResult } from '../types'

let worker: Worker | null = null
let nextId = 0
const pending = new Map<
  number,
  { resolve: (value: PiResult | ConvergencePoint[]) => void; reject: (error: Error) => void }
>()

function ensureWorker(): Worker {
  if (worker) return worker
  worker = new Worker(new URL('../workers/pi.worker.ts', import.meta.url), { type: 'module' })
  worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
    const entry = pending.get(event.data.id)
    if (!entry) return
    pending.delete(event.data.id)
    if (event.data.ok) {
      entry.resolve(event.data.data)
    } else {
      entry.reject(new Error(event.data.error))
    }
  }
  worker.onerror = (event) => {
    for (const [, entry] of pending) entry.reject(new Error(event.message || 'error en el worker'))
    pending.clear()
  }
  return worker
}

export function callWorker(fn: string, args: number[]): Promise<PiResult | ConvergencePoint[]> {
  const w = ensureWorker()
  const id = nextId++
  return new Promise<PiResult | ConvergencePoint[]>((resolve, reject) => {
    pending.set(id, { resolve, reject })
    const request: WorkerRequest = { id, fn, args }
    w.postMessage(request)
  })
}

export function warmUpEngine(): void {
  ensureWorker()
}