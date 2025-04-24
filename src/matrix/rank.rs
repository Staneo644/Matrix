use crate::matrix::Matrix;
use num_traits::Signed;

impl<
        K: std::fmt::Display
            + PartialEq
            + Signed
            + Copy
            + std::ops::Add<Output = K>
            + std::ops::Mul<Output = K>
            + std::ops::DivAssign
            + std::ops::MulAssign
            + std::ops::SubAssign
            + Default,
        const ROWS: usize,
        const COLS: usize,
    > Matrix<K, ROWS, COLS>
{
    pub fn rank(&self) -> usize {
        let row_echelon = self.row_echelon();

        let mut rank = 0;
        for i in 0..ROWS {
            for j in 0..COLS {
                if row_echelon[i][j] != K::default() {
                    rank += 1;
                    break;
                }
            }
        }
        rank
    }
}
