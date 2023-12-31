use super::tensor::Tensor;

type Matrix<T, const SIZE: usize> = Tensor<T, SIZE, 2>;

trait MatrixTrait {
    fn determinant(&self);
}

impl<T, const SIZE: usize> MatrixTrait for Matrix<T, SIZE> {
    fn determinant(&self) {

    }
}


trait Vector {

}

trait Scalar {

}