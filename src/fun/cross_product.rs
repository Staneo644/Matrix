use crate::vector::Vector;

pub fn cross_product<K: std::ops::Mul<Output = K> + std::ops::Sub<Output = K> + Copy>(u: &Vector::<K>, v: &Vector::<K>) -> Vector::<K> {
    if u.len() != 3 || v.len() != 3 {
        panic!("Cross product is only defined for 3D vectors");
    }

    let x = u[1] * v[2] - u[2] * v[1];
    let y = u[2] * v[0] - u[0] * v[2];
    let z = u[0] * v[1] - u[1] * v[0];

    Vector::from(&[x, y, z])
}