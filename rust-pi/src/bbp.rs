use crate::types::PiResult;
use crate::util::*;
use dashu::float::DBig;

fn hex_char(d: u32) -> char {
    if d < 10 {
        (b'0' + d as u8) as char
    } else {
        (b'A' + (d - 10) as u8) as char
    }
}

fn hex_of_pi(n_hex: usize) -> String {
    let work = (n_hex as f64 * 1.2041).ceil() as usize + 14;
    let pi = DBig::pi(work);
    let sixteen = parse_d("16", work);
    let mut x = pi.fract().with_precision(work).value();
    let mut out = String::with_capacity(n_hex);
    for _ in 0..n_hex {
        let y = &x * &sixteen;
        let d = y.trunc().to_f64().value();
        out.push(hex_char(d.clamp(0.0, 15.0) as u32));
        x = y.fract().with_precision(work).value();
    }
    out
}

fn mod_pow16(exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let base = 16u64 % m;
    if base == 0 {
        return if exp == 0 { 1 } else { 0 };
    }
    let mut e = exp;
    let mut b = base;
    let mut res = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            res = (res * b) % m;
        }
        b = (b * b) % m;
        e >>= 1;
    }
    res
}

fn bbp_hex_digit(p: u64) -> u32 {
    let mut frac = 0.0f64;
    for (s, j) in [(4i64, 1u64), (-2i64, 4u64), (-1i64, 5u64), (-1i64, 6u64)] {
        let mut sp = 0.0f64;
        let mut k = 0u64;
        loop {
            let den = 8 * k + j;
            if k <= p {
                let pk = mod_pow16(p - k, den);
                sp += (s as f64) * (pk as f64) / (den as f64);
            } else {
                let ds = (k - p) as f64;
                if ds > 20.0 {
                    break;
                }
                sp += (s as f64) / ((den as f64) * 16f64.powf(ds));
            }
            k += 1;
        }
        sp -= sp.floor();
        frac += sp;
        frac -= frac.floor();
    }
    let d = (frac * 16.0).floor().clamp(0.0, 15.0);
    d as u32
}

pub fn run(digits: u32, precision: u32) -> PiResult {
    let t0 = now_ms();
    let n = (digits.max(1) as usize).min(20000);
    let mut hex = String::with_capacity(n);
    for p in 0..n {
        hex.push(hex_char(bbp_hex_digit(p as u64)));
    }
    let ref_hex = hex_of_pi(n);
    let correct = count_correct(&hex, &ref_hex);
    let t1 = now_ms();
    let _ = precision;
    PiResult {
        digits: format!("3.{hex}"),
        time_ms: t1 - t0,
        iterations: n as u64,
        correct_digits: correct,
        algorithm: "bbp".to_string(),
        params: format!(r#"{{"hex_digits":{}}}"#, n),
    }
}