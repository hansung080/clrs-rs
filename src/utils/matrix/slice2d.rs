use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Index, Range, Sub};
use crate::utils::matrix::{Shape, Vec2d};
use crate::utils::ops::{IntoRange, Len, Slice};

#[derive(Clone)]
pub struct Slice2d<'a, T: 'a> {
    slice: &'a [T],
    row: Range<usize>,
    col: Range<usize>,
}

impl<'a, T> Slice2d<'a, T> {
    pub fn new(slice: &'a [T]) -> Self
    where
        T: Len,
    {
        Slice2d {
            slice,
            row: 0..slice.len(),
            col: 0..if slice.len() == 0 { 0 } else { slice[0].len() },
        }
    }

    pub fn to_vec2d(&self) -> Vec2d<<T as Index<usize>>::Output>
    where
        T: Index<usize>,
        <T as Index<usize>>::Output: Copy,
    {
        let mut result = Vec2d(Vec::with_capacity(self.row.len()));
        for i in self.row.clone() {
            let mut result_row = Vec::with_capacity(self.col.len());
            for j in self.col.clone() {
                result_row.push(self.slice[i][j]);
            }
            result.push(result_row);
        }
        result
    }
}

impl<'a, T> Debug for Slice2d<'a , T>
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

impl<'a, 'b, T, U> PartialEq<Slice2d<'b, U>> for Slice2d<'a, T>
where
    T: Index<usize>,
    <T as Index<usize>>::Output: PartialEq<<U as Index<usize>>::Output>,
    U: Index<usize>,
{
    fn eq(&self, other: &Slice2d<'b, U>) -> bool {
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

impl<'a, T> Shape for Slice2d<'a, T> {
    fn shape(&self) -> (usize, usize) {
        (self.row.len(), self.col.len())
    }
}

impl<'a, T> Index<(usize, usize)> for Slice2d<'a, T>
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

impl<'a, T> Index<(usize, usize)> for &Slice2d<'a, T>
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

impl<'a, T, Rng1, Rng2> Slice<'_, (Rng1, Rng2)> for Slice2d<'a, T>
where
    Rng1: IntoRange<usize>,
    Rng2: IntoRange<usize>,
{
    type Output = Self;

    fn slice(&self, (row, col): (Rng1, Rng2)) -> Self::Output {
        let row = row.into_range(0..self.row.len());
        let col = col.into_range(0..self.col.len());
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

impl<'a, T, Rhs> Add<Rhs> for Slice2d<'a, T>
where
    T: Index<usize>,
    <T as Index<usize>>::Output: Add<<Rhs as Index<(usize, usize)>>::Output, Output = <T as Index<usize>>::Output> + Copy,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    type Output = Vec2d<<T as Index<usize>>::Output>;

    #[inline(always)]
    fn add(self, rhs: Rhs) -> Self::Output {
        &self + rhs
    }
}

impl<'a, T, Rhs> Add<Rhs> for &Slice2d<'a, T>
where
    T: Index<usize>,
    <T as Index<usize>>::Output: Add<<Rhs as Index<(usize, usize)>>::Output, Output = <T as Index<usize>>::Output> + Copy,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    type Output = Vec2d<<T as Index<usize>>::Output>;

    fn add(self, rhs: Rhs) -> Self::Output {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for i in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for j in 0..col_len {
                result_row.push(self.slice[self.row.start + i][self.col.start + j] + rhs[(i, j)]);
            }
            result.push(result_row);
        }
        result
    }
}

impl<'a, T, Rhs> Sub<Rhs> for Slice2d<'a, T>
where
    T: Index<usize>,
    <T as Index<usize>>::Output: Sub<<Rhs as Index<(usize, usize)>>::Output, Output = <T as Index<usize>>::Output> + Copy,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    type Output = Vec2d<<T as Index<usize>>::Output>;

    #[inline(always)]
    fn sub(self, rhs: Rhs) -> Self::Output {
        &self - rhs
    }
}

impl<'a, T, Rhs> Sub<Rhs> for &Slice2d<'a, T>
where
    T: Index<usize>,
    <T as Index<usize>>::Output: Sub<<Rhs as Index<(usize, usize)>>::Output, Output = <T as Index<usize>>::Output> + Copy,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    type Output = Vec2d<<T as Index<usize>>::Output>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for i in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for j in 0..col_len {
                result_row.push(self.slice[self.row.start + i][self.col.start + j] - rhs[(i, j)]);
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
    fn slice2d_debug() {
        assert_eq!(format!("{:?}", Slice2d::<[i32; 0]>::new(&[])), String::from("Slice2d([])"));
        assert_eq!(format!("{:?}", Slice2d::new(&[[1]])), String::from("Slice2d([[1]])"));
        let mut a = Slice2d::new(&[[1, 2], [3, 4]]);
        assert_eq!(format!("{:?}", a), String::from("Slice2d([[1, 2], [3, 4]])"));
        assert_eq!(format!("{:?}", &a), String::from("Slice2d([[1, 2], [3, 4]])"));
        assert_eq!(format!("{:?}", &mut a), String::from("Slice2d([[1, 2], [3, 4]])"));
    }

    #[test]
    fn slice2d_partial_eq() {
        assert_eq!(Slice2d::new(&[[1]]), Slice2d::new(&[[1]]));
        assert_eq!(&Slice2d::new(&[[1]]), &Slice2d::new(&[[1]]));
        assert_eq!(&Slice2d::new(&[[1]]), &mut Slice2d::new(&[[1]]));
        assert_eq!(&mut Slice2d::new(&[[1]]), &Slice2d::new(&[[1]]));
        assert_eq!(&mut Slice2d::new(&[[1]]), &mut Slice2d::new(&[[1]]));
    }

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

        assert_eq!(a.slice((1..3, 1..3)), Slice2d::new(&[[6, 7], [10, 11]]));
        assert_eq!(a.slice((1..=2, 1..=2)), Slice2d::new(&[[6, 7], [10, 11]]));
        assert_eq!(a.slice((1.., 1..)), Slice2d::new(&[[6, 7, 8], [10, 11, 12], [14, 15, 16]]));
        assert_eq!(a.slice((..3, ..3)), Slice2d::new(&[[1, 2, 3], [5, 6, 7], [9, 10, 11]]));
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

    #[test]
    fn slice2d_add() {
        assert_eq!(Slice2d::<[i32; 0]>::new(&[]) + Slice2d::<[i32; 0]>::new(&[]), Vec2d::<i32>(vec![]));
        assert_eq!(Slice2d::new(&[[1]]) + Slice2d::new(&[[2]]), Vec2d(vec![vec![3]]));
        assert_eq!(Slice2d::new(&[[1.1]]) + Slice2d::new(&[[2.2]]), Vec2d(vec![vec![1.1 + 2.2]]));
        assert_eq!(Slice2d::new(
            &[
                [1, 2, 3],
                [4, 5, 6],
            ]
        ) + Slice2d::new(
            &[
                [ 7,  8,  9],
                [10, 11, 12],
            ]
        ), Vec2d(
            vec![
                vec![ 8, 10, 12],
                vec![14, 16, 18],
            ]
        ));

        assert_eq!(Slice2d::new(
            &[
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        ).slice((..2, ..3)) + Slice2d::new(
            &[
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        ).slice((2.., 1..)), Vec2d(
            vec![
                vec![11, 13, 15],
                vec![19, 21, 23],
            ]
        ));
    }

    #[test]
    fn slice2d_add_overloading() {
        assert_eq!(Slice2d::new(&[[1]]) + Slice2d::new(&[[2]]), Vec2d(vec![vec![3]]));
        assert_eq!(Slice2d::new(&[[1]]) + &Slice2d::new(&[[2]]), Vec2d(vec![vec![3]]));
        assert_eq!(&Slice2d::new(&[[1]]) + Slice2d::new(&[[2]]), Vec2d(vec![vec![3]]));
        assert_eq!(&Slice2d::new(&[[1]]) + &Slice2d::new(&[[2]]), Vec2d(vec![vec![3]]));
        assert_eq!(&Slice2d::new(&[[1]]) + &Slice2d::new(&[[2]]) + &Slice2d::new(&[[3]]) + &Slice2d::new(&[[4]]), Vec2d(vec![vec![10]]));
        let a = &mut Slice2d::new(&[[1]]);
        let b = &mut Slice2d::new(&[[2]]);
        assert_eq!(&*a + &*b, Vec2d(vec![vec![3]]));
    }

    #[test]
    fn slice2d_sub() {
        assert_eq!(Slice2d::<[i32; 0]>::new(&[]) - Slice2d::<[i32; 0]>::new(&[]), Vec2d::<i32>(vec![]));
        assert_eq!(Slice2d::new(&[[1]]) - Slice2d::new(&[[2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(&Slice2d::new(&[[1]]) - &Slice2d::new(&[[2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(Slice2d::new(&[[1.1]]) - Slice2d::new(&[[2.2]]), Vec2d(vec![vec![1.1 - 2.2]]));
        assert_eq!(Slice2d::new(
            &[
                [1, 2, 3],
                [4, 5, 6],
            ]
        ) - Slice2d::new(
            &[
                [ 7,  8,  9],
                [10, 11, 12],
            ]
        ), Vec2d(
            vec![
                vec![-6, -6, -6],
                vec![-6, -6, -6],
            ]
        ));

        assert_eq!(Slice2d::new(
            &[
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        ).slice((..2, ..3)) - Slice2d::new(
            &[
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        ).slice((2.., 1..)), Vec2d(
            vec![
                vec![-9, -9, -9],
                vec![-9, -9, -9],
            ]
        ));
    }

    #[test]
    fn slice2d_sub_overloading() {
        assert_eq!(Slice2d::new(&[[1]]) - Slice2d::new(&[[2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(Slice2d::new(&[[1]]) - &Slice2d::new(&[[2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(&Slice2d::new(&[[1]]) - Slice2d::new(&[[2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(&Slice2d::new(&[[1]]) - &Slice2d::new(&[[2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(&Slice2d::new(&[[1]]) - &Slice2d::new(&[[2]]) - &Slice2d::new(&[[3]]) - &Slice2d::new(&[[4]]), Vec2d(vec![vec![-8]]));
        let a = &mut Slice2d::new(&[[1]]);
        let b = &mut Slice2d::new(&[[2]]);
        assert_eq!(&*a - &*b, Vec2d(vec![vec![-1]]));
    }
}