extern crate matrix;

use crate::matrix::tests::*;
use crate::matrix::Matrix;
use crate::matrix::Vector;

fn main() {
    // Create a new vector
    let mut vector1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let vector2 = Vector::from(&[4.0, 5.0, 6.0]);
    // Add the vectors
    vector1.add(&vector2);
    vector1.print();

    let matrix1 = Matrix::from([[1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [6.0, 7.0, 8.0]]);
    matrix1.print();

    let matrix2 = Matrix::from([[1.0, 2.0, 3.0]]);
    matrix2.print();

    let matrix3 = Matrix::from([[1.0, 2.0, 3.0], [3.0, 4.0, 5.0]]);
    matrix3.print();

    basic_operations();
}
