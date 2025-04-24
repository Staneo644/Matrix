use crate::matrix::Matrix;

impl<K: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<K, ROWS, COLS> {
    pub fn transpose(&self) -> Matrix<K, COLS, ROWS> {
        let mut transposed = Matrix::<K, COLS, ROWS>::new(vec![vec![K::default(); ROWS]; COLS]);
        for i in 0..ROWS {
            for j in 0..COLS {
                transposed[j][i] = self.data[i][j];
            }
        }
        transposed
    }
}
