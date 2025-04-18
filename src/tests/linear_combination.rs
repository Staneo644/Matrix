use crate::fun::linear_combination;
use crate::vector::Vector;

pub fn linear_combination_test() {
    let e1 = Vector::from(&[1., 0., 0.]);
    let e2 = Vector::from(&[0., 1., 0.]);
    let e3 = Vector::from(&[0., 0., 1.]);
    let v1 = Vector::from(&[1., 2., 3.]);
    let v2 = Vector::from(&[0., 10., -100.]);
    println!(
        "{}",
        linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5])
    );
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination::<f32>(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]
}
