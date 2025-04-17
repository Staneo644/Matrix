use crate::vector::Vector;

impl<K: std::ops::Add<Output = K> + Copy> Vector<K> {
    pub fn add(&mut self, other: &Vector<K>){
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] + other.data[i];    
        }
    }
}

impl <K: std::ops::Sub<Output = K> + Copy> Vector<K> {
    pub fn sub(&mut self, other: &Vector<K>){
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] - other.data[i];    
        }
    }
}

impl <K: std::ops::Mul<Output = K> + Copy> Vector<K> {
    pub fn scl(&mut self, scalar: K) {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] * scalar;
        }
    }
}
