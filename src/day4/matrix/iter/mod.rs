use std::iter::Chain;
use std::marker::PhantomData;
use super::Matrix;

pub type ArithmeticWalk<'a, M, T> = Walk<'a, M, T, ArithmeticIndices>;

pub struct Walk<'a, M, T, IT> {
    matrix: &'a M,
    indices: IT,
    phantom_t: PhantomData<T>
}

impl<'a, M, T, IT> Walk<'a, M, T, IT> {
    pub fn new(matrix: &'a M, indices: IT) -> Self {
        Self { matrix, indices, phantom_t: PhantomData::default() }
    }
}

impl<'a, M, T: 'a, IT> Iterator for Walk<'a, M, T, IT>
where
    M : Matrix<'a, T>,
    IT: Iterator<Item = (usize, usize)>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next();
        index.map(|(x, y)| self.matrix.get(x, y)).unwrap_or(None)
    }
}

#[derive(Clone, Debug)]
pub struct ArithmeticIndices {
    cur_x: i32,
    cur_y: i32,
    step_x: i32,
    step_y: i32,
}

impl ArithmeticIndices {
    pub fn new(cur_x: i32, cur_y: i32, step_x: i32, step_y: i32) -> Self {
        Self {
            cur_x,
            cur_y,
            step_x,
            step_y,
        }
    }
}

impl Iterator for ArithmeticIndices {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_x < 0 || self.cur_y < 0 {
            None
        } else {
            let (prev_x, prev_y) = (self.cur_x, self.cur_y);
            self.cur_x += self.step_x;
            self.cur_y += self.step_y;
            Some((prev_x as usize, prev_y as usize))
        }
    }
}

pub struct Rows<'a, M, T> {
    matrix: &'a M,
    current_row: usize,
    phantom_t: PhantomData<T>
}

impl<'a, M, T> Rows<'a, M, T> {
    pub fn new(matrix: &'a M) -> Self {
        Self {
            matrix,
            current_row: 0,
            phantom_t: PhantomData::default()
        }
    }
}

impl<'a, M, T> Iterator for Rows<'a, M, T>
where
    M: Matrix<'a, T>
{
    type Item = ArithmeticWalk<'a, M, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.matrix.row(self.current_row);
        self.current_row += 1;
        result
    }
}

pub struct Columns<'a, M, T> {
    matrix: &'a M,
    current_column: usize,
    phantom_t: PhantomData<T>
}

impl<'a, M, T> Columns<'a, M, T> {
    pub fn new(matrix: &'a M) -> Self {
        Self {
            matrix,
            current_column: 0,
            phantom_t: PhantomData::default()
        }
    }
}

impl<'a, M, T> Iterator for Columns<'a, M, T>
where
    M: Matrix<'a, T>
{
    type Item = ArithmeticWalk<'a, M, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.matrix.column(self.current_column);
        self.current_column += 1;
        result
    }
}

pub struct DownDiagonals<'a, M, T> {
    matrix: &'a M,
    current_diagonal: i32,
    phantom_t: PhantomData<T>
}

impl<'a, M, T> DownDiagonals<'a, M, T>
where
    M: Matrix<'a, T>
{
    pub fn new(matrix: &'a M) -> Self {
        Self {
            matrix,
            current_diagonal: -(matrix.row_count() as i32) + 1,
            phantom_t: PhantomData::default()
        }
    }
}

impl<'a, M, T> Iterator for DownDiagonals<'a, M, T>
where
    M: Matrix<'a, T>
{
    type Item = ArithmeticWalk<'a, M, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.matrix.down_diagonal(self.current_diagonal);
        self.current_diagonal += 1;
        result
    }
}

pub struct UpDiagonals<'a, M, T> {
    matrix: &'a M,
    current_diagonal: i32,
    phantom_t: PhantomData<T>
}

impl<'a, M, T> UpDiagonals<'a, M, T>
where
    M: Matrix<'a, T>
{
    pub fn new(matrix: &'a M) -> Self {
        Self {
            matrix,
            current_diagonal: -(matrix.row_count() as i32) + 1,
            phantom_t: PhantomData::default()
        }
    }
}

impl<'a, M, T> Iterator for UpDiagonals<'a, M, T>
where
    M: Matrix<'a, T>
{
    type Item = ArithmeticWalk<'a, M, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.matrix.up_diagonal(self.current_diagonal);
        self.current_diagonal += 1;
        result
    }
}

pub type WordSearch<'a, M, T> = Chain<Chain<Chain<Rows<'a, M, T>, Columns<'a, M, T>>, DownDiagonals<'a, M, T>>, UpDiagonals<'a, M, T>>;
