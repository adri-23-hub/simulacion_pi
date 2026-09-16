use crate::types::{ConvergencePoint, PiResult};
use crate::util::*;
use dashu::float::DBig;

fn required_terms(work: usize) -> u32 {
    ((work as f64) / 14.1816474627).ceil() as u32 + 2
}

fn ct_eps(work: usize) -> DBig {
    parse_d(&format!("1e-{}", work as i32), work)
}

fn c_k(k: usize) -> i128 {
    13591409 + 545140134 * k as i128
}

fn ratio_num(k: usize) -> i128 {
    let n = k as i128;
    (6 * n) * (6 * n - 1) * (6 * n - 2) * (6 * n - 3) * (6 * n - 4) * (6 * n - 5)
}

fn ratio_den(k: usize) -> i128 {
    let n = k as i128;
    (3 * n) * (3 * n - 1) * (3 * n - 2) * n * n * n
}

fn iterate(terms: u32, work: usize) -> (DBig, u32) {
    let c_fact = parse_d("426880", work) * parse_d("10005", work).sqrt();
    let p4 = parse_d("262537412640768000", work);
    let mut t = parse_d("1", work);
    let mut p = parse_d("1", work);
    let mut sum = parse_d("0", work);
    let mut used = 0u32;
    for k in 0..terms as usize {
        let term = ((&t * DBig::from(c_k(k))) / &p).with_precision(work).value();
        let signed = if k % 2 == 1 { -term } else { term };
        sum = (signed + &sum).with_precision(work).value();
        used = (k + 1) as u32;
        if k + 1 < terms as usize {
            let num = DBig::from(ratio_num(k + 1));
            let den = DBig::from(ratio_den(k + 1));
            t = ((&t * &num) / &den).with_precision(work).value();
            p = (&p * &p4).with_precision(work).value();
        }
    }
    ((&c_fact / &sum).with_precision(work).value(), used)
}

pub fn run(terms: u32, precision: u32) -> PiResult {
    let t0 = now_ms();
    let work = precision as usize + GUARD;
    let n = if terms == 0 { required_terms(work) } else { terms.min(100000) };
    let (pi, used) = iterate(n, work);
    let t1 = now_ms();
    make_result(
        "chudnovsky",
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
    let c_fact = parse_d("426880", work) * parse_d("10005", work).sqrt();
    let p4 = parse_d("262537412640768000", work);
    let mut t = parse_d("1", work);
    let mut p = parse_d("1", work);
    let mut sum = parse_d("0", work);
    let mut points = Vec::new();
for k in 0..n as usize {
        let term = ((&t * DBig::from(c_k(k))) / &p).with_precision(work).value();
        let signed = if k % 2 == 1 { -term.clone() } else { term.clone() };
        sum = (signed + &sum).with_precision(work).value();
        if k + 1 < n as usize {
            let num = DBig::from(ratio_num(k + 1));
            let den = DBig::from(ratio_den(k + 1));
            t = ((&t * &num) / &den).with_precision(work).value();
            p = (&p * &p4).with_precision(work).value();
        }
        let pi = (&c_fact / &sum).with_precision(work).value();
        let computed = to_pi_string(&pi, work);
        let correct = count_correct_decimal(&computed, &ref_str);
        points.push(ConvergencePoint {
            iteration: (k + 1) as u64,
            correct_digits: correct,
            time_ms: now_ms() - t0,
        });
        if abs_lt(&term, &ct_eps(work)) {
            break;
        }
    }
    points
}