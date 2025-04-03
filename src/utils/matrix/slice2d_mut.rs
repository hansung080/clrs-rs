use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Index, Range, Sub};
use crate::utils::matrix::Vec2d;
use crate::utils::ops::{IntoRange, Len, SliceMut};

pub struct Slice2dMut<'a, T: 'a> {
    slice: &'a mut [T],
    row: Range<usize>,
    col: Range<usize>,
}

impl<'a, T: Len> Slice2dMut<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        let row_len = slice.len();
        let col_len = if row_len == 0 { 0 } else { slice[0].len() };
        Slice2dMut {
            slice,
            row: 0..row_len,
            col: 0..col_len,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.row.len(), self.col.len())
    }
}

impl<'a, T> Debug for Slice2dMut<'a , T>
where
    T: Index<Range<usize>>,
    <T as Index<Range<usize>>>::Output: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut v = Vec::new();
        for i in self.row.clone() {
            v.push(&self.slice[i][self.col.clone()])
        }
        write!(f, "Slice2dMut({v:?})")
    }
}

impl<'a, T> PartialEq for Slice2dMut<'a, T>
where
    T: Index<usize> + Len,
    <T as Index<usize>>::Output: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.shape() != other.shape() { return false; }

        for i in 0..self.row.len() {
            for j in 0..self.col.len() {
                if self.slice[self.row.start + i][self.col.start + j] != other.slice[other.row.start + i][other.col.start + j] {
                    return false;
                }
            }
        }
        true
    }
}

impl<'a, T> Index<(usize, usize)> for Slice2dMut<'a, T>
where
    T: Index<usize>,
{
    type Output = <T as Index<usize>>::Output;

    fn index(&self, (row_idx, col_idx): (usize, usize)) -> &Self::Output {
        assert!(row_idx < self.row.len(), "row index out of bounds: the len is {} but the index is {}", self.row.len(), row_idx);
        assert!(col_idx < self.col.len(), "column index out of bounds: the len is {} but the index is {}", self.col.len(), col_idx);
        &self.slice[self.row.start + row_idx][self.col.start + col_idx]
    }
}

impl<'a, 'b, T: 'b, Rng1, Rng2> SliceMut<'b, (Rng1, Rng2)> for Slice2dMut<'a, T>
where
    Rng1: IntoRange<usize>,
    Rng2: IntoRange<usize>,
{
    type Output = Slice2dMut<'b, T>;

    fn slice(&'b mut self, (row, col): (Rng1, Rng2)) -> Self::Output {
        let (row, col) =
            (row.into_range(Range { start: 0, end: self.row.len() }),
             col.into_range(Range { start: 0, end: self.col.len() }));
        assert!(row.start <= row.end, "row range index starts at {} but ends at {}", row.start, row.end);
        assert!(col.start <= col.end, "column range index starts at {} but ends at {}", col.start, col.end);
        assert!(row.end <= self.row.len(), "row range end index {} out of range for row of length {}", row.end, self.row.len());
        assert!(col.end <= self.col.len(), "column range end index {} out of range for column of length {}", col.end, self.col.len());

        Slice2dMut {
            slice: self.slice,
            row: Range { start: self.row.start + row.start, end: self.row.start + row.end },
            col: Range { start: self.col.start + col.start, end: self.col.start + col.end },
        }
    }
}

impl<'a, T> Add for Slice2dMut<'a, T>
where
    T: Index<usize> + Len,
    <T as Index<usize>>::Output: Add<Output = <T as Index<usize>>::Output> + Copy,
{
    type Output = Vec2d<<T as Index<usize>>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for i in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for j in 0..col_len {
                result_row.push(self.slice[self.row.start + i][self.col.start + j] + rhs.slice[rhs.row.start + i][rhs.col.start + j]);
            }
            result.push(result_row);
        }
        result
    }
}

