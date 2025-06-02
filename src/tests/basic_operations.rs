use crate::matrix::Matrix;
use crate::tests::Complex;
use crate::vector::Vector;

pub fn basic_operations_test() {
    println!("\nBasic Operations Test");
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.add(&v);
    println!("Expected: [7.0, 10.0]");
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from(&[2., 3.]);
    let v = Vector::from(&[5., 7.]);
    u.sub(&v);
    println!("Expected: [-3.0, -4.0]");
    println!("{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from(&[2., 3.]);
    u.scl(2.);
    println!("Expected: [4.0, 6.0]");
    println!("{}", u);
    // [4.0]
    // [6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);
    println!("Expected: [8.0, 6.0] [1.0, 6.0]");
    println!("{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    println!("Expected: [-6.0, -2.0] [5.0, 2.0]");
    println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("Expected: [2.0, 4.0] [6.0, 8.0]");
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]

    let c = Complex::new(3., 4.);
    let d = Complex::new(1., 2.);
    let e = Complex::new(2., 0.);
    let g = Complex::new(1., 1.);

    let mut f = Matrix::from([[c, d], [d, c]]);
    let mut h = Matrix::from([[g, g], [g, g]]);
    f.scl(e);
    println!("{}", f);

    h.add(&h.clone());
    println!("Expected: [2.0, 2.0], [2.0, 2.0]");
    println!("{}", h);
    // [2.0 + 2.0], [2.0 + 2.0]
        

    h.sub(&h.clone());
    println!("Expected: [0.0, 0.0], [0.0, 0.0]");
    println!("{}", h);
    // [0.0 + 0.0], [0.0 + 0.0]

}
