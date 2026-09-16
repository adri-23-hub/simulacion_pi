import type { WorkerRequest, WorkerResponse } from '../types/worker'
import type { ConvergencePoint, PiResult } from '../types'

interface WasmModule {
  [key: string]: (data: number, second?: number, third?: number) => string
}

import init, * as wasmMod from '../../rust-pi/pkg/rust_pi.js'
import wasmUrl from '../../rust-pi/pkg/rust_pi_bg.wasm?url'

let wasm: WasmModule | null = null

const ctx = self as unknown as Worker

async function getModule(): Promise<WasmModule> {
  if (wasm) return wasm
  await init(wasmUrl)
  wasm = wasmMod as unknown as WasmModule
  return wasm
}

ctx.onmessage = async (event: MessageEvent<WorkerRequest>) => {
  const { id, fn, args } = event.data
  const respond = (payload: WorkerResponse) => ctx.postMessage(payload)
  try {
    const mod = await getModule()
    const call = mod[fn]
    if (typeof call !== 'function') {
      respond({ id, ok: false, error: `función desconocida: ${fn}` })
      return
    }
    const raw = call(args[0] ?? 0, args[1] ?? 0, args[2] ?? 0)
    const data: PiResult | ConvergencePoint[] = JSON.parse(raw)
    respond({ id, ok: true, data })
  } catch (error) {
    respond({ id, ok: false, error: error instanceof Error ? error.message : String(error) })
  }
}

export {};