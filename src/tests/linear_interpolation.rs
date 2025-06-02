use crate::fun::lerp;
use crate::matrix::Matrix;
use crate::vector::Vector;

pub fn linear_interpolation_test() {
    println!("\nLinear Interpolation Test");
    println!("Expected: 0.0");
    println!("{}", lerp(0., 1., 0.));
    // 0.0
    println!("Expected: 1.0");
    println!("{}", lerp(0., 1., 1.));
    // 1.0
    println!("Expected: 0.5");
    println!("{}", lerp(0., 1., 0.5));
    // 0.5
    println!("Expected: 27.3");
    println!("{}", lerp(21., 42., 0.3));
    // 27.3
    println!("Expected: [2.6, 1.3]");
    println!(
        "{}",
        lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3)
    );
    // [2.6]
    // [1.3]
    println!("Expected: [[11.0, 5.5], [16.5, 22.0]]");
    println!(
        "{}",
        lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5
        )
    );
    // [[11., 5.5]
    // [16.5, 22.]]
}
