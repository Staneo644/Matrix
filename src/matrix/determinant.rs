use crate::matrix::Matrix;
use num_traits::Zero;
use num_traits::One;

impl<
        K: Clone
            + std::ops::Mul<Output = K>
            + std::ops::DivAssign
            + std::ops::SubAssign
            + std::ops::Div<Output = K>
            + Default
            + std::cmp::PartialEq
            + Copy
            + std::ops::MulAssign
            + std::fmt::Display
            + Zero + One + std::ops::Neg<Output = K>,
        const ROWS: usize,
    > Matrix<K, ROWS, ROWS>
{
    pub fn determinant(&self) -> K {
        let mut matrix = self.clone();
        let mut swap_count = 0;

        for i in 0..ROWS {
            if matrix[i][i] == K::zero() {
                let mut found = false;
                for j in (i + 1)..ROWS {
                    if matrix[j][i] != K::zero() {
                        matrix.swap_rows(i, j);
                        swap_count += 1;
                        found = true;
                        break;
                    }
                }
                if !found {
                    return K::zero();
                }
            }

            for j in (i + 1)..ROWS {
                let e = matrix[j][i];
                let d = matrix[i][i];
                let factor = e / d;
                matrix.subtract_scaled_row(j, i, factor);
            }
        }

        let mut det = K::one();
        for i in 0..ROWS {
            det *= matrix[i][i];
        }
        if swap_count % 2 == 1 {
            det = -det;
        }
        det
    }
}