impl<'a, T> Sub for Slice2dMut<'a, T>
where
    T: Index<usize> + Len,
    <T as Index<usize>>::Output: Sub<Output = <T as Index<usize>>::Output> + Copy,
{
    type Output = Vec2d<<T as Index<usize>>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for i in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for j in 0..col_len {
                result_row.push(self.slice[self.row.start + i][self.col.start + j] - rhs.slice[rhs.row.start + i][rhs.col.start + j]);
            }
            result.push(result_row);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice2d_index_and_slice() {
        let mut a = [
            [ 1,  2,  3,  4],
            [ 5,  6,  7,  8],
            [ 9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        let mut a = Slice2dMut::new(&mut a);

        assert_eq!(a[(0, 0)], 1);
        assert_eq!(a[(1, 2)], 7);
        assert_eq!(a[(3, 3)], 16);

        let mut expected = [[6, 7, 0, 0], [10, 11, 0, 0]];
        assert_eq!(a.slice((1..3, 1..3)), Slice2dMut::new(&mut expected).slice((.., 0..2)));
        let mut expected = [[6, 7, 0, 0], [10, 11, 0, 0]];
        assert_eq!(a.slice((1..=2, 1..=2)), Slice2dMut::new(&mut expected).slice((.., ..2)));
        let mut expected = [[6, 7, 8, 0], [10, 11, 12, 0], [14, 15, 16, 0]];
        assert_eq!(a.slice((1.., 1..)), Slice2dMut::new(&mut expected).slice((.., 0..3)));
        let mut expected = [[1, 2, 3, 0], [5, 6, 7, 0], [9, 10, 11, 0]];
        assert_eq!(a.slice((..3, ..3)), Slice2dMut::new(&mut expected).slice((.., ..3)));
        let mut expected = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]];
        assert_eq!(a.slice((.., ..)), Slice2dMut::new(&mut expected));

        let mut a = a.slice((1..3, 1..));
        let mut b = [
            [ 6,  7,  8, 0],
            [10, 11, 12, 0],
        ];
        let mut b = Slice2dMut::new(&mut b);
        let mut b = b.slice((.., ..3));
        assert_eq!(a, b);

        assert_eq!(a[(0, 0)], 6);
        assert_eq!(a[(1, 2)], 12);

        assert_eq!(a.slice((1..2, 2..3)), b.slice((1..2, 2..3)));
        assert_eq!(a.slice((1..=1, 2..=2)), b.slice((1..=1, 2..=2)));
        assert_eq!(a.slice((1.., 2..)), b.slice((1.., 2..)));
        assert_eq!(a.slice((..2, ..3)), b.slice((..2, ..3)));
        assert_eq!(a.slice((.., ..)), b.slice((.., ..)));
    }

    #[test]
    fn slice2d_add() {
        let mut a = [];
        let mut b = [];
        assert_eq!(Slice2dMut::<[i32; 0]>::new(&mut a) + Slice2dMut::new(&mut b), Vec2d(vec![]));

        let mut a = [[1]];
        let mut b = [[2]];
        assert_eq!(Slice2dMut::new(&mut a) + Slice2dMut::new(&mut b), Vec2d(vec![vec![3]]));

        let mut a = [[1.1]];
        let mut b = [[2.2]];
        assert_eq!(Slice2dMut::new(&mut a) + Slice2dMut::new(&mut b), Vec2d(vec![vec![1.1 + 2.2]]));

        let mut a = [
            [1, 2, 3],
            [4, 5, 6],
        ];
        let mut b = [
            [ 7,  8,  9],
            [10, 11, 12],
        ];
        assert_eq!(Slice2dMut::new(&mut a) + Slice2dMut::new(&mut b), Vec2d(
            vec![
                vec![ 8, 10, 12],
                vec![14, 16, 18],
            ]
        ));

        let mut a = [
            [ 1,  2,  3,  4],
            [ 5,  6,  7,  8],
            [ 9, 10, 11, 12],
            [13, 14, 15, 16],
        ];

        let mut b = [
            [ 1,  2,  3,  4],
            [ 5,  6,  7,  8],
            [ 9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        assert_eq!(Slice2dMut::new(&mut a).slice((..2, ..3)) + Slice2dMut::new(&mut b).slice((2.., 1..)), Vec2d(
            vec![
                vec![11, 13, 15],
                vec![19, 21, 23],
            ]
        ));
    }

    #[test]
    fn slice2d_sub() {
        let mut a = [];
        let mut b = [];
        assert_eq!(Slice2dMut::<[i32; 0]>::new(&mut a) - Slice2dMut::new(&mut b), Vec2d(vec![]));

        let mut a = [[1]];
        let mut b = [[2]];
        assert_eq!(Slice2dMut::new(&mut a) - Slice2dMut::new(&mut b), Vec2d(vec![vec![-1]]));

        let mut a = [[1.1]];
        let mut b = [[2.2]];
        assert_eq!(Slice2dMut::new(&mut a) - Slice2dMut::new(&mut b), Vec2d(vec![vec![1.1 - 2.2]]));

        let mut a = [
            [1, 2, 3],
            [4, 5, 6],
        ];
        let mut b = [
            [ 7,  8,  9],
            [10, 11, 12],
        ];
        assert_eq!(Slice2dMut::new(&mut a) - Slice2dMut::new(&mut b), Vec2d(
            vec![
                vec![-6, -6, -6],
                vec![-6, -6, -6],
            ]
        ));

        let mut a = [
            [ 1,  2,  3,  4],
            [ 5,  6,  7,  8],
            [ 9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        let mut b = [
            [ 1,  2,  3,  4],
            [ 5,  6,  7,  8],
            [ 9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        assert_eq!(Slice2dMut::new(&mut a).slice((..2, ..3)) - Slice2dMut::new(&mut b).slice((2.., 1..)), Vec2d(
            vec![
                vec![-9, -9, -9],
                vec![-9, -9, -9],
            ]
        ));
    }
}