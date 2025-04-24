use crate::matrix::Matrix;

pub fn transpose_test() {
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.transpose());
    // 2.0
    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("{}", u.transpose());
    // 9.0
    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.]]);
    println!("{}", u.transpose());
    println!("{}", u.transpose().transpose());
    // -21.0
}
