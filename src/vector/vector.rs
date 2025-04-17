#[derive(Clone, PartialEq, Eq, Default)]
pub struct Vector<K> {
    pub data: Vec<K>,
}

impl<K: std::fmt::Display> std::fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, item) in self.data.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, ")")
    }
}

impl <K : std::clone::Clone> Vector<K> {
    pub fn from(data: &[K]) -> Self {
        Vector {
            data: data.to_vec(),
        }
    }
}

impl <K> Vector<K> {
    pub fn new(data: Vec<K>) -> Self {
        Vector { data }
    }
}

impl<K> std::ops::Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K> std::ops::IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl <K: std::fmt::Display> Vector<K> {
    pub fn print(&self) {
        println!("{}", self);
    }
}