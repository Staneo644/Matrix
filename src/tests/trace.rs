use crate::matrix::Matrix;

pub fn trace_test() {
    println!("\nTrace Test");
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("Expected: 2.0");
    println!("{}", u.trace());
    // 2.0
    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("Expected: 9.0");
    println!("{}", u.trace());
    // 9.0
    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("Expected: -21.0");
    println!("{}", u.trace());
    // -21.0
}
