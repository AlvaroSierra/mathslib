use crate::generals::tensor::MathVec;
use crate::generals::traits::{Pow, Trig};

use super::cartesian::CartesianVelocity2D;

#[cfg(test)]
mod test {
    use crate::generals::tensor::MathVec;
    use approx::relative_eq;
    use std::f32;

    use crate::geometry::polar::PolarCoordinates;

    #[test]
    fn test_coodinate_conversion() {
        let test_case = PolarCoordinates {
            magnitude: 1.5,
            amplitude: 0.5 * std::f32::consts::PI,
        };
        let transposed: MathVec<f32, 2> = test_case.into();
        let correct = MathVec::new([0f32, 1.5]);

        relative_eq!(
            transposed.data()[0],
            correct.data()[0],
            epsilon = f32::EPSILON
        );
        relative_eq!(
            transposed.data()[1],
            correct.data()[1],
            epsilon = f32::EPSILON
        );
    }
}

/// Although polar coordinates could be represented as a vector, doing so would mean we inherit
/// all the methods from it which would not work eg. adding a MathVec is not the same as adding
/// two polar coordinates.
///
/// Note: Amplitude assumed to always be in radians.
pub struct PolarCoordinates<T> {
    pub magnitude: T,
    pub amplitude: T,
}

impl From<MathVec<f32, 2>> for PolarCoordinates<f32> {
    fn from(value: MathVec<f32, 2>) -> Self {
        Self {
            // FIXME: Extraction of data needs to be directly indexed and comiled time check
            // for correct index
            magnitude: f32::powf(value.data()[0].pow(2) + value.data()[1].pow(2), 0.5),
            amplitude: f32::tan(value.data()[1] / value.data()[0]),
        }
    }
}

impl From<PolarCoordinates<f32>> for MathVec<f32, 2> {
    fn from(value: PolarCoordinates<f32>) -> Self {
        MathVec::new([
            value.magnitude * value.amplitude.cos(),
            value.magnitude * value.amplitude.sin(),
        ])
    }
}

pub struct PolarVelocity2D<T> {
    pub u_r: T,
    pub u_theta: T,
}

// TODO: Is this going to become too permisive, possibly allowing values which may result in undefined behaviour?
impl<T: Trig + std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + Copy>
    CartesianVelocity2D<T>
{
    pub fn from_polar(value: PolarVelocity2D<T>, point: impl Into<PolarCoordinates<T>>) -> Self {
        let point: PolarCoordinates<T> = point.into();

        let cosine = point.amplitude.cos();
        let sine = point.amplitude.sin();
        CartesianVelocity2D::new([
            value.u_r * cosine - point.magnitude * value.u_theta * sine,
            value.u_theta * cosine - point.magnitude * value.u_r * sine,
        ])
    }
}
