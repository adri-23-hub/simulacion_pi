use crate::types::PiResult;
use dashu::float::DBig;

pub fn parse_d(s: &str, prec: usize) -> DBig {
    s.parse::<DBig>()
        .unwrap_or_else(|_| panic!("invalid literal: {s}"))
        .with_precision(prec.max(1))
        .value()
}

pub const GUARD: usize = 10;

pub fn now_ms() -> f64 {
    js_sys::Date::now()
}

pub fn pi_ref(prec: usize) -> DBig {
    DBig::pi(prec.max(1))
}

pub fn to_pi_string(pi: &DBig, digits: usize) -> String {
    pi.clone().with_precision(digits.max(1)).value().to_string()
}

pub fn count_correct_decimal(computed: &str, reference: &str) -> usize {
    let a: String = computed.chars().filter(|c| *c != '.').collect();
    let b: String = reference.chars().filter(|c| *c != '.').collect();
    count_correct(&a, &b)
}

pub fn count_correct(a: &str, b: &str) -> usize {
    let mut n = 0;
    for (x, y) in a.chars().zip(b.chars()) {
        if x == y {
            n += 1;
        } else {
            break;
        }
    }
    n
}

pub fn correct_vs_pi(pi: &DBig, requested: usize) -> usize {
    let ref_str = to_pi_string(&pi_ref(requested + GUARD), requested);
    let computed = to_pi_string(pi, requested);
    count_correct_decimal(&computed, &ref_str)
}

pub fn json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap()
}

pub fn make_result(
    algorithm: &str,
    pi: &DBig,
    requested: usize,
    iterations: u64,
    time_ms: f64,
    params: &str,
) -> PiResult {
    let digits = to_pi_string(pi, requested);
    let correct = correct_vs_pi(pi, requested);
    PiResult {
        digits,
        time_ms,
        iterations,
        correct_digits: correct,
        algorithm: algorithm.to_string(),
        params: params.to_string(),
    }
}

pub fn from_f64(v: f64) -> DBig {
    if v.is_finite() {
        parse_d(&v.to_string(), 17).with_precision(20).value()
    } else {
        DBig::pi(20)
    }
}

pub fn abs_lt(x: &DBig, eps: &DBig) -> bool {
    let ne = -eps.clone();
    x.clone() < eps.clone() && x.clone() > ne
}