// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const LOWEST_PROD_RATE: f64 = 221.00;

fn success_rate(speed: u8) -> Option<f64> {
    match (speed) {
        1..=4 => Some(1_f64),
        5..=8 => Some(0.9),
        9..=10 => Some(0.77),
        _ => None,
    }
}

fn ideal_prod_rate(speed: u8) -> f64 {
    speed as f64 * LOWEST_PROD_RATE
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match (success_rate(speed)) {
        Some(succ_rate) => succ_rate * ideal_prod_rate(speed),
        None => 0_f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60_f64) as u32
}
