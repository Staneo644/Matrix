use crate::matrix::Matrix;

impl<K: Default + std::ops::AddAssign + Copy, const ROWS: usize> Matrix<K, ROWS, ROWS> {
    pub fn trace(&self) -> K {
        let mut sum = K::default();
        for i in 0..ROWS {
            sum += self.data[i][i];
        }
        sum
    }
}
