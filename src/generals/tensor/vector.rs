use std::ops::{ Div, Mul};

/// A mathematical representation of a vector
/// # TODO
/// - Change MathVec becomes
pub struct MathVec<T, const DIMS: usize> {
    data: [T; DIMS]
}

impl<T: Copy, const DIMS: usize> MathVec<T, DIMS> {
    pub fn cross_product<P: Copy>(mut self, rhs: MathVec<P, DIMS>) -> Self where T: Mul<P, Output=T> {
        // TODO: Move this function to a Trait, the cross product also applies for matrices (although signature might not be the same therefore kept here for )
        for m in 0..DIMS {
            self.data[m] = self.data[m] * rhs.data[m]
        }

        self
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

// impl<T, const DIMS: usize> FromIterator<[T; DIMS]> for MathVec<T, DIMS>  where [T; DIMS]: FromIterator<T>{
//     fn from_iter<P: IntoIterator<Item=[T; DIMS]>>(iter: P) -> Self{
//         Self {
//             data: iter.into_iter().collect(),
//         }
//     }
// }

impl<T: Div<Output=T> + Copy, const DIMS: usize> MathVecTrait<T, DIMS> for MathVec<T, DIMS> {
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
