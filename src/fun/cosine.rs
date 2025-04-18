use crate::vector::Vector;
use num_traits::Signed;
use num_traits::Float;

pub fn angle_cos<K: std::ops::Mul<Output = K> + Copy + Default + std::ops::AddAssign + Signed + Float>
(u: &Vector::<K>, v: &Vector::<K>) -> K {
    let dot_product = u.dot(v);

    if u.norm() == K::zero() || v.norm() == K::zero() {
        panic!("Cannot compute angle with a zero-magnitude vector");
    }

    let cos_theta = dot_product / (u.norm() * v.norm());
    cos_theta
}