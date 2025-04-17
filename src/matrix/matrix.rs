
#[derive(Clone, PartialEq, Eq, Default)]
pub struct Matrix<K> {
    pub data: Vec<Vec<K>>,
}

impl<K: std::fmt::Display> std::fmt::Display for Matrix<K>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.data.len() {
            match (i, self.data.len()) {
                (0, 1) => write!(f, "[ ")?,
                (0, _) => write!(f, "┌ ")?,
                (x, len) if x == len - 1 && len > 1 => write!(f, "└ ")?,
                _ => write!(f, "│ ")?,
            }

            for j in 0..self.data[i].len() {
                if j != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.data[i][j])?;
            }

            match (i, self.data.len()) {
                (0, 1) => write!(f, " ]")?,
                (x, len) if x == len - 1 && len > 1 => write!(f, " ┘")?,
                (0, _) => write!(f, " ┐")?,
                _ => write!(f, " │")?,
            }

            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Matrix<f32> {
    pub fn from(data: [f32; 4]) -> Matrix<f32> {
        Matrix {
            data: vec![
                vec![data[0], data[1]],
                vec![data[2], data[3]],
            ],
        }
    }
}

impl<K> Matrix<K> {
    pub fn new(data: Vec<Vec<K>>) -> Self {
        Matrix { data }
    }


    pub fn is_square(&self) -> bool {
        self.data.len() == self.data[0].len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty() || self.data[0].is_empty()
    }
}

impl<K: std::fmt::Display> Matrix<K> {
    pub fn print(&self) {
        for i in 0..self.data.len() {


            match (i, self.data.len()) {
                (0, 1) => print!("[ "),
                (0, _) => print!("┌ "),
                (x, len) if x == len - 1 && len > 1 => print!("└ "),
                _ => print!("│ "),
            }

            for j in 0..self.data[i].len() {
                if j != 0 {
                    print!(", ");
                }
                print!("{}", self.data[i][j]);
            }

            match (i, self.data.len()) {
                (0, 1) => print!(" ]"),
                (x, len) if x == len - 1 && len > 1 => print!(" ┘"),
                (0, _) => print!(" ┐"),
                _ => print!(" │"),
            }

            println!();
        }
    }
}

