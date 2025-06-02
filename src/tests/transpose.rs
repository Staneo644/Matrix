use crate::matrix::Matrix;

pub fn transpose_test() {
    println!("\nTranspose Test");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("Expected: [1 0][0 1]");
    println!("{}", u.transpose());
    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("Expected: [2.0, 4.0, -2.0][-5.0, 3.0, 3.0][0.0, 7.0, 4.0]");
    println!("{}", u.transpose());
    // 9.0
    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.]]);
    println!("Expected: [-2.0, 1.0], [-8.0, -23.0], [4.0, 4.0]");
    println!("{}", u.transpose());

    println!("Expected: [-2.0, -8.0, 4.0], [1.0, -23.0, 4.0]");
    println!("{}", u.transpose().transpose());
    // -21.0
}
