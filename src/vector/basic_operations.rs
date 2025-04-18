use crate::vector::Vector;

impl<K: std::ops::Add<Output = K> + Copy> Vector<K> {
    pub fn add(&mut self, other: &Vector<K>) {
        *self += other;
    }
}

impl<K: std::ops::Add<Output = K> + Copy> std::ops::Add for Vector<K> {
    type Output = Vector<K>;

    fn add(self, other: Vector<K>) -> Vector<K> {
        let mut result = self.clone();
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }
}

impl<K: std::ops::Add<Output = K> + Copy> std::ops::AddAssign<&Vector<K>> for Vector<K> {
    fn add_assign(&mut self, other: &Vector<K>) {
        let result = self.clone() + other.clone();
        *self = result;
    }
}

////////////////////////////////////

impl<K: std::ops::Sub<Output = K> + Copy> Vector<K> {
    pub fn sub(&mut self, other: &Vector<K>) {
        *self -= other;
    }
}

impl<K: std::ops::Sub<Output = K> + Copy> std::ops::Sub for Vector<K> {
    type Output = Vector<K>;

    fn sub(self, other: Vector<K>) -> Vector<K> {
        let mut result = self.clone();
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}

impl<K: std::ops::Sub<Output = K> + Copy> std::ops::SubAssign<&Vector<K>> for Vector<K> {
    fn sub_assign(&mut self, other: &Vector<K>) {
        let result = self.clone() - other.clone();
        *self = result;
    }
}

////////////////////////////////////

impl<K: std::ops::Mul<Output = K> + Copy> Vector<K> {
    pub fn scl(&mut self, scalar: K) {
        *self *= scalar;
    }
}

impl<K: std::ops::Mul<Output = K> + Copy> std::ops::Mul<K> for Vector<K> {
    type Output = Vector<K>;

    fn mul(self, scalar: K) -> Vector<K> {
        let mut result = self.clone();
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] * scalar;
        }
        result
    }
}

impl<K: std::ops::Mul<Output = K> + Copy> std::ops::MulAssign<K> for Vector<K> {
    fn mul_assign(&mut self, scalar: K) {
        let result = self.clone() * scalar;
        *self = result;
    }
}
