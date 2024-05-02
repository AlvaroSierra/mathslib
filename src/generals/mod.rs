mod complex;
pub mod tensor;
pub mod traits;

pub fn deg2rad(val: f64) -> f64 {
    val * 2f64 * std::f64::consts::PI / 180f64
}

pub fn rad2deg(val: f64) -> f64 {
    val * 180f64 / (2f64 * std::f64::consts::PI)
}
