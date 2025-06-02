use crate::matrix::Matrix;
use num_traits::{Zero, One};

impl<
        K: Default
            + Copy
            + std::ops::Mul<Output = K>
            + std::ops::DivAssign
            + std::ops::MulAssign
            + std::ops::SubAssign
            + std::cmp::PartialEq
            + std::ops::Div<Output = K>
            + std::ops::Neg<Output = K>
            + Zero
            + One
            + std::fmt::Display,
        const ROWS: usize,
    > Matrix<K, ROWS, ROWS>
{
    pub fn inverse(&mut self) -> Result<Matrix<K, ROWS, ROWS>, ()> {
        let determinant = self.determinant();
        if determinant == K::zero() {
            return Err(());
        }
        let mut identity_matrix = Matrix::<K, ROWS, ROWS>::identity();
        let mut lead = 0;
        let mut row_echelon = self.clone();

        for r in 0..ROWS {
            if lead >= ROWS {
                return Err(());
            }

            let i = row_echelon.find_pivot(r, &mut lead);
            if i == ROWS {
                return Err(());
            }

            row_echelon.swap_rows(i, r);
            identity_matrix.swap_rows(i, r);

            identity_matrix.scale_row(r, row_echelon[r][lead]);
            row_echelon.scale_row(r, row_echelon[r][lead]);
            for i in 0..ROWS {
                if i != r {
                    identity_matrix.subtract_scaled_row(i, r, row_echelon[i][lead]);
                    row_echelon.subtract_scaled_row(i, r, row_echelon[i][lead]);
                }
            }
            lead += 1;
        }

        Ok(identity_matrix)
    }
}

impl<K: Zero + std::cmp::PartialEq, const ROWS: usize, const COLS: usize> Matrix<K, ROWS, COLS> {
    pub fn find_pivot(&self, r: usize, lead: &mut usize) -> usize {
        let mut i = r;
        while self[i][*lead] == K::zero() {
            i += 1;
            if i == ROWS {
                i = r;
                *lead += 1;
                if COLS == *lead {
                    return ROWS;
                }
            }
        }
        i
    }
}
