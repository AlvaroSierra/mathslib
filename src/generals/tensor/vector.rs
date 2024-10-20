use crate::generals::traits::{Pow, Zero};
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

/// A mathematical representation of a vector
/// # TODO
/// - Change MathVec becomes
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct MathVec<T, const DIMS: usize> {
    data: [T; DIMS],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product() {
        let a = MathVec::new([1, 2, 3]);
        let b = MathVec::new([4, 5, 6]);

        assert_eq!(a.cross_product(b), MathVec::new([-3, 6, -3]));
    }
}

impl<T: Copy, const DIMS: usize> MathVec<T, DIMS> {
    pub fn add_dim(self, new_val: T) -> MathVec<T, { DIMS + 1 }> {
        let mut data = [new_val; DIMS + 1];

        for (inx, i) in self.data.into_iter().enumerate() {
            data[inx] = i
        }

        MathVec::new(data)
    }
}

impl<T: Zero + Copy, const DIMS: usize> Zero for MathVec<T, DIMS> {
    fn zero() -> Self {
        Self {
            data: [T::zero(); DIMS],
        }
    }
}

impl<T: Copy> MathVec<T, 3> {
    pub fn cross_product<P: Copy>(&self, rhs: MathVec<P, 3>) -> Self
    where
        T: Mul<P, Output = T> + Sub<T, Output = T>,
    {
        MathVec::new([
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        ])
    }
}

impl<T: Copy + Zero + AddAssign, const DIMS: usize> MathVec<T, DIMS> {
    pub fn dot_product<P: Copy>(&self, rhs: MathVec<P, DIMS>) -> T
    where
        T: Mul<P, Output = T>,
    {
        let mut total: T = T::zero();

        for m in 0..DIMS {
            total += self.data[m] * rhs.data[m]
        }

        total
    }
}

impl<T: Zero + Copy + AddAssign<T>, const DIMS: usize> Sum for MathVec<T, DIMS> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(MathVec::zero(), |a, b| a + b)
    }
}

impl<T, const DIMS: usize> MathVec<T, DIMS> {
    pub fn new(data: [T; DIMS]) -> Self {
        Self { data }
    }

    #[inline]
    pub fn data(&self) -> &[T; DIMS] {
        &self.data
    }
}

/// Trait for multiplying a scalar by a vector
///
/// To perform multiplication between vectors use cross_product or dot_product functions.
impl<T: Mul<P, Output = T> + Copy, const DIMS: usize, P: Copy> Mul<P> for MathVec<T, DIMS> {
    type Output = Self;

    fn mul(mut self, rhs: P) -> Self::Output {
        for m in 0..DIMS {
            self.data[m] = self.data[m] * rhs
        }
        self
    }
}

impl<T: Div<P, Output = T> + Copy, const DIMS: usize, P: Copy> Div<P> for MathVec<T, DIMS> {
    type Output = MathVec<T, DIMS>;

    fn div(mut self, rhs: P) -> Self::Output {
        for m in 0..DIMS {
            self.data[m] = self.data[m] / rhs
        }
        self
    }
}

impl<T: AddAssign + Copy, const DIMS: usize> Add for MathVec<T, DIMS> {
    type Output = MathVec<T, DIMS>;

    fn add(mut self, rhs: MathVec<T, DIMS>) -> Self::Output {
        for m in 0..DIMS {
            // Indexing both vectors is safe because both vectors have the same number of dimensions
            self.data[m] += rhs.data[m]
        }
        self
    }
}

impl<T: SubAssign + Copy, const DIMS: usize> Sub for MathVec<T, DIMS> {
    type Output = MathVec<T, DIMS>;

    fn sub(mut self, rhs: MathVec<T, DIMS>) -> Self::Output {
        for m in 0..DIMS {
            // Indexing both vectors is safe because both vectors have the same number of dimensions
            self.data[m] -= rhs.data[m]
        }
        self
    }
}

impl<
        T: Div<Output = T> + Copy + Zero + Pow<i8, T> + AddAssign + Into<f32> + From<f32>,
        const DIMS: usize,
    > MathVecTrait<T, DIMS> for MathVec<T, DIMS>
{
    fn unit_vector(&self) -> [T; DIMS] {
        let mag = self.magnitude();
        self.data.map(|x| x / mag)
    }

    /// Magnitude will always be higher that the elements
    fn magnitude(&self) -> T {
        let mut total: T = T::zero();
        for i in self.data.map(|x| x.pow(2)) {
            total += i
        }

        total.into().sqrt().into() // TODO: Make sqrt a trait
    }
}

impl<T, const DIMS: usize> From<[T; DIMS]> for MathVec<T, DIMS> {
    fn from(value: [T; DIMS]) -> Self {
        Self::new(value)
    }
}

impl<T, const DIMS: usize> From<MathVec<T, DIMS>> for [T; DIMS] {
    fn from(value: MathVec<T, DIMS>) -> Self {
        value.data
    }
}

pub trait MathVecTrait<T, const DIMS: usize> {
    fn unit_vector(&self) -> [T; DIMS];

    #[doc(alias = "abs")]
    fn magnitude(&self) -> T;
}

impl<T> From<MathVec<T, 1>> for (T,) {
    fn from(value: MathVec<T, 1>) -> Self {
        return value.data.into();
    }
}

impl<T> From<MathVec<T, 2>> for (T, T) {
    fn from(value: MathVec<T, 2>) -> Self {
        return value.data.into();
    }
}

impl<T> From<MathVec<T, 3>> for (T, T, T) {
    fn from(value: MathVec<T, 3>) -> Self {
        return value.data.into();
    }
}
