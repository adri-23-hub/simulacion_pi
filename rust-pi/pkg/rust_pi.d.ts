/* tslint:disable */
/* eslint-disable */

export function convergence_archimedes(max_doublings: number, precision: number): string;

export function convergence_borwein(iterations: number, precision: number): string;

export function convergence_buffon(needles: number, checkpoints: number, seed: number): string;

export function convergence_chudnovsky(terms: number, precision: number): string;

export function convergence_gauss_legendre(iterations: number, precision: number): string;

export function convergence_montecarlo(samples: number, seed: number): string;

export function convergence_ramanujan(terms: number, precision: number): string;

export function run_archimedes(max_doublings: number, precision: number): string;

export function run_bbp(digits: number, precision: number): string;

export function run_borwein(iterations: number, precision: number): string;

export function run_buffon(needles: number, precision: number, seed: number): string;

export function run_chudnovsky(terms: number, precision: number): string;

export function run_gauss_legendre(iterations: number, precision: number): string;

export function run_montecarlo(samples: number, seed: number): string;

export function run_ramanujan(terms: number, precision: number): string;

export function start(): void;

export function version(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly convergence_archimedes: (a: number, b: number) => [number, number];
    readonly convergence_borwein: (a: number, b: number) => [number, number];
    readonly convergence_buffon: (a: number, b: number, c: number) => [number, number];
    readonly convergence_chudnovsky: (a: number, b: number) => [number, number];
    readonly convergence_gauss_legendre: (a: number, b: number) => [number, number];
    readonly convergence_montecarlo: (a: number, b: number) => [number, number];
    readonly convergence_ramanujan: (a: number, b: number) => [number, number];
    readonly run_archimedes: (a: number, b: number) => [number, number];
    readonly run_bbp: (a: number, b: number) => [number, number];
    readonly run_borwein: (a: number, b: number) => [number, number];
    readonly run_buffon: (a: number, b: number, c: number) => [number, number];
    readonly run_chudnovsky: (a: number, b: number) => [number, number];
    readonly run_gauss_legendre: (a: number, b: number) => [number, number];
    readonly run_montecarlo: (a: number, b: number) => [number, number];
    readonly run_ramanujan: (a: number, b: number) => [number, number];
    readonly start: () => void;
    readonly version: () => [number, number];
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
