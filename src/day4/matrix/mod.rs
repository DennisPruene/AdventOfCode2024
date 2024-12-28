use std::ascii::Char;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use self::iter::*;

pub mod base;
mod iter;
mod view;

pub trait Matrix<'a, T> {
    fn row_count(&self) -> usize;

    fn column_count(&self) -> usize;

    fn get(&'a self, x: usize, y: usize) -> Option<&'a T>;

    fn walk<IT>(&'a self, indices: IT) -> Walk<'a, Self, T, IT> where Self: Sized {
        Walk::new(self, indices)
    }

    fn arithmetic_walk(
        &'a self,
        cur_x: usize,
        cur_y: usize,
        step_x: i32,
        step_y: i32,
    ) -> ArithmeticWalk<'a, Self, T> where Self: Sized {
        Walk::new(
            self,
            ArithmeticIndices::new(cur_x as i32, cur_y as i32, step_x, step_y),
        )
    }

    fn row(&'a self, row_index: usize) -> Option<ArithmeticWalk<'a, Self, T>> where Self: Sized {
        if row_index >= self.row_count() {
            None
        } else {
            Some(self.arithmetic_walk(0, row_index, 1, 0))
        }
    }

    fn rows(&'a self) -> Rows<'a, Self, T> where Self: Sized {
        Rows::new(self)
    }

    fn column(&'a self, column_index: usize) -> Option<ArithmeticWalk<'a, Self, T>> where Self: Sized {
        if column_index >= self.column_count() {
            None
        } else {
            Some(self.arithmetic_walk(column_index, 0, 0, 1))
        }
    }

    fn columns(&'a self) -> Columns<'a, Self, T> where Self: Sized {
        Columns::new(self)
    }

    fn down_diagonal(&'a self, diagonal_index: i32) -> Option<ArithmeticWalk<'a, Self, T>> where Self: Sized {
        if diagonal_index <= -(self.row_count() as i32) || diagonal_index >= self.column_count() as i32 {
            None
        } else if diagonal_index < 0 {
            Some(self.arithmetic_walk(0, (-diagonal_index) as usize, 1, 1))
        } else {
            Some(self.arithmetic_walk(diagonal_index as usize, 0, 1, 1))
        }
    }

    fn down_diagonals(&'a self) -> DownDiagonals<'a, Self, T> where Self: Sized {
        DownDiagonals::new(self)
    }

    fn up_diagonal(&'a self, diagonal_index: i32) -> Option<ArithmeticWalk<'a, Self, T>> where Self: Sized {
        if diagonal_index <= -(self.row_count() as i32) || diagonal_index >= self.column_count() as i32 {
            None
        } else if diagonal_index < 0 {
            Some(self.arithmetic_walk(0, self.row_count() - ((-diagonal_index) as usize) - 1, 1, -1))
        } else {
            Some(self.arithmetic_walk(diagonal_index as usize, self.column_count() - 1, 1, -1))
        }
    }

    fn up_diagonals(&'a self) -> UpDiagonals<'a, Self, T> where Self: Sized {
        UpDiagonals::new(self)
    }

    fn word_search(&'a self) -> WordSearch<'a, Self, T> where Self: Sized {
        self.rows().chain(self.columns()).chain(self.down_diagonals()).chain(self.up_diagonals())
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
