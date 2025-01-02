use self::base::MatrixBase;
use self::iter::*;
use self::view::{MatrixView, Range};
use std::ascii::Char;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use self::iter::indexed::IndexedFullWalk;

pub mod base;
pub mod bool;
pub mod sparse;
mod iter;
mod view;

pub trait Matrix<T>: Sized {
    fn row_count(&self) -> usize;

    fn column_count(&self) -> usize;

    fn get(&self, x: usize, y: usize) -> Option<T>;

    fn size(&self) -> usize {
        self.row_count() * self.column_count()
    }

    fn iter(&self) -> FullWalk<Self, T> {
        FullWalk::new(self, AllIndices::new(self.column_count()))
    }

    fn indexed_iter(&self) -> IndexedFullWalk<Self, T> {
        IndexedFullWalk::new(self, AllIndices::new(self.column_count()))
    }

    fn view(
        &self,
        x_start: isize,
        x_end: isize,
        y_start: isize,
        y_end: isize,
    ) -> Option<MatrixView<Self, T>> {
        if x_start < 0
            || x_end > self.column_count() as isize
            || y_start < 0
            || y_end > self.row_count() as isize
        {
            None
        } else {
            Some(MatrixView::new(
                self,
                Range::new(x_start, x_end, 1),
                Range::new(y_start, y_end, 1),
            ))
        }
    }

    fn walk<IT>(&self, indices: IT) -> Walk<Self, T, IT> {
        Walk::new(self, indices)
    }

    fn arithmetic_walk(
        &self,
        cur_x: usize,
        cur_y: usize,
        step_x: i32,
        step_y: i32,
    ) -> ArithmeticWalk<Self, T> {
        Walk::new(
            self,
            ArithmeticIndices::new(cur_x as i32, cur_y as i32, step_x, step_y),
        )
    }

    fn row(&self, row_index: usize) -> Option<ArithmeticWalk<Self, T>> {
        if row_index >= self.row_count() {
            None
        } else {
            Some(self.arithmetic_walk(0, row_index, 1, 0))
        }
    }

    fn rows(&self) -> Rows<Self, T> {
        Rows::new(self)
    }

    fn column(&self, column_index: usize) -> Option<ArithmeticWalk<Self, T>> {
        if column_index >= self.column_count() {
            None
        } else {
            Some(self.arithmetic_walk(column_index, 0, 0, 1))
        }
    }

    fn columns(&self) -> Columns<Self, T> {
        Columns::new(self)
    }

    fn down_diagonal(&self, diagonal_index: i32) -> Option<ArithmeticWalk<Self, T>> {
        if diagonal_index <= -(self.row_count() as i32)
            || diagonal_index >= self.column_count() as i32
        {
            None
        } else if diagonal_index < 0 {
            Some(self.arithmetic_walk(0, (-diagonal_index) as usize, 1, 1))
        } else {
            Some(self.arithmetic_walk(diagonal_index as usize, 0, 1, 1))
        }
    }

    fn down_diagonals(&self) -> DownDiagonals<Self, T> {
        DownDiagonals::new(self)
    }

    fn up_diagonal(&self, diagonal_index: i32) -> Option<ArithmeticWalk<Self, T>> {
        if diagonal_index <= -(self.row_count() as i32)
            || diagonal_index >= self.column_count() as i32
        {
            None
        } else if diagonal_index < 0 {
            Some(self.arithmetic_walk(
                0,
                self.row_count() - ((-diagonal_index) as usize) - 1,
                1,
                -1,
            ))
        } else {
            Some(self.arithmetic_walk(diagonal_index as usize, self.column_count() - 1, 1, -1))
        }
    }

    fn up_diagonals(&self) -> UpDiagonals<Self, T> {
        UpDiagonals::new(self)
    }

    fn word_search(&self) -> WordSearch<Self, T> {
        self.rows()
            .chain(self.columns())
            .chain(self.down_diagonals())
            .chain(self.up_diagonals())
    }

    fn convolve_iter(
        &self,
        convolve_width: usize,
        convolve_height: usize,
    ) -> ConvolveIter<Self, T> {
        ConvolveIter::new(self, convolve_width, convolve_height)
    }

    fn convolve<F, U>(&self, width: usize, height: usize, convolve: F) -> MatrixBase<U>
    where
        F: Fn(MatrixView<Self, T>) -> U,
    {
        let new_row_count = self.row_count() - height + 1;
        let new_column_count = self.column_count() - width + 1;
        let mut result = Vec::with_capacity(new_row_count * new_column_count);
        for view in self.convolve_iter(width, height) {
            result.push(convolve(view));
        }
        MatrixBase::new(new_row_count, new_column_count, result)
    }

    fn count_where<P: Fn(&T) -> bool>(&self, predicate: P) -> usize {
        self.iter().filter(predicate).count()
    }

    fn count_non_zero(&self) -> usize
    where
        T: Default + Eq,
    {
        self.count_where(|t| t != &T::default())
    }
}

pub trait MutMatrix<T>: Matrix<T> {
    fn set(&mut self, x: usize, y: usize, value: T);
}

#[derive(Debug)]
pub struct CharAsAsciiError(char);

impl Display for CharAsAsciiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tried to convert non-ascii character '{}' to an ascii-character",
            self.0
        )
    }
}

impl Error for CharAsAsciiError {}
