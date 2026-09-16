use crate::types::{ConvergencePoint, PiResult};
use crate::util::*;
use dashu::float::DBig;

fn required_terms(work: usize) -> u32 {
    ((work as f64) / 8.0).ceil() as u32 + 2
}

fn c_k(k: usize) -> i128 {
    1103 + 26390 * k as i128
}

fn ratio_num(k: usize) -> i128 {
    let n = k as i128;
    (4 * n) * (4 * n - 1) * (4 * n - 2) * (4 * n - 3)
}

fn ratio_den(k: usize) -> i128 {
    let n = k as i128;
    n * n * n * n
}

fn iterate(terms: u32, work: usize) -> (DBig, u32) {
    let p4 = parse_d("24591257856", work);
    let lead = (parse_d("9801", work) / (parse_d("2", work) * parse_d("2", work).sqrt())).with_precision(work).value();
    let mut t = parse_d("1", work);
    let mut p = parse_d("1", work);
    let mut sum = parse_d("0", work);
    let mut used = 0u32;
for k in 0..terms as usize {
        let term = ((&t * DBig::from(c_k(k))) / &p).with_precision(work).value();
        sum = (term + &sum).with_precision(work).value();
        used = (k + 1) as u32;
        if k + 1 < terms as usize {
            let num = DBig::from(ratio_num(k + 1));
            let den = DBig::from(ratio_den(k + 1));
            t = ((&t * &num) / &den).with_precision(work).value();
            p = (&p * &p4).with_precision(work).value();
        }
    }
    ((&lead / &sum).with_precision(work).value(), used)
}

pub fn run(terms: u32, precision: u32) -> PiResult {
    let t0 = now_ms();
    let work = precision as usize + GUARD;
    let n = if terms == 0 { required_terms(work) } else { terms.min(100000) };
    let (pi, used) = iterate(n, work);
    let t1 = now_ms();
    make_result(
        "ramanujan",
        &pi,
        precision as usize,
        used as u64,
        t1 - t0,
        &format!(r#"{{"terms":{}}}"#, used),
    )
}

pub fn convergence(terms: u32, precision: u32) -> Vec<ConvergencePoint> {
    let t0 = now_ms();
    let work = precision as usize + GUARD;
    let n = if terms == 0 { required_terms(work) } else { terms.min(100000) };
    let ref_str = to_pi_string(&pi_ref(work), work);
    let p4 = parse_d("24591257856", work);
    let lead = (parse_d("9801", work) / (parse_d("2", work) * parse_d("2", work).sqrt())).with_precision(work).value();
    let mut t = parse_d("1", work);
    let mut p = parse_d("1", work);
    let mut sum = parse_d("0", work);
    let mut points = Vec::new();
for k in 0..n as usize {
        let term = ((&t * DBig::from(c_k(k))) / &p).with_precision(work).value();
        sum = (term + &sum).with_precision(work).value();
        if k + 1 < n as usize {
            let num = DBig::from(ratio_num(k + 1));
            let den = DBig::from(ratio_den(k + 1));
            t = ((&t * &num) / &den).with_precision(work).value();
            p = (&p * &p4).with_precision(work).value();
        }
        let pi = (&lead / &sum).with_precision(work).value();
        let computed = to_pi_string(&pi, work);
        let correct = count_correct_decimal(&computed, &ref_str);
        points.push(ConvergencePoint {
            iteration: (k + 1) as u64,
            correct_digits: correct,
            time_ms: now_ms() - t0,
        });
    }
    points
}