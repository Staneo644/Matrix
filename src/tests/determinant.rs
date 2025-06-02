use crate::matrix::Matrix;
use crate::tests::Complex;

pub fn determinant_test() {
    println!("\nDeterminant Test");
    let u = Matrix::from([[1., -1.], [-1., 1.]]);
    println!("Expected: 0.0");
    println!("{}", u.determinant());
    // 0.0
    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("Expected: 8.0");
    println!("{}", u.determinant());
    // 8.0
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("Expected: -174.0");
    println!("{}", u.determinant());
    // -174.0
    let u = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    println!("Expected: 1032.0");
    println!("{}", u.determinant());
    // 1032

    
    let u = Matrix::from([
        [Complex::new(0.0, 1.0), Complex::new(0.0, 1.0)],
        [Complex::new(1.0, 0.0), Complex::new(2.0, 3.0)],
    ]);
    println!("Expected: -3.0 + 1i");
    println!("{}", u.determinant());
    // -3.0 + 1i
}
