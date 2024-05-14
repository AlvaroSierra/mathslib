use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use crate::generals::traits::{Pow, Zero,};


/// A mathematical representation of a vector
/// # TODO
/// - Change MathVec becomes
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct MathVec<T, const DIMS: usize> {
    data: [T; DIMS],
}

impl<T: Copy, const DIMS: usize> MathVec<T, DIMS> {
    pub fn cross_product<P: Copy>(mut self, rhs: MathVec<P, DIMS>) -> Self
    where
        T: Mul<P, Output = T>,
    {
        // TODO: Fix this, this is not the cross product XD
        // TODO: Move this function to a Trait, the cross product also applies for matrices (although signature might not be the same therefore kept here for )
        for m in 0..DIMS {
            self.data[m] = self.data[m] * rhs.data[m]
        }

        self
    }

    pub fn add_dim(self, new_val: T) -> MathVec<T, { DIMS + 1 }> {
        let mut data = [new_val; DIMS + 1];

        for (inx, i) in self.data.into_iter().enumerate() {
            data[inx] = i
        }

        MathVec::new(data)
    }
}

impl<T: Copy + Zero + AddAssign, const DIMS: usize> MathVec<T, DIMS> {

    pub fn dot_product<P: Copy>(&self, rhs: MathVec<P, DIMS>) -> T where T: Mul<P, Output = T> {
        let mut total: T = T::zero();
        
        for m in 0..DIMS {
            total += self.data[m] * rhs.data[m]
        }
        
        total
    }

}

impl<T, const DIMS: usize> MathVec<T, DIMS> {
    pub fn new(data: [T; DIMS]) -> Self {
        Self { data }
    }

    #[inline]
    pub(crate) fn data(&self) -> &[T; DIMS] {
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

impl<T: Div<Output = T> + Copy + Zero + Pow<i8, T> + AddAssign + Into<f32> + From<f32>, const DIMS: usize> MathVecTrait<T, DIMS> for MathVec<T, DIMS> {
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

    #[doc(alias="abs")]
    fn magnitude(&self) -> T;
}

