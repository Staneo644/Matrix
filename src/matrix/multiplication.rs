use crate::vector::Vector;
use crate::matrix::Matrix;

impl<K: std::ops::Mul<Output = K> + Default + std::ops::AddAssign + Copy, 
    const ROWS: usize, const COLS: usize
> Matrix::<K, ROWS, COLS> {
    pub fn mul_vec(&self, vec: Vector::<K>) -> Vector::<K> {
        let mut result = Vec::new();
        for i in 0..ROWS {
            let mut sum = K::default();
            for j in 0..COLS {
                sum += self.data[i][j] * vec.data[j];
            }
            result.push(sum);
        }
        Vector::new(result)
    }

    pub fn mul_mat(&self, mat: Matrix::<K, ROWS, COLS>) -> Matrix::<K, ROWS, COLS> {
        let mut result = Vec::new();
        for i in 0..self.data.len() {
            let mut row = Vec::new();
            for j in 0..mat.data[0].len() {
                let mut sum = K::default();
                for k in 0..self.data[i].len() {
                    sum += self[i][k] * mat[k][j];
                }
                row.push(sum);
            }
            result.push(row);
        }
        Matrix::<K, ROWS, COLS>::new(result)
    }
}