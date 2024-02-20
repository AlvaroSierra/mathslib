pub mod polar {
    use crate::generals::{tensor::MathVec, traits::{Number, Pow}};

   
    /// Although polar coordinates could be represented as a vector, doing so would mean we inherit
    /// all the methods from it which would not work eg. adding a MathVec is not the same as adding
    /// two polar coordinates.
    ///
    /// Note: Amplitude assumed to always be in radians.
    pub struct PolarCoordinates<T> {
        magnitude: T,
        amplitude: T, 
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
}

pub mod cartesian {
    use crate::generals::tensor::MathVec;

    // In this library a vector is the same as a Cartesian coordinates point
    pub type CartesianCoordinates2D<T> = MathVec<T, 2>;
}

