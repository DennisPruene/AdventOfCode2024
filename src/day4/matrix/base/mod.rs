use super::*;

#[derive(Debug)]
pub struct MatrixBase<T> {
    rows: usize,
    columns: usize,
    inner: Vec<T>,
}

impl<T> MatrixBase<T> {
    pub fn new(rows: usize, columns: usize, inner: Vec<T>) -> Self {
        assert_eq!(rows * columns, inner.len());
        Self {
            rows,
            columns,
            inner,
        }
    }
}

impl<'a, T> IntoIterator for &'a MatrixBase<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<T: Clone> Matrix<T> for MatrixBase<T> {
    fn row_count(&self) -> usize {
        self.rows
    }

    fn column_count(&self) -> usize {
        self.columns
    }

    fn get(&self, x: usize, y: usize) -> Option<T> {
        if x >= self.rows || y >= self.columns {
            None
        } else {
            self.inner.get(y * self.columns + x).cloned()
        }
    }
}

impl<T: Clone> MutMatrix<T> for MatrixBase<T> {
    fn set(&mut self, x: usize, y: usize, value: T) {
        if let Some(destination) = self.inner.get_mut(y * self.columns + x) {
            *destination = value;
        }
    }
}

impl TryFrom<String> for MatrixBase<Char> {
    type Error = CharAsAsciiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut rows = None;
        let mut columns = 0;
        let mut chars = vec![];
        for line in value.lines() {
            columns += 1;
            let mut cur_chars = vec![];
            for c in line.chars() {
                cur_chars.push(c.as_ascii().ok_or(CharAsAsciiError(c))?);
            }
            if let Some(rows) = &rows {
                assert_eq!(*rows, cur_chars.len());
            } else {
                rows = Some(cur_chars.len());
            }
            chars.append(&mut cur_chars);
        }

        Ok(MatrixBase::new(rows.unwrap(), columns, chars))
    }
}
