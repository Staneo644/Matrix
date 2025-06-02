use crate::fun::cross_product;
use crate::vector::Vector;

pub fn cross_product_test() {
    println!("\nCross Product Test");
    let u = Vector::from(&[0., 0., 1.]);
    let v = Vector::from(&[1., 0., 0.]);
    println!("Expected: [0.0, 1.0, 0.0]");
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from(&[1., 2., 3.]);
    let v = Vector::from(&[4., 5., 6.]);
    println!("Expected: [-3.0, 6.0, -3.0]");
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from(&[4., 2., -3.]);
    let v = Vector::from(&[-2., -5., 16.]);
    println!("Expected: [-17.0, -58.0, -16.0]");
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}
