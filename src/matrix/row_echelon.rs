use crate::matrix::Matrix;
use num_traits::Zero;

impl<
        K: std::ops::Mul<Output = K>
            // + std::ops::Div<Output = K>
            + Zero
            + std::ops::Div
            + std::ops::DivAssign
            + std::ops::MulAssign
            + std::ops::SubAssign
            + std::cmp::PartialEq
            + Copy,
        const ROWS: usize,
        const COLS: usize,
    > Matrix<K, ROWS, COLS>
{
    pub fn row_echelon(&self) -> Matrix<K, ROWS, COLS> {
        let mut row_echelon = self.clone();
        let mut lead = 0;

        for r in 0..ROWS {
            if lead >= COLS {
                return row_echelon;
            }
            let i = row_echelon.find_pivot(r, &mut lead);
            if i == ROWS {
                return row_echelon;
            }

            row_echelon.swap_rows(i, r);
            row_echelon.scale_row(r, row_echelon[r][lead]);
            for i in 0..ROWS {
                if i != r {
                    row_echelon.subtract_scaled_row(i, r, row_echelon[i][lead]);
                }
            }
            lead += 1;
        }
        row_echelon
    }

    pub fn swap_rows(&mut self, i: usize, j: usize) {
        for k in 0..COLS {
            let temp = self[i][k];
            self[i][k] = self[j][k];
            self[j][k] = temp;
        }
    }

    pub fn scale_row(&mut self, row: usize, scale: K) {
        for j in 0..COLS {
            self[row][j] /= scale;
        }
    }

    pub fn scale_row_reverse(&mut self, row: usize, scale: K) {
        for j in 0..COLS {
            self[row][j] *= scale;
        }
    }

    pub fn subtract_scaled_row(&mut self, target_row: usize, source_row: usize, scale: K) {
        for j in 0..COLS {
            let source_value = self[source_row][j];
            self[target_row][j] -= source_value * scale;
        }
    }
}
