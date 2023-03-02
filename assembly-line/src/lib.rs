#![allow(unused)]

const RATE_PRODUCT: f64 = 221.0;
const MINUTES_FROM_HOUR: f64 = 60.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (speed as f64) * RATE_PRODUCT * match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=u8::MAX => 0.77,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    //let cars: f64 = production_rate_per_hour(speed);
    // (cars / MINUTES_FROM_HOUR) as u32
    (production_rate_per_hour(speed) / MINUTES_FROM_HOUR) as u32
}
