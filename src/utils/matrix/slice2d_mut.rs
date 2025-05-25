use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, AddAssign, Index, IndexMut, Range, Sub, SubAssign};
use crate::utils::matrix::{Shape, Slice2d, Vec2d};
use crate::utils::ops::{IntoRange, Len, Slice, SliceMut};

pub struct Slice2dMut<'a, T: 'a> {
    slice: &'a mut [T],
    row: Range<usize>,
    col: Range<usize>,
}

impl<'a, T> Slice2dMut<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self
    where
        T: Len,
    {
        let row_len = slice.len();
        let col_len = if row_len == 0 { 0 } else { slice[0].len() };
        Slice2dMut {
            slice,
            row: 0..row_len,
            col: 0..col_len,
        }
    }

    pub fn as_slice2d(&self) -> Slice2d<T>
    where
        T: Len,
    {
        Slice2d::new(self.slice).slice((self.row.clone(), self.col.clone()))
    }

    pub fn to_vec2d(&self) -> Vec2d<<T as Index<usize>>::Output>
    where
        T: Index<usize> + Len,
        <T as Index<usize>>::Output: Copy,
    {
        self.as_slice2d().to_vec2d()
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

impl<'a, 'b, T, U> PartialEq<Slice2dMut<'b, U>> for Slice2dMut<'a, T>
where
    T: Index<usize>,
    <T as Index<usize>>::Output: PartialEq<<U as Index<usize>>::Output>,
    U: Index<usize>,
{
    fn eq(&self, other: &Slice2dMut<'b, U>) -> bool {
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

impl<'a, T> Shape for Slice2dMut<'a, T> {
    fn shape(&self) -> (usize, usize) {
        (self.row.len(), self.col.len())
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

impl<'a, T> Index<(usize, usize)> for &Slice2dMut<'a, T>
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

impl<'a, T> IndexMut<(usize, usize)> for Slice2dMut<'a, T>
where
    T: IndexMut<usize>,
{
    fn index_mut(&mut self, (row_idx, col_idx): (usize, usize)) -> &mut Self::Output {
        assert!(row_idx < self.row.len(), "row index out of bounds: the len is {} but the index is {}", self.row.len(), row_idx);
        assert!(col_idx < self.col.len(), "column index out of bounds: the len is {} but the index is {}", self.col.len(), col_idx);
        &mut self.slice[self.row.start + row_idx][self.col.start + col_idx]
    }
}

impl<'a, 'b, T, Rng1, Rng2> Slice<'b, (Rng1, Rng2)> for Slice2dMut<'a, T>
where
    T: 'b + Len,
    Rng1: IntoRange<usize>,
    Rng2: IntoRange<usize>,
{
    type Output = Slice2d<'b, T>;

    fn slice(&'b self, range: (Rng1, Rng2)) -> Self::Output {
        self.as_slice2d().slice(range)
    }
}

impl<'a, 'b, T, Rng1, Rng2> SliceMut<'b, (Rng1, Rng2)> for Slice2dMut<'a, T>
where
    T: 'b,
    Rng1: IntoRange<usize>,
    Rng2: IntoRange<usize>,
{
    type Output = Slice2dMut<'b, T>;

    fn slice_mut(&'b mut self, (row, col): (Rng1, Rng2)) -> Self::Output {
        let row = row.into_range(0..self.row.len());
        let col = col.into_range(0..self.col.len());
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

impl<'a, T, Rhs> Add<Rhs> for Slice2dMut<'a, T>
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

impl<'a, T, Rhs> Add<Rhs> for &Slice2dMut<'a, T>
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

impl<'a, T, Rhs> AddAssign<Rhs> for Slice2dMut<'a, T>
where
    T: IndexMut<usize>,
    <T as Index<usize>>::Output: AddAssign<<Rhs as Index<(usize, usize)>>::Output>,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    #[inline(always)]
    fn add_assign(mut self: &mut Self, rhs: Rhs) {
        self += rhs
    }
}

impl<'a, T, Rhs> AddAssign<Rhs> for &mut Slice2dMut<'a, T>
where
    T: IndexMut<usize>,
    <T as Index<usize>>::Output: AddAssign<<Rhs as Index<(usize, usize)>>::Output>,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        for i in 0..row_len {
            for j in 0..col_len {
                self.slice[self.row.start + i][self.col.start + j] += rhs[(i, j)];
            }
        }
    }
}

impl<'a, T, Rhs> Sub<Rhs> for Slice2dMut<'a, T>
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

impl<'a, T, Rhs> Sub<Rhs> for &Slice2dMut<'a, T>
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

impl<'a, T, Rhs> SubAssign<Rhs> for Slice2dMut<'a, T>
where
    T: IndexMut<usize>,
    <T as Index<usize>>::Output: SubAssign<<Rhs as Index<(usize, usize)>>::Output>,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    #[inline(always)]
    fn sub_assign(mut self: &mut Self, rhs: Rhs) {
        self -= rhs
    }
}

impl<'a, T, Rhs> SubAssign<Rhs> for &mut Slice2dMut<'a, T>
where
    T: IndexMut<usize>,
    <T as Index<usize>>::Output: SubAssign<<Rhs as Index<(usize, usize)>>::Output>,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        for i in 0..row_len {
            for j in 0..col_len {
                self.slice[self.row.start + i][self.col.start + j] -= rhs[(i, j)];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice2d_mut_debug() {
        assert_eq!(format!("{:?}", Slice2dMut::<[i32; 0]>::new(&mut [])), String::from("Slice2dMut([])"));
        assert_eq!(format!("{:?}", Slice2dMut::new(&mut [[1]])), String::from("Slice2dMut([[1]])"));
        let mut a = [[1, 2], [3, 4]];
        let mut a = Slice2dMut::new(&mut a);
        assert_eq!(format!("{:?}", a), String::from("Slice2dMut([[1, 2], [3, 4]])"));
        assert_eq!(format!("{:?}", &a), String::from("Slice2dMut([[1, 2], [3, 4]])"));
        assert_eq!(format!("{:?}", &mut a), String::from("Slice2dMut([[1, 2], [3, 4]])"));
    }

    #[test]
    fn slice2d_mut_partial_eq() {
        assert_eq!(Slice2dMut::new(&mut [[1]]), Slice2dMut::new(&mut [[1]]));
        assert_eq!(&Slice2dMut::new(&mut [[1]]), &Slice2dMut::new(&mut [[1]]));
        assert_eq!(&Slice2dMut::new(&mut [[1]]), &mut Slice2dMut::new(&mut [[1]]));
        assert_eq!(&mut Slice2dMut::new(&mut [[1]]), &Slice2dMut::new(&mut [[1]]));
        assert_eq!(&mut Slice2dMut::new(&mut [[1]]), &mut Slice2dMut::new(&mut [[1]]));
    }

    #[test]
    fn slice2d_mut_index_and_slice() {
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

        let mut expected = [[6, 7], [10, 11]];
        assert_eq!(a.slice((1..3, 1..3)), Slice2d::new(&mut expected));
        assert_eq!(a.slice_mut((1..3, 1..3)), Slice2dMut::new(&mut expected));
        let mut expected = [[6, 7], [10, 11]];
        assert_eq!(a.slice((1..=2, 1..=2)), Slice2d::new(&mut expected));
        assert_eq!(a.slice_mut((1..=2, 1..=2)), Slice2dMut::new(&mut expected));
        let mut expected = [[6, 7, 8], [10, 11, 12], [14, 15, 16]];
        assert_eq!(a.slice((1.., 1..)), Slice2d::new(&mut expected));
        assert_eq!(a.slice_mut((1.., 1..)), Slice2dMut::new(&mut expected));
        let mut expected = [[1, 2, 3], [5, 6, 7], [9, 10, 11]];
        assert_eq!(a.slice((..3, ..3)), Slice2d::new(&mut expected));
        assert_eq!(a.slice_mut((..3, ..3)), Slice2dMut::new(&mut expected));
        let mut expected = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]];
        assert_eq!(a.slice((.., ..)), Slice2d::new(&mut expected));
        assert_eq!(a.slice_mut((.., ..)), Slice2dMut::new(&mut expected));

        let mut a = a.slice_mut((1..3, 1..));
        let mut b = [
            [ 6,  7,  8, 0],
            [10, 11, 12, 0],
        ];
        let mut b = Slice2dMut::new(&mut b);
        let mut b = b.slice_mut((.., ..3));
        assert_eq!(a, b);

        assert_eq!(a[(0, 0)], 6);
        assert_eq!(a[(1, 2)], 12);

        assert_eq!(a.slice((1..2, 2..3)), b.slice((1..2, 2..3)));
        assert_eq!(a.slice_mut((1..2, 2..3)), b.slice_mut((1..2, 2..3)));
        assert_eq!(a.slice((1..=1, 2..=2)), b.slice((1..=1, 2..=2)));
        assert_eq!(a.slice_mut((1..=1, 2..=2)), b.slice_mut((1..=1, 2..=2)));
        assert_eq!(a.slice((1.., 2..)), b.slice((1.., 2..)));
        assert_eq!(a.slice_mut((1.., 2..)), b.slice_mut((1.., 2..)));
        assert_eq!(a.slice((..2, ..3)), b.slice((..2, ..3)));
        assert_eq!(a.slice_mut((..2, ..3)), b.slice_mut((..2, ..3)));
        assert_eq!(a.slice((.., ..)), b.slice((.., ..)));
        assert_eq!(a.slice_mut((.., ..)), b.slice_mut((.., ..)));
    }

    #[test]
    fn slice2d_mut_index_mut() {
        let mut a = [
            [ 1,  2,  3,  4],
            [ 5,  6,  7,  8],
            [ 9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        let mut a = Slice2dMut::new(&mut a);

        a[(0, 0)] += 100;
        a[(1, 2)] += 100;
        a[(3, 3)] += 100;
        assert_eq!(a[(0, 0)], 101);
        assert_eq!(a[(1, 2)], 107);
        assert_eq!(a[(3, 3)], 116);

        let mut a = a.slice_mut((1..3, 1..));

        a[(0, 0)] += 100;
        a[(0, 1)] += 100;
        a[(1, 2)] += 100;
        assert_eq!(a[(0, 0)], 106);
        assert_eq!(a[(0, 1)], 207);
        assert_eq!(a[(1, 2)], 112);
    }

    #[test]
    fn slice2d_mut_add() {
        let mut a = [];
        let mut b = [];
        assert_eq!(Slice2dMut::<[i32; 0]>::new(&mut a) + Slice2dMut::<[i32; 0]>::new(&mut b), Vec2d::<i32>(vec![]));

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
        assert_eq!(Slice2dMut::new(&mut a).slice_mut((..2, ..3)) + Slice2dMut::new(&mut b).slice_mut((2.., 1..)), Vec2d(
            vec![
                vec![11, 13, 15],
                vec![19, 21, 23],
            ]
        ));
    }

    #[test]
    fn slice2d_mut_add_overloading() {
        let mut a = [[1]];
        let mut b = [[2]];
        assert_eq!(Slice2dMut::new(&mut a) + Slice2dMut::new(&mut b), Vec2d(vec![vec![3]]));
        assert_eq!(Slice2dMut::new(&mut a) + &Slice2dMut::new(&mut b), Vec2d(vec![vec![3]]));
        assert_eq!(&Slice2dMut::new(&mut a) + Slice2dMut::new(&mut b), Vec2d(vec![vec![3]]));
        assert_eq!(&Slice2dMut::new(&mut a) + &Slice2dMut::new(&mut b), Vec2d(vec![vec![3]]));
        let mut c = [[3]];
        let mut d = [[4]];
        assert_eq!(&Slice2dMut::new(&mut a) + &Slice2dMut::new(&mut b) + &Slice2dMut::new(&mut c) + &Slice2dMut::new(&mut d), Vec2d(vec![vec![10]]));
        let a = &mut Slice2dMut::new(&mut a);
        let b = &mut Slice2dMut::new(&mut b);
        assert_eq!(&*a + &*b, Vec2d(vec![vec![3]]));
    }

    #[test]
    fn slice2d_mut_add_assign() {
        let mut a = [];
        let mut b = [];
        let mut expected = [];
        let mut a = Slice2dMut::<[i32; 0]>::new(&mut a);
        a += Slice2dMut::<[i32; 0]>::new(&mut b);
        assert_eq!(a, Slice2dMut::<[i32; 0]>::new(&mut expected));

        let mut a = [[1]];
        let mut b = [[2]];
        let mut expected = [[3]];
        let mut a = Slice2dMut::new(&mut a);
        a += Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

        let mut a = [[1.1]];
        let mut b = [[2.2]];
        let mut expected = [[1.1 + 2.2]];
        let mut a = Slice2dMut::new(&mut a);
        a += Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

        let mut a = [
            [1, 2, 3],
            [4, 5, 6],
        ];
        let mut b = [
            [ 7,  8,  9],
            [10, 11, 12],
        ];
        let mut expected = [
            [ 8, 10, 12],
            [14, 16, 18],
        ];
        let mut a = Slice2dMut::new(&mut a);
        a += Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

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
        let mut expected = [
            [11, 13, 15, 0],
            [19, 21, 23, 0],
        ];
        let mut a = Slice2dMut::new(&mut a);
        let mut a = a.slice_mut((..2, ..3));
        let mut b = Slice2dMut::new(&mut b);
        a += b.slice_mut((2.., 1..));
        assert_eq!(a, Slice2dMut::new(&mut expected).slice_mut((.., ..3)));
    }

    #[test]
    fn slice2d_mut_add_assign_overloading() {
        let mut a = [[1]];
        let mut b = [[2]];
        let mut expected = [[3]];
        let mut a = Slice2dMut::new(&mut a);
        a += Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

        let mut a = [[1]];
        let mut a = Slice2dMut::new(&mut a);
        a += &Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

        let mut a = [[1]];
        let mut a = &mut Slice2dMut::new(&mut a);
        a += Slice2dMut::new(&mut b);
        assert_eq!(a, &mut Slice2dMut::new(&mut expected));

        let mut a = [[1]];
        let mut a = &mut Slice2dMut::new(&mut a);
        a += &Slice2dMut::new(&mut b);
        assert_eq!(a, &mut Slice2dMut::new(&mut expected));

        let mut a = [[1]];
        let mut c = [[3]];
        let mut d = [[4]];
        let mut expected = [[10]];
        let mut a = Slice2dMut::new(&mut a);
        a += &Slice2dMut::new(&mut b) + &Slice2dMut::new(&mut c) + &Slice2dMut::new(&mut d);
        assert_eq!(a, Slice2dMut::new(&mut expected));
    }

    #[test]
    fn slice2d_mut_sub() {
        let mut a = [];
        let mut b = [];
        assert_eq!(Slice2dMut::<[i32; 0]>::new(&mut a) - Slice2dMut::<[i32; 0]>::new(&mut b), Vec2d::<i32>(vec![]));

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
        assert_eq!(Slice2dMut::new(&mut a).slice_mut((..2, ..3)) - Slice2dMut::new(&mut b).slice_mut((2.., 1..)), Vec2d(
            vec![
                vec![-9, -9, -9],
                vec![-9, -9, -9],
            ]
        ));
    }

    #[test]
    fn slice2d_mut_sub_overloading() {
        let mut a = [[1]];
        let mut b = [[2]];
        assert_eq!(Slice2dMut::new(&mut a) - Slice2dMut::new(&mut b), Vec2d(vec![vec![-1]]));
        assert_eq!(Slice2dMut::new(&mut a) - &Slice2dMut::new(&mut b), Vec2d(vec![vec![-1]]));
        assert_eq!(&Slice2dMut::new(&mut a) - Slice2dMut::new(&mut b), Vec2d(vec![vec![-1]]));
        assert_eq!(&Slice2dMut::new(&mut a) - &Slice2dMut::new(&mut b), Vec2d(vec![vec![-1]]));
        let mut c = [[3]];
        let mut d = [[4]];
        assert_eq!(&Slice2dMut::new(&mut a) - &Slice2dMut::new(&mut b) - &Slice2dMut::new(&mut c) - &Slice2dMut::new(&mut d), Vec2d(vec![vec![-8]]));
        let a = &mut Slice2dMut::new(&mut a);
        let b = &mut Slice2dMut::new(&mut b);
        assert_eq!(&*a - &*b, Vec2d(vec![vec![-1]]));
    }

    #[test]
    fn slice2d_mut_sub_assign() {
        let mut a = [];
        let mut b = [];
        let mut expected = [];
        let mut a = Slice2dMut::<[i32; 0]>::new(&mut a);
        a -= Slice2dMut::<[i32; 0]>::new(&mut b);
        assert_eq!(a, Slice2dMut::<[i32; 0]>::new(&mut expected));

        let mut a = [[1]];
        let mut b = [[2]];
        let mut expected = [[-1]];
        let mut a = Slice2dMut::new(&mut a);
        a -= Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

        let mut a = [[1.1]];
        let mut b = [[2.2]];
        let mut expected = [[1.1 - 2.2]];
        let mut a = Slice2dMut::new(&mut a);
        a -= Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

        let mut a = [
            [1, 2, 3],
            [4, 5, 6],
        ];
        let mut b = [
            [ 7,  8,  9],
            [10, 11, 12],
        ];
        let mut expected = [
            [-6, -6, -6],
            [-6, -6, -6],
        ];
        let mut a = Slice2dMut::new(&mut a);
        a -= Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

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
        let mut expected = [
            [-9, -9, -9, 0],
            [-9, -9, -9, 0],
        ];
        let mut a = Slice2dMut::new(&mut a);
        let mut a = a.slice_mut((..2, ..3));
        let mut b = Slice2dMut::new(&mut b);
        a -= b.slice_mut((2.., 1..));
        assert_eq!(a, Slice2dMut::new(&mut expected).slice_mut((.., ..3)));
    }

    #[test]
    fn slice2d_mut_sub_assign_overloading() {
        let mut a = [[1]];
        let mut b = [[2]];
        let mut expected = [[-1]];
        let mut a = Slice2dMut::new(&mut a);
        a -= Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

        let mut a = [[1]];
        let mut a = Slice2dMut::new(&mut a);
        a -= &Slice2dMut::new(&mut b);
        assert_eq!(a, Slice2dMut::new(&mut expected));

        let mut a = [[1]];
        let mut a = &mut Slice2dMut::new(&mut a);
        a -= Slice2dMut::new(&mut b);
        assert_eq!(a, &mut Slice2dMut::new(&mut expected));

        let mut a = [[1]];
        let mut a = &mut Slice2dMut::new(&mut a);
        a -= &Slice2dMut::new(&mut b);
        assert_eq!(a, &mut Slice2dMut::new(&mut expected));

        let mut a = [[1]];
        let mut c = [[3]];
        let mut d = [[4]];
        let mut expected = [[6]];
        let mut a = Slice2dMut::new(&mut a);
        a -= &Slice2dMut::new(&mut b) - &Slice2dMut::new(&mut c) - &Slice2dMut::new(&mut d);
        assert_eq!(a, Slice2dMut::new(&mut expected));
    }
}