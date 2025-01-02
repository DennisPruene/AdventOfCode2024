use super::{Matrix, MutMatrix};
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug)]
pub struct SparseMatrix<T> {
    rows: usize,
    columns: usize,
    zero: T,
    matrix: HashMap<(usize, usize), T>,
    features_per_row: HashMap<T, HashMap<usize, BTreeSet<usize>>>,
    features_per_column: HashMap<T, HashMap<usize, BTreeSet<usize>>>,
}

impl<T> SparseMatrix<T> {
    pub fn new(rows: usize, columns: usize, zero: T) -> Self {
        Self {
            rows,
            columns,
            zero,
            matrix: HashMap::new(),
            features_per_row: HashMap::new(),
            features_per_column: HashMap::new(),
        }
    }
}

impl<T: Clone + Eq + Hash> SparseMatrix<T> {
    pub fn from_matrix<M: Matrix<T>>(matrix: M, zero: T) -> Self {
        let mut result = Self::new(matrix.row_count(), matrix.column_count(), zero.clone());
        for ((x, y), val) in matrix.indexed_iter() {
            if val == zero {
                continue;
            }
            result.set_non_zero(x, y, val);
        }
        result
    }

    fn clear(&mut self, x: usize, y: usize) {
        if !self.matrix.contains_key(&(x, y)) {
            return;
        }

        let removed_value = self.matrix.remove(&(x, y)).unwrap();
        self.features_per_row
            .get_mut(&removed_value)
            .unwrap()
            .get_mut(&y)
            .unwrap()
            .remove(&x);
        self.features_per_column
            .get_mut(&removed_value)
            .unwrap()
            .get_mut(&x)
            .unwrap()
            .remove(&y);
    }

    fn set_non_zero(&mut self, x: usize, y: usize, value: T) {
        if self.matrix.contains_key(&(x, y)) {
            self.clear(x, y);
        }

        self.matrix.insert((x, y), value.clone());
        self.features_per_row
            .entry(value.clone())
            .or_default()
            .entry(y)
            .or_default()
            .insert(x);
        self.features_per_column
            .entry(value)
            .or_default()
            .entry(x)
            .or_default()
            .insert(y);
    }
}

impl<T: Clone> Matrix<T> for SparseMatrix<T> {
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
            if let Some(value) = self.matrix.get(&(x, y)) {
                Some(value.clone())
            } else {
                Some(self.zero.clone())
            }
        }
    }
}

impl<T: Clone + Eq + Hash> MutMatrix<T> for SparseMatrix<T> {
    fn set(&mut self, x: usize, y: usize, value: T) {
        if value == self.zero {
            self.clear(x, y);
        } else {
            self.set_non_zero(x, y, value);
        }
    }
}

impl<T: Clone + Display> Display for SparseMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.columns {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
