use num_traits::One;

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<K, const ROWS: usize = 0, const COLS: usize = 0> {
    pub data: [[K; COLS]; ROWS],
}

impl<K: Default + Copy, const ROWS: usize, const COLS: usize> Default for Matrix<K, ROWS, COLS> {
    fn default() -> Self {
        Matrix {
            data: [[K::default(); COLS]; ROWS],
        }
    }
}

impl<K: std::fmt::Display, const ROWS: usize, const COLS: usize> std::fmt::Display
    for Matrix<K, ROWS, COLS>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..ROWS {
            match (i, ROWS) {
                (_, 1) => write!(f, "[ ")?,
                (0, _) => write!(f, "┌ ")?,
                (x, len) if x == len - 1 && len > 1 => write!(f, "└ ")?,
                _ => write!(f, "│ ")?,
            }

            for j in 0..COLS {
                if j != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.data[i][j])?;
            }

            match (i, ROWS) {
                (_, 1) => write!(f, " ]")?,
                (i, len) if i == len - 1 && len > 1 => write!(f, " ┘")?,
                (0, _) => write!(f, " ┐")?,
                _ => write!(f, " │")?,
            }

            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<K, const ROWS: usize, const COLS: usize> From<[[K; COLS]; ROWS]> for Matrix<K, ROWS, COLS> {
    fn from(array: [[K; COLS]; ROWS]) -> Self {
        Matrix { data: array }
    }
}

impl<K: One + Default + Copy, const ROWS: usize> Matrix<K, ROWS, ROWS> {
    pub fn identity() -> Self {
        let mut identity = Matrix::default();
        for i in 0..ROWS {
            identity.data[i][i] = K::one();
        }
        identity
    }
}

impl<K: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<K, ROWS, COLS> {
    pub fn new(data: Vec<Vec<K>>) -> Self {
        Matrix {
            data: data
                .into_iter()
                .map(|row| {
                    let mut arr = [K::default(); COLS];
                    for (i, val) in row.into_iter().enumerate() {
                        arr[i] = val;
                    }
                    arr
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!("Invalid matrix dimensions")),
        }
    }

    pub fn is_square(&self) -> bool {
        self.data.len() == self.data[0].len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty() || self.data[0].is_empty()
    }
}

impl<K, const ROWS: usize, const COLS: usize> std::ops::Index<usize> for Matrix<K, ROWS, COLS> {
    type Output = [K; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K: Copy, const ROWS: usize, const COLS: usize> std::ops::IndexMut<usize>
    for Matrix<K, ROWS, COLS>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<K: std::fmt::Display, const ROWS: usize, const COLS: usize> Matrix<K, ROWS, COLS> {
    pub fn print(&self) {
        println!("{}", self);
    }
}
