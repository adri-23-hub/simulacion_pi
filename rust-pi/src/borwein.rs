use crate::types::{ConvergencePoint, PiResult};
use crate::util::*;
use dashu::float::DBig;

fn auto_iters(work: usize) -> u32 {
    ((work as f64).log2() / 2.0).ceil() as u32 + 3
}

fn iterate(iterations: u32, work: usize) -> (DBig, u32) {
    let sqrt2 = parse_d("2", work).sqrt();
    let mut y = &sqrt2 - parse_d("1", work);
    let mut a = parse_d("6", work) - &parse_d("4", work) * &sqrt2;
    let one = parse_d("1", work);
    let mut iters = 0u32;
    for i in 0..iterations {
        let r = (one.clone() - y.sqr().sqr()).sqrt().sqrt();
        y = (&one - &r) / (&one + &r);
        let sy = &one + &y;
        let inner = &one + &y + y.sqr();
        let shift = 2u64.pow(2 * i + 3);
        let subtract = (&y * &inner) * shift;
        a = (&a * sy.sqr().sqr()) - &subtract;
        iters = i + 1;
    }
    (a.with_precision(work).value().inv(), iters)
}

pub fn run(iterations: u32, precision: u32) -> PiResult {
    let t0 = now_ms();
    let work = precision as usize + GUARD;
    let n = if iterations == 0 { auto_iters(work) } else { iterations.min(40) };
    let (pi, iters) = iterate(n, work);
    let t1 = now_ms();
    make_result(
        "borwein",
        &pi,
        precision as usize,
        iters as u64,
        t1 - t0,
        &format!(r#"{{"iterations":{}}}"#, iters),
    )
}

pub fn convergence(iterations: u32, precision: u32) -> Vec<ConvergencePoint> {
    let t0 = now_ms();
    let work = precision as usize + GUARD;
    let n = if iterations == 0 { auto_iters(work) } else { iterations.min(40) };
    let ref_str = to_pi_string(&pi_ref(work), work);
    let sqrt2 = parse_d("2", work).sqrt();
    let mut y = &sqrt2 - parse_d("1", work);
    let mut a = parse_d("6", work) - &parse_d("4", work) * &sqrt2;
    let one = parse_d("1", work);
    let mut points = Vec::new();
    for i in 0..n {
        let r = (one.clone() - y.sqr().sqr()).sqrt().sqrt();
        y = (&one - &r) / (&one + &r);
        let sy = &one + &y;
        let inner = &one + &y + y.sqr();
        let shift = 2u64.pow(2 * i + 3);
        let subtract = (&y * &inner) * shift;
        a = (&a * sy.sqr().sqr()) - &subtract;
        let pi = a.clone().with_precision(work).value().inv();
        let computed = to_pi_string(&pi, work);
        let correct = count_correct_decimal(&computed, &ref_str);
        points.push(ConvergencePoint {
            iteration: (i + 1) as u64,
            correct_digits: correct,
            time_ms: now_ms() - t0,
        });
    }
    points
}