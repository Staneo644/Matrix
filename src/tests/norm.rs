use crate::vector::Vector;

pub fn norm_test() {
    println!("\nNorm Test");
    let u = Vector::from(&[0., 0., 0.]);
    println!("Expected: 0.0, 0.0, 0.0");
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let u = Vector::from(&[1., 2., 3.]);
    println!("Expected: 6.0, 3.74165738, 3.0");
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let u = Vector::from(&[-1., -2.]);
    println!("Expected: 3.0, 2.236067977, 2.0");
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}
