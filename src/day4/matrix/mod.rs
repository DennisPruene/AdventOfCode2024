use self::base::MatrixBase;
use self::iter::*;
use self::view::{MatrixView, Range};
use std::ascii::Char;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod base;
mod iter;
mod view;

pub trait Matrix<'a, T>: Sized {
    fn row_count(&self) -> usize;

    fn column_count(&self) -> usize;

    fn get(&'a self, x: usize, y: usize) -> Option<&'a T>;

    fn size(&self) -> usize {
        self.row_count() * self.column_count()
    }

    fn iter(&'a self) -> FullWalk<'a, Self, T> {
        FullWalk::new(self, AllIndices::new(self.column_count()))
    }

    fn view(
        &'a self,
        x_start: isize,
        x_end: isize,
        y_start: isize,
        y_end: isize,
    ) -> Option<MatrixView<'a, Self, T>> {
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

    fn walk<IT>(&'a self, indices: IT) -> Walk<'a, Self, T, IT> {
        Walk::new(self, indices)
    }

    fn arithmetic_walk(
        &'a self,
        cur_x: usize,
        cur_y: usize,
        step_x: i32,
        step_y: i32,
    ) -> ArithmeticWalk<'a, Self, T> {
        Walk::new(
            self,
            ArithmeticIndices::new(cur_x as i32, cur_y as i32, step_x, step_y),
        )
    }

    fn row(&'a self, row_index: usize) -> Option<ArithmeticWalk<'a, Self, T>> {
        if row_index >= self.row_count() {
            None
        } else {
            Some(self.arithmetic_walk(0, row_index, 1, 0))
        }
    }

    fn rows(&'a self) -> Rows<'a, Self, T> {
        Rows::new(self)
    }

    fn column(&'a self, column_index: usize) -> Option<ArithmeticWalk<'a, Self, T>> {
        if column_index >= self.column_count() {
            None
        } else {
            Some(self.arithmetic_walk(column_index, 0, 0, 1))
        }
    }

    fn columns(&'a self) -> Columns<'a, Self, T> {
        Columns::new(self)
    }

    fn down_diagonal(&'a self, diagonal_index: i32) -> Option<ArithmeticWalk<'a, Self, T>> {
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

    fn down_diagonals(&'a self) -> DownDiagonals<'a, Self, T> {
        DownDiagonals::new(self)
    }

    fn up_diagonal(&'a self, diagonal_index: i32) -> Option<ArithmeticWalk<'a, Self, T>> {
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

    fn up_diagonals(&'a self) -> UpDiagonals<'a, Self, T> {
        UpDiagonals::new(self)
    }

    fn word_search(&'a self) -> WordSearch<'a, Self, T> {
        self.rows()
            .chain(self.columns())
            .chain(self.down_diagonals())
            .chain(self.up_diagonals())
    }

    fn convolve_iter(
        &'a self,
        convolve_width: usize,
        convolve_height: usize,
    ) -> ConvolveIter<'a, Self, T> {
        ConvolveIter::new(self, convolve_width, convolve_height)
    }

    fn convolve<F, U>(&'a self, width: usize, height: usize, convolve: F) -> MatrixBase<U>
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

    fn count_non_zero(&'a self) -> usize
    where
        T: 'a + Default + Eq,
    {
        self.iter().filter(|t| t != &&T::default()).count()
    }
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
