use std::fmt::{self, Debug, Formatter};
use std::ops::{Index, Range};
use crate::utils::ops::{IntoRange, Len, Slice};

pub struct Slice2d<'s, T: 's> {
    slice: &'s [T],
    row: Range<usize>,
    col: Range<usize>,
}

impl<'s, T: Len> Slice2d<'s, T> {
    pub fn new(slice: &'s [T]) -> Self {
        Slice2d {
            slice,
            row: 0..slice.len(),
            col: 0..if slice.len() == 0 { 0 } else { slice[0].len() },
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.row.len(), self.col.len())
    }
}

impl<'s, T> Debug for Slice2d<'s , T>
where
    T: Index<Range<usize>>,
    <T as Index<Range<usize>>>::Output: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut v = Vec::new();
        for i in self.row.clone() {
            v.push(&self.slice[i][self.col.clone()])
        }
        write!(f, "Slice2d({v:?})")
    }
}

impl<'s, T> PartialEq for Slice2d<'s, T>
where
    T: Len + Index<usize>,
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

impl<'s, T> Index<(usize, usize)> for Slice2d<'s, T>
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

impl<'s, T, Rng1, Rng2> Slice<(Rng1, Rng2)> for Slice2d<'s, T>
where
    Rng1: IntoRange<usize>,
    Rng2: IntoRange<usize>,
{
    type Output = Self;

    fn slice(&self, (row, col): (Rng1, Rng2)) -> Self::Output {
        let (row, col) =
            (row.into_range(Range { start: 0, end: self.row.len() }),
             col.into_range(Range { start: 0, end: self.col.len() }));
        assert!(row.start <= row.end, "row range index starts at {} but ends at {}", row.start, row.end);
        assert!(col.start <= col.end, "column range index starts at {} but ends at {}", col.start, col.end);
        assert!(row.end <= self.row.len(), "row range end index {} out of range for row of length {}", row.end, self.row.len());
        assert!(col.end <= self.col.len(), "column range end index {} out of range for column of length {}", col.end, self.col.len());

        Slice2d {
            slice: self.slice,
            row: Range { start: self.row.start + row.start, end: self.row.start + row.end },
            col: Range { start: self.col.start + col.start, end: self.col.start + col.end },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice2d_index_and_slice() {
        let a = Slice2d::new(
            &[
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        );

        assert_eq!(a[(0, 0)], 1);
        assert_eq!(a[(1, 2)], 7);
        assert_eq!(a[(3, 3)], 16);

        assert_eq!(a.slice((1..3, 1..3)), Slice2d::new(&[[6, 7, 0, 0], [10, 11, 0, 0]]).slice((.., 0..2)));
        assert_eq!(a.slice((1..=2, 1..=2)), Slice2d::new(&[[6, 7, 0, 0], [10, 11, 0, 0]]).slice((.., ..2)));
        assert_eq!(a.slice((1.., 1..)), Slice2d::new(&[[6, 7, 8, 0], [10, 11, 12, 0], [14, 15, 16, 0]]).slice((.., 0..3)));
        assert_eq!(a.slice((..3, ..3)), Slice2d::new(&[[1, 2, 3, 0], [5, 6, 7, 0], [9, 10, 11, 0]]).slice((.., ..3)));
        assert_eq!(a.slice((.., ..)), Slice2d::new(&[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]));

        let a = a.slice((1..3, 1..));
        let b = Slice2d::new(
            &[
                [ 6,  7,  8, 0],
                [10, 11, 12, 0],
            ]
        ).slice((.., ..3));
        assert_eq!(a, b);

        assert_eq!(a[(0, 0)], 6);
        assert_eq!(a[(1, 2)], 12);

        assert_eq!(a.slice((1..2, 2..3)), b.slice((1..2, 2..3)));
        assert_eq!(a.slice((1..=1, 2..=2)), b.slice((1..=1, 2..=2)));
        assert_eq!(a.slice((1.., 2..)), b.slice((1.., 2..)));
        assert_eq!(a.slice((..2, ..3)), b.slice((..2, ..3)));
        assert_eq!(a.slice((.., ..)), b.slice((.., ..)));
    }
}