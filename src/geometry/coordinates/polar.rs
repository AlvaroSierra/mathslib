use std::cmp::Ordering;
use crate::generals::tensor::MathVec;
use crate::generals::traits::{Pow, Trig};

use super::cartesian::CartesianVelocity2D;

#[cfg(test)]
mod test {
    use crate::generals::tensor::MathVec;
    use approx::{assert_relative_eq, assert_abs_diff_eq};
    use std::f32;
    use crate::geometry::cartesian::CartesianCoordinates2D;


    use super::PolarCoordinates;


    #[test]
    fn test_coodinate_conversion() {
        let test_case = PolarCoordinates {
            magnitude: 1.5,
            amplitude: 0.5 * std::f32::consts::PI,
        };
        let transposed: MathVec<f32, 2> = test_case.into();
        let correct = MathVec::new([0f32, 1.5]);

        assert_relative_eq!(
            transposed.data()[0],
            correct.data()[0],
            epsilon = f32::EPSILON
        );
        assert_relative_eq!(
            transposed.data()[1],
            correct.data()[1],
            epsilon = f32::EPSILON
        );
    }

    #[test]
    fn test_coordinate_conversion_2() {
        let angle = std::f32::consts::PI * 0.25;
        let test_case = PolarCoordinates {
            magnitude: 1.5,
            amplitude: angle,            
        };

        let transposed: CartesianCoordinates2D<f32> = test_case.clone().into();
        let back_polar: PolarCoordinates<f32> = transposed.into();

        assert_abs_diff_eq!(transposed.data()[0], 1.060660172f32);
        assert_abs_diff_eq!(transposed.data()[1], 1.060660172f32);

        assert_relative_eq!(&test_case.magnitude, &back_polar.magnitude);
        assert_relative_eq!(&test_case.amplitude, &back_polar.amplitude);
    }

}

/// Although polar coordinates could be represented as a vector, doing so would mean we inherit
/// all the methods from it which would not work eg. adding a MathVec is not the same as adding
/// two polar coordinates.
///
/// Note: Amplitude assumed to always be in radians.
#[derive(Debug, Clone)]
pub struct PolarCoordinates<T> {
    pub magnitude: T,
    pub amplitude: T,
}

impl From<MathVec<f32, 2>> for PolarCoordinates<f32> {
    fn from(value: MathVec<f32, 2>) -> Self {

        if value.data()[0] == 0f32 {

            let magnitude = value.data()[1];
            return match magnitude.total_cmp(&0f32) {
                Ordering::Less => Self { magnitude, amplitude: std::f32::consts::PI * -0.5f32 },
                Ordering::Equal => Self { magnitude: 0f32, amplitude: 0f32},
                Ordering::Greater => Self { magnitude, amplitude: std::f32::consts::PI * 0.5f32}
            }
        }

        Self {
            // FIXME: Extraction of data needs to be directly indexed and comiled time check
            // for correct index
            magnitude: f32::powf(value.data()[0].pow(2) + value.data()[1].pow(2), 0.5),
            amplitude: f32::atan(value.data()[1] / value.data()[0]),
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

#[derive(Debug)]
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
            value.u_theta * cosine * point.magnitude + value.u_r * sine,
        ])
    }
}
