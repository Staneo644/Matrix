use crate::matrix::Matrix;

impl <K: std::ops::Add<Output = K> + Copy> Matrix<K> {
    pub fn add(&mut self, other: &Matrix<K>) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = self.data[i][j] + other.data[i][j];    
            }
        }
    }
}

impl <K: std::ops::Sub<Output = K> + Copy> Matrix<K> {
    pub fn sub(&mut self, other: &Matrix<K>) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = self.data[i][j] - other.data[i][j];    
            }
        }
    }
}

impl <K: std::ops::Mul<Output = K> + Copy> Matrix<K> {
    pub fn scl(&mut self, scalar: K) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = self.data[i][j] * scalar;
            }
        }
    }
}
