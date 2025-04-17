use crate::matrix::Matrix;

impl <K: std::ops::Add<Output = K> + Copy> Matrix<K> {
    pub fn add(&mut self, other: &Matrix<K>) {
        *self += other;
    }
}

impl<K: std::ops::Add<Output = K> + Copy> std::ops::Add for Matrix<K> {
    type Output = Matrix<K>;

    fn add(self, other: Matrix<K>) -> Matrix<K> {
        let mut result = self.clone();
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                result.data[i][j] = self.data[i][j] + other.data[i][j];    
            }
        }
        result
    }
}

impl<K: std::ops::Add<Output = K> + Copy> std::ops::AddAssign<&Matrix<K>> for Matrix<K> {
    fn add_assign(&mut self, other: &Matrix<K>) {
        let result = self.clone() + other.clone();
        *self = result;
    }
}

////////////////////////////////////

impl <K: std::ops::Sub<Output = K> + Copy> Matrix<K> {
    pub fn sub(&mut self, other: &Matrix<K>) {
        *self -= other;
    }
}

impl<K: std::ops::Sub<Output = K> + Copy> std::ops::Sub for Matrix<K> {
    type Output = Matrix<K>;

    fn sub(self, other: Matrix<K>) -> Matrix<K> {
        let mut result = self.clone();
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                result.data[i][j] = self.data[i][j] - other.data[i][j];    
            }
        }
        result
    }
}

impl<K: std::ops::Sub<Output = K> + Copy> std::ops::SubAssign<&Matrix<K>> for Matrix<K> {
    fn sub_assign(&mut self, other: &Matrix<K>) {
        let result = self.clone() - other.clone();
        *self = result;
    }
}

///////////////////////////////////

impl <K: std::ops::Mul<Output = K> + Copy> Matrix<K> {
    pub fn scl(&mut self, scalar: K) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = self.data[i][j] * scalar;
            }
        }
    }
}

impl<K: std::ops::Mul<Output = K> + Copy> std::ops::Mul<K> for Matrix<K> {
    type Output = Matrix<K>;

    fn mul(self, scalar: K) -> Matrix<K> {
        let mut result = self.clone();
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                result.data[i][j] = self.data[i][j] * scalar;
            }
        }
        result
    }
}

impl<K: std::ops::Mul<Output = K> + Copy> std::ops::MulAssign<K> for Matrix<K> {
    fn mul_assign(&mut self, scalar: K) {
        let result = self.clone() * scalar;
        *self = result;
    }
}