use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PiResult {
    pub digits: String,
    pub time_ms: f64,
    pub iterations: u64,
    pub correct_digits: usize,
    pub algorithm: String,
    pub params: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConvergencePoint {
    pub iteration: u64,
    pub correct_digits: usize,
    pub time_ms: f64,
}