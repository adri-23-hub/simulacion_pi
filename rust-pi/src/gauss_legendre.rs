use crate::types::{ConvergencePoint, PiResult};
use crate::util::*;
use dashu::float::DBig;

fn auto_iters(work: usize) -> u32 {
    ((work as f64).log2().ceil() as u32) + 7
}

fn update(a: &DBig, b: &DBig, t: &DBig, p: &DBig, half: &DBig, work: usize) -> (DBig, DBig, DBig, DBig) {
    let a_next = (a + b) * half;
    let b_new = (a * b).sqrt();
    let d = a - &a_next;
    let t_new = (t - &(p * d.sqr())).with_precision(work).value();
    (a_next.with_precision(work).value(), b_new.with_precision(work).value(), t_new, (p * 2u32).with_precision(work).value())
}

fn approx(a: &DBig, b: &DBig, t: &DBig) -> DBig {
    ((a + b).sqr() / (t * 4u32)).with_precision(a.precision()).value()
}

fn iterate(iterations: u32, work: usize) -> (DBig, u32) {
    let mut a = parse_d("1", work);
    let mut b = parse_d("2", work).sqrt().inv();
    let mut t = parse_d("0.25", work);
    let mut p = parse_d("1", work);
    let half = parse_d("0.5", work);
    let mut iters = 0u32;
    for i in 0..iterations {
        let (na, nb, nt, np2) = update(&a, &b, &t, &p, &half, work);
        a = na;
        b = nb;
        t = nt;
        p = np2;
        iters = i + 1;
    }
    (approx(&a, &b, &t), iters)
}

pub fn run(iterations: u32, precision: u32) -> PiResult {
    let t0 = now_ms();
    let work = precision as usize + GUARD;
    let n = if iterations == 0 { auto_iters(work) } else { iterations.min(100) };
    let (pi, iters) = iterate(n, work);
    let t1 = now_ms();
    make_result(
        "gauss_legendre",
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
    let n = if iterations == 0 { auto_iters(work) } else { iterations.min(100) };
    let ref_str = to_pi_string(&pi_ref(work), work);
    let mut a = parse_d("1", work);
    let mut b = parse_d("2", work).sqrt().inv();
    let mut t = parse_d("0.25", work);
    let mut p = parse_d("1", work);
    let half = parse_d("0.5", work);
    let mut points = Vec::new();
    for i in 0..n {
        let (na, nb, nt, np2) = update(&a, &b, &t, &p, &half, work);
        a = na;
        b = nb;
        t = nt;
        p = np2;
        let pi = approx(&a, &b, &t);
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