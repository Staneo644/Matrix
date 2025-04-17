use crate::vector::Vector;
use num_traits::Signed;
use num_traits::real::Real;

impl <K: Signed + std::ops::Add<Output = K>> Vector<K> {
    pub fn norm_1(&self) -> K {
        let mut sum = K::zero();
        for i in 0..self.data.len() {
            sum = sum + self.data[i].abs();
        }
        sum
    }
}

impl <K: Signed + std::ops::Add<Output = K> + std::ops::Mul<Output = K> + Real> Vector<K> {
    pub fn norm(&self) -> K {
        let mut sum = K::zero();
        for i in 0..self.data.len() {
            sum = sum + self.data[i].abs() * self.data[i].abs();
        }
        sum.sqrt()
    }
}

impl <K: Signed + std::ops::Add<Output = K> + std::ops::Mul<Output = K> + std::cmp::PartialOrd> Vector<K> {
    pub fn norm_inf(&self) -> K {
        let mut max = K::zero();
        for i in 0..self.data.len() {
            if self.data[i].abs() > max {
                max = self.data[i].abs();
            }
        }
        max
    }
}