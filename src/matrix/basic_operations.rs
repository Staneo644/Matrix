use crate::matrix::Matrix;

impl<K: std::ops::Add<Output = K> + Copy, const ROWS: usize, const COLS: usize>
    Matrix<K, ROWS, COLS>
{
    pub fn add(&mut self, other: &Matrix<K, ROWS, COLS>) {
        *self += other;
    }
}

impl<K: std::ops::Add<Output = K> + Copy, const ROWS: usize, const COLS: usize> std::ops::Add
    for Matrix<K, ROWS, COLS>
{
    type Output = Matrix<K, ROWS, COLS>;

    fn add(self, other: Matrix<K, ROWS, COLS>) -> Matrix<K, ROWS, COLS> {
        let mut result = self.clone();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}

impl<K: std::ops::Add<Output = K> + Copy, const ROWS: usize, const COLS: usize>
    std::ops::AddAssign<&Matrix<K, ROWS, COLS>> for Matrix<K, ROWS, COLS>
{
    fn add_assign(&mut self, other: &Matrix<K, ROWS, COLS>) {
        let result = self.clone() + other.clone();
        *self = result;
    }
}

////////////////////////////////////

impl<K: std::ops::Sub<Output = K> + Copy, const ROWS: usize, const COLS: usize>
    Matrix<K, ROWS, COLS>
{
    pub fn sub(&mut self, other: &Matrix<K, ROWS, COLS>) {
        *self -= other;
    }
}

impl<K: std::ops::Sub<Output = K> + Copy, const ROWS: usize, const COLS: usize> std::ops::Sub
    for Matrix<K, ROWS, COLS>
{
    type Output = Matrix<K, ROWS, COLS>;

    fn sub(self, other: Matrix<K, ROWS, COLS>) -> Matrix<K, ROWS, COLS> {
        let mut result = self.clone();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }
}

impl<K: std::ops::Sub<Output = K> + Copy, const ROWS: usize, const COLS: usize>
    std::ops::SubAssign<&Matrix<K, ROWS, COLS>> for Matrix<K, ROWS, COLS>
{
    fn sub_assign(&mut self, other: &Matrix<K, ROWS, COLS>) {
        let result = self.clone() - other.clone();
        *self = result;
    }
}

///////////////////////////////////

impl<K: std::ops::Mul<Output = K> + Copy, const ROWS: usize, const COLS: usize>
    Matrix<K, ROWS, COLS>
{
    pub fn scl(&mut self, scalar: K) {
        *self *= scalar;
    }
}

impl<K: std::ops::Mul<Output = K> + Copy, const ROWS: usize, const COLS: usize> std::ops::Mul<K>
    for Matrix<K, ROWS, COLS>
{
    type Output = Matrix<K, ROWS, COLS>;

    fn mul(self, scalar: K) -> Matrix<K, ROWS, COLS> {
        let mut result = self.clone();
        for i in 0..ROWS {
            for j in 0..COLS {
                result.data[i][j] = self.data[i][j] * scalar;
            }
        }
        result
    }
}

impl<K: std::ops::Mul<Output = K> + Copy, const ROWS: usize, const COLS: usize>
    std::ops::MulAssign<K> for Matrix<K, ROWS, COLS>
{
    fn mul_assign(&mut self, scalar: K) {
        let result = self.clone() * scalar;
        *self = result;
    }
}
