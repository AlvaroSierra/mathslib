use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

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

impl<T: Div<Output = T> + Copy, const DIMS: usize> MathVecTrait<T, DIMS> for MathVec<T, DIMS> {
    fn unit_vector(&self) -> [T; DIMS] {
        let mag = self.magnitude();
        self.data.map(|x| x / mag)
    }

    /// Magnitude will always be higher that the elements
    fn magnitude(&self) -> T {
        todo!()
        // self.data.map(|x| x.pow(2))
    }
}

impl<T, const DIMS: usize> From<[T; DIMS]> for MathVec<T, DIMS> {
    fn from(value: [T; DIMS]) -> Self {
        Self::new(value)
    }
}

impl<T, const DIMS: usize> Into<[T; DIMS]> for MathVec<T, DIMS> {
    fn into(self) -> [T; DIMS] {
        self.data
    }
}

pub trait MathVecTrait<T, const DIMS: usize> {
    fn unit_vector(&self) -> [T; DIMS];
    fn magnitude(&self) -> T;
}

impl<T> MathVec2Trait<T> for MathVec<T, 2> {
    fn amplitude(&self) -> T {
        todo!()
    }
}

pub trait MathVec2Trait<T> {
    fn amplitude(&self) -> T;
}
