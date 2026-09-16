use crate::types::{ConvergencePoint, PiResult};
use crate::util::*;

fn required_precision(samples: u64) -> usize {
    (0.5 * (samples as f64).log2()).ceil() as usize + 2
}

fn estimate(efficient: bool, seed: u64, samples: u64) -> f64 {
    let mut state = seed;
    let mut hits: u64 = 0;
    if efficient {
        for _ in 0..samples {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let x = (state >> 11) as f64 / 9007199254740992.0;
            let y = ((state.wrapping_mul(1664525).wrapping_add(1013904223)) >> 11) as f64 / 9007199254740992.0;
            if x * x + y * y <= 1.0 {
                hits += 1;
            }
        }
    } else {
        for _ in 0..samples {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let x = (state & 0x1FFFFF) as f64 / 2097152.0;
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let y = (state & 0x1FFFFF) as f64 / 2097152.0;
            if x * x + y * y <= 1.0 {
                hits += 1;
            }
        }
    }
    4.0 * (hits as f64) / (samples as f64)
}

pub fn run(samples: u64, seed: u64, efficient: bool) -> PiResult {
    let t0 = now_ms();
    let n = if samples > 0 { samples } else { 10_000_000 };
    let work = required_precision(n);
    let est = estimate(efficient, seed, n);
    let pi = from_f64(est);
    let t1 = now_ms();
    make_result(
        if efficient { "montecarlo_efficient" } else { "montecarlo" },
        &pi,
        work,
        n,
        t1 - t0,
        &format!(r#"{{"samples":{}}}"#, n),
    )
}

pub fn convergence(samples: u64, seed: u64, efficient: bool) -> Vec<ConvergencePoint> {
    let t0 = now_ms();
    let n = if samples > 0 { samples } else { 1_000_000 };
    let ref_str = to_pi_string(&pi_ref(22), 22);
    let steps: Vec<u64> = if n <= 1000 {
        (1..=n).collect()
    } else {
        let step = (n / 1000).max(1);
        (1..=n).step_by(step as usize).collect()
    };
    let mut points = Vec::new();
    let mut state = seed;
    let mut hits: u64 = 0;
    let mut last_idx = 0usize;
    let mut past_limit = false;
    for i in 1..=n {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let (x, y) = if efficient {
            let x = (state >> 11) as f64 / 9007199254740992.0;
            let y = ((state.wrapping_mul(1664525).wrapping_add(1013904223)) >> 11) as f64 / 9007199254740992.0;
            (x, y)
        } else {
            let x = (state & 0x1FFFFF) as f64 / 2097152.0;
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let y = (state & 0x1FFFFF) as f64 / 2097152.0;
            (x, y)
        };
        if x * x + y * y <= 1.0 {
            hits += 1;
        }
        if past_limit || (last_idx < steps.len() && steps[last_idx] == i) {
            let est = 4.0 * (hits as f64) / (i as f64);
            let pi = from_f64(est);
            let computed = to_pi_string(&pi, 22);
            let correct = count_correct_decimal(&computed, &ref_str);
            if last_idx < steps.len() {
                last_idx += 1;
            }
            points.push(ConvergencePoint {
                iteration: i,
                correct_digits: correct,
                time_ms: now_ms() - t0,
            });
            if last_idx >= steps.len() {
                past_limit = true;
            }
        }
    }
    points
}