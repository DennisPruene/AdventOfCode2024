use super::Matrix;
use std::marker::PhantomData;
use std::ops::Mul;

pub struct Range<Idx> {
    start: Idx,
    end: Idx,
    step: Idx,
}

impl<Idx> Range<Idx>
where
    Idx: Mul + Copy,
    <Idx as Mul>::Output: Ord,
{
    pub fn new(start: Idx, end: Idx, step: Idx) -> Self {
        assert!(start * step <= end * step);
        Self { start, end, step }
    }
}

pub struct MatrixView<'a, M, T> {
    matrix: &'a M,
    x_range: Range<isize>,
    y_range: Range<isize>,
    phantom_t: PhantomData<T>,
}

impl<'a, M, T> MatrixView<'a, M, T>
where
    M: Matrix<'a, T>,
{
    pub fn new(matrix: &'a M, x_range: Range<isize>, y_range: Range<isize>) -> Self {
        assert!(x_range.start >= 0);
        assert!(x_range.start <= matrix.column_count() as isize);
        assert!(x_range.end >= 0);
        assert!(x_range.end <= matrix.column_count() as isize);
        assert!(y_range.start >= 0);
        assert!(y_range.start <= matrix.column_count() as isize);
        assert!(y_range.end >= 0);
        assert!(y_range.end <= matrix.column_count() as isize);
        Self {
            matrix,
            x_range,
            y_range,
            phantom_t: PhantomData::default(),
        }
    }
}

impl<'a, M, T> Matrix<'a, T> for MatrixView<'a, M, T>
where
    M: Matrix<'a, T>,
{
    fn row_count(&self) -> usize {
        ((self.y_range.end - self.y_range.start) / self.y_range.step) as usize
    }

    fn column_count(&self) -> usize {
        ((self.x_range.end - self.x_range.start) / self.x_range.step) as usize
    }

    fn get(&'a self, x: usize, y: usize) -> Option<&'a T> {
        if y >= self.row_count() || x >= self.column_count() {
            None
        } else {
            self.matrix.get(
                (self.x_range.start + (x as isize) * self.x_range.step) as usize,
                (self.y_range.start + (y as isize) * self.y_range.step) as usize,
            )
        }
    }
}
