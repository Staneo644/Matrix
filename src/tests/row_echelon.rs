use crate::matrix::Matrix;

pub fn row_echelon_test() {
    println!("\nRow Echelon Test");
    let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("Expected: \n[1.0, 0.0, 0.0]\n[0.0, 1.0, 0.0]\n[0.0, 0.0, 1.0]");
    println!("{}", u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("Expected: \n[1.0, 0.0]\n[0.0, 1.0]");
    println!("{}", u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let u = Matrix::from([[1., 2.], [2., 4.]]);
    println!("Expected: \n[1.0, 2.0]\n[0.0, 0.0]");
    println!("{}", u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    println!("Expected: \n[1.0, 0.625, 0.0, 0.0, -12.1666667]\n[0.0, 0.0, 1.0, 0.0, -3.6666667]\n[0.0, 0.0, 0.0, 1.0, 29.5]");
    println!("{}", u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}
