use std::marker::PhantomData;
use super::Matrix;
use super::AllIndices;
use super::ArithmeticIndices;

pub type IndexedFullWalk<'a, M, T> = IndexedWalk<'a, M, T, AllIndices>;

pub type IndexedArithmeticWalk<'a, M, T> = IndexedWalk<'a, M, T, ArithmeticIndices>;

pub struct IndexedWalk<'a, M, T, IT> {
    matrix: &'a M,
    indices: IT,
    phantom_t: PhantomData<T>,
}

impl<'a, M, T, IT> IndexedWalk<'a, M, T, IT> {
    pub fn new(matrix: &'a M, indices: IT) -> Self {
        Self {
            matrix,
            indices,
            phantom_t: PhantomData::default()
        }
    }
}

impl<'a, M, T, IT> Iterator for IndexedWalk<'a, M, T, IT>
where
    M: Matrix<T>,
    IT: Iterator<Item = (usize, usize)>,
{
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((x, y)) = self.indices.next() {
            self.matrix.get(x, y).map(|val| ((x, y), val))
        } else {
            None
        }
    }
}