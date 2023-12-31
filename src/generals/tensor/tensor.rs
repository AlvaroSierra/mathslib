use std::fmt::{Display, Formatter};
use std::ops::Range;
use thiserror::Error;
use crate::generals::number::Number;

/// Error enum for tensors generated from vectors
#[derive(Error, Debug)]
pub enum InitTensorFromVecError {
    #[error("Incorrect vector size for the specified number of dimensions")]
    IncorrectVectorSize
}


pub struct Tensor<T, const SIZE: usize, const RANK: usize> {
    data: [T; SIZE],
    dimensions: [usize; RANK]
}

impl<T: Number, const RANK: usize, const SIZE: usize> Tensor<T, SIZE, RANK> {

    pub fn storage_size(&self) -> usize {
        Self::storage_size2(self.dimensions)
    }

    fn storage_size2(dimensions: [usize; RANK]) -> usize {
        let mut final_result: usize = 1;

        for i in dimensions.iter() {
            final_result *= i
        }

        final_result
    }

    pub fn from_vec(data: Vec<T>, dimensions: [usize; RANK]) -> Result<Self, InitTensorFromVecError> {
        let size = Self::storage_size2(dimensions);

        if data.len() != size {
            return Err(InitTensorFromVecError::IncorrectVectorSize)
        }

        match data.try_into() {
            Ok(data) => Ok(Self {data, dimensions}),
            Err(_) => return Err(InitTensorFromVecError::IncorrectVectorSize)
        }
    }
}

struct TensorIterator<T, const SIZE: usize, const RANK: usize>{
    tensor: Tensor<T, SIZE, RANK>,
    alive: Range<usize>
}

enum TensorIters<T, const SIZE: usize, const RANK: usize>{
    Tensor(TensorIterator<T, SIZE, RANK>),
    Vals([T; SIZE])
}



// enum TensorIter<T, const SIZE: usize, const RESULTING_RANK: usize> {
//     Tensor(Tensor<T, SIZE, RESULTING_RANK>),
//     Val(T)
// }

// impl<T, const SIZE: usize, const RANK: usize, const NEWSIZE: usize> Iterator for TensorIterator<T, SIZE, RANK> {
//     type Item = Tensor<T, NEWSIZE, { RANK - 1 }>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.tensor.dimensions.len() > 1 {
//             // Return a concatenated tensor
//         }
//
//         // Else return the final data
//
//
//
//         self.tensor.data[]
//     }
// }
//
// impl<T, const SIZE: usize, const RANK: usize> IntoIterator for Tensor<T, SIZE, RANK>  {
//     type Item = TensorIter<T, SIZE, { RANK - 1}>;
//     type IntoIter = TensorIterator<T, SIZE, RANK>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         // TODO: Consider here if we return a Tensor iterator or return the final array (which is in turn converted into an iterator
//         Self::IntoIter {
//             tensor: &self,
//             alive: Range { start: 0, end: *&self.dimensions[0]} // FIXME: Are we supposed to move this number, clone or reference
//         }
//     }
// }
//
//
// impl<T: Display, const SIZE: usize, const RANK: usize> Display for Tensor<T, SIZE, RANK> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         for i in self {
//
//         }
//     }
// }


trait TensorTrait {
    fn hyperdeterminant(&self);
}
