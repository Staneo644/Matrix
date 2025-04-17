use crate::vector::Vector;

impl<K: std::ops::Mul<Output = K> + Copy + Default + std::ops::AddAssign> Vector<K> {
    pub fn dot(&self, v: &Vector<K>) -> K {
        if self.data.len() != v.data.len() {
            panic!("dot_product: vectors must be of the same length");
        }
        let mut result = K::default();
        for i in 0..self.data.len() {
            result += self.data[i] * v.data[i];
        }
        result
    }
}