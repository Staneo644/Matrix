use crate::matrix::Matrix;
use crate::vector::Vector;

pub fn multiplication_test() {
    println!("\nMultiplication Test");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from(&[4., 2.]);
    println!("Expected: [4.0, 2.0]");
    println!("{}", u.mul_vec(v));
    // [4.]
    // [2.]
    let u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from(&[4., 2.]);
    println!("Expected: [8.0, 4.0]");
    println!("{}", u.mul_vec(v));
    // [8.]
    // [4.]
    let u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from(&[4., 2.]);
    println!("Expected: [4.0, -4.0]");
    println!("{}", u.mul_vec(v));
    // [4.]
    // [-4.]
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);
    println!("Expected: [1.0, 0.0; 0.0, 1.0]");
    println!("{}", u.mul_mat(v));
    // [1., 0.]
    // [0., 1.]
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("Expected: [2.0, 1.0; 4.0, 2.0]");
    println!("{}", u.mul_mat(v));
    // [2., 1.]
    // [4., 2.]
    let u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("Expected: [-14.0, -7.0; 44.0, 22.0]");
    println!("{}", u.mul_mat(v));
    // [-14., -7.]
    // [44., 22.]
}
