use std::fmt::{Display, Formatter};
use super::{Matrix, MutMatrix};

pub struct BoolMatrix {
    rows: usize,
    columns: usize,
    inner: Vec<u8>
}

impl BoolMatrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            inner: vec![0; (rows * columns).div_ceil(8)]
        }
    }

    pub fn transitive_closure(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.columns {
                if self.get(x, y).unwrap() {
                    for z in 0..self.rows {
                        if self.get(y, z) == Some(true) {
                            self.set(x, z, true);
                        }
                    }
                }
            }
        }
    }
}

impl Matrix<bool> for BoolMatrix {
    fn row_count(&self) -> usize {
        self.rows
    }

    fn column_count(&self) -> usize {
        self.columns
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.columns || y >= self.rows {
            None
        } else {
            let flat_index = y * self.columns + x;
            let byte = *self.inner.get(flat_index / 8).unwrap();
            Some(((byte >> (7 - (flat_index % 8))) & 1) != 0)
        }
    }
}

impl MutMatrix<bool> for BoolMatrix {
    fn set(&mut self, x: usize, y: usize, value: bool) {
        if x >= self.columns || y >= self.rows {
            return;
        }

        let flat_index = y * self.columns + x;
        let byte = self.inner.get_mut(flat_index / 8).unwrap();
        let byte_index = 7 - (flat_index % 8);
        *byte = (*byte & !(1u8 << byte_index)) | (u8::from(value) << byte_index);
    }
}

impl Display for BoolMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.columns {
                write!(f, "{}", if self.get(x, y).unwrap() { 1 } else { 0 })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}