use crate::generals::tensor::MathVec;
use crate::generals::traits::Pow;

#[cfg(test)]
mod test {
    use crate::generals::tensor::MathVec;

    use crate::geometry::polar::PolarCoordinates;


    #[test]
    fn test_coodinate_conversion() {
        let test_case = PolarCoordinates {
                magnitude: 1.5,
                amplitude: 0.5 * std::f32::consts::PI,
            };
        let transposed: MathVec<f32, 2> = test_case.into();

        dbg!(&transposed);

        assert!(transposed == MathVec::new([0f32, 1.5]))
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
            amplitude: f32::tan(value.data()[1] / value.data()[0])
        }
    }
}

impl Into<MathVec<f32,2>> for PolarCoordinates<f32> {
   fn into(self) -> MathVec<f32,2> {
       MathVec::new([self.magnitude * self.amplitude.cos(), self.magnitude * self.amplitude.sin()])
   }
}
