use crate::types::{ConvergencePoint, PiResult};
use crate::util::*;

pub fn run(needles: u32, precision: u32, seed: u32) -> PiResult {
    let t0 = now_ms();
    let n = needles.max(1) as u64;
    let mut state = 0x853c49e6748fea9b ^ (seed as u64);
    if state == 0 {
        state = 0x9e3779b97f4a7c15;
    }
    let mut hits: u64 = 0;
    for _ in 0..n {
        let y = rand01(&mut state) * 0.5;
        let theta = rand01(&mut state) * std::f64::consts::PI;
        if y <= 0.5 * theta.sin() {
            hits += 1;
        }
    }
    let est = if hits > 0 {
        2.0 * (n as f64) / (hits as f64)
    } else {
        f64::INFINITY
    };
    let pi = from_f64(est);
    let t1 = now_ms();
    let requested = (precision as usize).clamp(1, 17);
    make_result(
        "buffon",
        &pi,
        requested,
        n,
        t1 - t0,
        &format!(r#"{{"needles":{},"seed":{}}}"#, n, seed),
    )
}

pub fn convergence(needles: u32, checkpoints: u32, seed: u32) -> Vec<ConvergencePoint> {
    let t0 = now_ms();
    let n = needles.max(1) as u64;
    let marks = checkpoints.max(2) as u64;
    let mut state = 0x853c49e6748fea9b ^ (seed as u64);
    if state == 0 {
        state = 0x9e3779b97f4a7c15;
    }
    let mut hits: u64 = 0;
    let mut points = Vec::new();
    let mut sample_at = 4u64;
    for i in 1..=n {
        let y = rand01(&mut state) * 0.5;
        let theta = rand01(&mut state) * std::f64::consts::PI;
        if y <= 0.5 * theta.sin() {
            hits += 1;
        }
        if i == sample_at || i == n {
            let est = if hits > 0 {
                2.0 * (i as f64) / (hits as f64)
            } else {
                f64::INFINITY
            };
            let pi = from_f64(est);
            let correct = correct_vs_pi(&pi, 12).min(8);
            points.push(ConvergencePoint {
                iteration: i,
                correct_digits: correct,
                time_ms: now_ms() - t0,
            });
            if i == n {
                break;
            }
            let ratio = (n as f64 / i as f64).powf(1.0 / marks as f64);
            sample_at = ((i as f64 * ratio).ceil() as u64).max(i + 1);
        }
    }
    points
}

fn rand01(state: &mut u64) -> f64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    (x >> 11) as f64 / (1u64 << 53) as f64
}