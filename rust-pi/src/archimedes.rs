use crate::types::{ConvergencePoint, PiResult};
use crate::util::*;

fn needed_doublings(work: usize) -> u32 {
    ((work as f64) / 0.602).ceil() as u32 + 8
}

pub fn run(max_doublings: u32, precision: u32) -> PiResult {
    let t0 = now_ms();
    let work = precision as usize + GUARD;
    let mut u = parse_d("3", work);
    let mut v = parse_d("12", work).sqrt();
    let two = parse_d("2", work);
    let cap: u32 = if max_doublings == 0 {
        needed_doublings(work)
    } else {
        max_doublings
    };
    let threshold = parse_d(&format!("1e-{}", work.saturating_sub(6).max(1) as i32), work);
    let mut iterations: u32 = 0;
    for _ in 0..cap {
        let v_new = (&two * &u * &v) / (&u + &v);
        let u_new = (&u * &v_new).sqrt();
        u = u_new;
        v = v_new;
        iterations += 1;
        if (&v - &u) < threshold {
            break;
        }
    }
    let t1 = now_ms();
    make_result(
        "archimedes",
        &u,
        precision as usize,
        iterations as u64,
        t1 - t0,
        &format!(r#"{{"doublings":{},"sides":{}}}"#, iterations, 6u64 << iterations),
    )
}

pub fn convergence(max_doublings: u32, precision: u32) -> Vec<ConvergencePoint> {
    let t0 = now_ms();
    let work = precision as usize + GUARD;
    let mut u = parse_d("3", work);
    let mut v = parse_d("12", work).sqrt();
    let two = parse_d("2", work);
    let cap: u32 = if max_doublings == 0 {
        needed_doublings(work)
    } else {
        max_doublings
    };
    let threshold = parse_d(&format!("1e-{}", work.saturating_sub(6).max(1) as i32), work);
    let ref_str = to_pi_string(&pi_ref(work), work);
    let sample_every = (cap / 60).max(1);
    let mut points = Vec::new();
    for i in 1..=cap {
        let v_new = (&two * &u * &v) / (&u + &v);
        let u_new = (&u * &v_new).sqrt();
        u = u_new;
        v = v_new;
        if i % sample_every == 0 || i == cap {
            let computed = to_pi_string(&u, work);
            let correct = count_correct_decimal(&computed, &ref_str);
            points.push(ConvergencePoint {
                iteration: i as u64,
                correct_digits: correct,
                time_ms: now_ms() - t0,
            });
        }
        if (&v - &u) < threshold {
            if i % sample_every == 0 || points.is_empty() {
                let computed = to_pi_string(&u, work);
                let correct = count_correct_decimal(&computed, &ref_str);
                points.push(ConvergencePoint {
                    iteration: i as u64,
                    correct_digits: correct,
                    time_ms: now_ms() - t0,
                });
            }
            break;
        }
    }
    points
}