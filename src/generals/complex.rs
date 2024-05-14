use super::traits::Number;
use crate::generals::tensor::MathVec;
use crate::IMAGINARY_NUMBER_LETTER;
use std::fmt::{Display, Formatter};

#[cfg(test)]
mod test {}

/// A complex number with a real and imaginary part
///
///
///
/// The complex number of type T will include 2 values of type T, one for each element
type ComplexNumber<T> = MathVec<T, 2>;
impl<T: Display> Display for ComplexNumber<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} + {}{}",
            self.data()[0],
            self.data()[1],
            IMAGINARY_NUMBER_LETTER
        ))
    }
}

impl<T: Number> Number for ComplexNumber<T> {}
