use crate::fun::angle_cos;
use crate::vector::Vector;

pub fn cosine_test() {
    println!("\nCosine Test");
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[1., 0.]);
    println!("Expected: 1.0");
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[0., 1.]);
    println!("Expected: 0.0");
    println!("{}", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from(&[-1., 1.]);
    let v = Vector::from(&[1., -1.]);
    println!("Expected: -1.0");
    println!("{}", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from(&[2., 1.]);
    let v = Vector::from(&[4., 2.]);
    println!("Expected: 1.0");
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(&[1., 2., 3.]);
    let v = Vector::from(&[4., 5., 6.]);
    println!("Expected: 0.974631846");
    println!("{}", angle_cos(&u, &v));
    // 0.974631846
}
