use crate::vector::Vector;

pub fn linear_combination<K: std::ops::Mul<Output = K> + std::ops::Add<Output = K> + Copy>
(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    if u.len() == 0 || coefs.len() == 0  || u.len() != coefs.len() {
        panic!("linear_combination: u is empty");
    }
    
    let mut result = u[0].clone();
    result.scl(coefs[0]);
    for i in 1..coefs.len() {
        result += &(u[i].clone() * coefs[i]);
    }
    result
}