mod archimedes;
mod bbp;
mod borwein;
mod buffon;
mod chudnovsky;
mod gauss_legendre;
mod montecarlo;
mod ramanujan;
mod types;
mod util;

use types::PiResult;
use util::{json, now_ms};

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn version() -> String {
    "0.1.0".to_string()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_montecarlo(samples: u32, seed: u32) -> String {
    wrap(|| json(&montecarlo::run(samples as u64, seed as u64, true)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_buffon(needles: u32, precision: u32, seed: u32) -> String {
    wrap(|| json(&buffon::run(needles, precision, seed)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_archimedes(max_doublings: u32, precision: u32) -> String {
    wrap(|| json(&archimedes::run(max_doublings, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_gauss_legendre(iterations: u32, precision: u32) -> String {
    wrap(|| json(&gauss_legendre::run(iterations, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_borwein(iterations: u32, precision: u32) -> String {
    wrap(|| json(&borwein::run(iterations, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_ramanujan(terms: u32, precision: u32) -> String {
    wrap(|| json(&ramanujan::run(terms, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_chudnovsky(terms: u32, precision: u32) -> String {
    wrap(|| json(&chudnovsky::run(terms, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_bbp(digits: u32, precision: u32) -> String {
    wrap(|| json(&bbp::run(digits, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn convergence_montecarlo(samples: u32, seed: u32) -> String {
    wrap(|| json(&montecarlo::convergence(samples as u64, seed as u64, false)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn convergence_buffon(needles: u32, checkpoints: u32, seed: u32) -> String {
    wrap(|| json(&buffon::convergence(needles, checkpoints, seed)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn convergence_archimedes(max_doublings: u32, precision: u32) -> String {
    wrap(|| json(&archimedes::convergence(max_doublings, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn convergence_gauss_legendre(iterations: u32, precision: u32) -> String {
    wrap(|| json(&gauss_legendre::convergence(iterations, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn convergence_borwein(iterations: u32, precision: u32) -> String {
    wrap(|| json(&borwein::convergence(iterations, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn convergence_ramanujan(terms: u32, precision: u32) -> String {
    wrap(|| json(&ramanujan::convergence(terms, precision)))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn convergence_chudnovsky(terms: u32, precision: u32) -> String {
    wrap(|| json(&chudnovsky::convergence(terms, precision)))
}

pub fn run_all(precision: u32, samples: u32) -> String {
    wrap(|| {
        let mut results: Vec<PiResult> = Vec::new();
        results.push(montecarlo::run(samples as u64, 1u64, true));
        results.push(chudnovsky::run(0, precision));
        results.push(gauss_legendre::run(0, precision));
        results.push(borwein::run(0, precision));
        results.push(ramanujan::run(0, precision));
        results.push(archimedes::run(0, precision));
        json(&results)
    })
}

fn wrap<F: FnOnce() -> String>(f: F) -> String {
    let t0 = now_ms();
    let out = f();
    let _ = t0;
    out
}