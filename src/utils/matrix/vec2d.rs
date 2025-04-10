use std::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Sub, SubAssign};
use crate::utils::matrix::{Shape, Slice2d, Slice2dMut};
use crate::utils::ops::{IntoRange, Slice};

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2d<T>(pub Vec<Vec<T>>);

impl<T> Vec2d<T> {
    pub fn defaults((row_len, col_len): (usize, usize)) -> Self
    where
        T: Default,
    {
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for _ in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for _ in 0..col_len {
                result_row.push(T::default());
            }
            result.push(result_row);
        }
        result
    }

    pub fn as_slice2d(&self) -> Slice2d<Vec<T>> {
        Slice2d::new(&self.0)
    }

    pub fn as_slice2d_mut(&mut self) -> Slice2dMut<Vec<T>> {
        Slice2dMut::new(&mut self.0)
    }
}

impl<T> Deref for Vec2d<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vec2d<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Shape for Vec2d<T> {
    fn shape(&self) -> (usize, usize) {
        (self.len(), if self.len() == 0 { 0 } else { self.0[0].len() })
    }
}

impl<T> Index<(usize, usize)> for Vec2d<T> {
    type Output = T;

    fn index(&self, (row_idx, col_idx): (usize, usize)) -> &Self::Output {
        &self.0[row_idx][col_idx]
    }
}

impl<T> Index<(usize, usize)> for &Vec2d<T> {
    type Output = T;

    fn index(&self, (row_idx, col_idx): (usize, usize)) -> &Self::Output {
        &self.0[row_idx][col_idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, (row_idx, col_idx): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row_idx][col_idx]
    }
}

impl<'a, T, Rng1, Rng2> Slice<'a, (Rng1, Rng2)> for Vec2d<T>
where
    T: 'a,
    Rng1: IntoRange<usize>,
    Rng2: IntoRange<usize>,
{
    type Output = Slice2d<'a, Vec<T>>;

    fn slice(&'a self, range: (Rng1, Rng2)) -> Self::Output {
        self.as_slice2d().slice(range)
    }
}

impl<T, Rhs> Add<Rhs> for Vec2d<T>
where
    T: Add<<Rhs as Index<(usize, usize)>>::Output, Output = T> + Copy,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Rhs) -> Self::Output {
        &self + rhs
    }
}

impl<T, Rhs> Add<Rhs> for &Vec2d<T>
where
    T: Add<<Rhs as Index<(usize, usize)>>::Output, Output = T> + Copy,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    type Output = Vec2d<T>;

    fn add(self, rhs: Rhs) -> Self::Output {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for i in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for j in 0..col_len {
                result_row.push(self[(i, j)] + rhs[(i, j)]);
            }
            result.push(result_row);
        }
        result
    }
}

impl<T, Rhs> AddAssign<Rhs> for Vec2d<T>
where
    T: AddAssign<<Rhs as Index<(usize, usize)>>::Output>,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    #[inline(always)]
    fn add_assign(mut self: &mut Self, rhs: Rhs) {
        self += rhs;
    }
}

impl<T, Rhs> AddAssign<Rhs> for &mut Vec2d<T>
where
    T: AddAssign<<Rhs as Index<(usize, usize)>>::Output>,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        for i in 0..row_len {
            for j in 0..col_len {
                self[(i, j)] += rhs[(i, j)];
            }
        }
    }
}

impl<T, Rhs> Sub<Rhs> for Vec2d<T>
where
    T: Sub<<Rhs as Index<(usize, usize)>>::Output, Output = T> + Copy,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Rhs) -> Self::Output {
        &self - rhs
    }
}

impl<T, Rhs> Sub<Rhs> for &Vec2d<T>
where
    T: Sub<<Rhs as Index<(usize, usize)>>::Output, Output = T> + Copy,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    type Output = Vec2d<T>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for i in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for j in 0..col_len {
                result_row.push(self[(i, j)] - rhs[(i, j)]);
            }
            result.push(result_row);
        }
        result
    }
}

impl<T, Rhs> SubAssign<Rhs> for Vec2d<T>
where
    T: SubAssign<<Rhs as Index<(usize, usize)>>::Output>,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    #[inline(always)]
    fn sub_assign(mut self: &mut Self, rhs: Rhs) {
        self -= rhs;
    }
}

impl<T, Rhs> SubAssign<Rhs> for &mut Vec2d<T>
where
    T: SubAssign<<Rhs as Index<(usize, usize)>>::Output>,
    Rhs: Shape + Index<(usize, usize)>,
    <Rhs as Index<(usize, usize)>>::Output: Copy,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        for i in 0..row_len {
            for j in 0..col_len {
                self[(i, j)] -= rhs[(i, j)];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ops::SliceMut;
    use super::*;

    #[test]
    fn vec2d_debug() {
        assert_eq!(format!("{:?}", Vec2d::<i32>(vec![])), String::from("Vec2d([])"));
        assert_eq!(format!("{:?}", Vec2d(vec![vec![1]])), String::from("Vec2d([[1]])"));
        let mut a = Vec2d(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(format!("{:?}", a), String::from("Vec2d([[1, 2], [3, 4]])"));
        assert_eq!(format!("{:?}", &a), String::from("Vec2d([[1, 2], [3, 4]])"));
        assert_eq!(format!("{:?}", &mut a), String::from("Vec2d([[1, 2], [3, 4]])"));
    }

    #[test]
    fn vec2d_partial_eq() {
        assert_eq!(Vec2d(vec![vec![1]]), Vec2d(vec![vec![1]]));
        assert_eq!(&Vec2d(vec![vec![1]]), &Vec2d(vec![vec![1]]));
        assert_eq!(&Vec2d(vec![vec![1]]), &mut Vec2d(vec![vec![1]]));
        assert_eq!(&mut Vec2d(vec![vec![1]]), &Vec2d(vec![vec![1]]));
        assert_eq!(&mut Vec2d(vec![vec![1]]), &mut Vec2d(vec![vec![1]]));
    }

    #[test]
    fn vec2d_index_and_slice() {
        let a = Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
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
        let b = Vec2d(
            vec![
                vec![ 6,  7,  8, 0],
                vec![10, 11, 12, 0],
            ]
        );
        let b = b.slice((.., ..3));
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
    fn vec2d_index_mut() {
        let mut a = Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        );

        a[(0, 0)] += 100;
        a[(1, 2)] += 100;
        a[(3, 3)] += 100;
        assert_eq!(a[(0, 0)], 101);
        assert_eq!(a[(1, 2)], 107);
        assert_eq!(a[(3, 3)], 116);

        let mut a = a.as_slice2d_mut();
        let mut a = a.slice_mut((1..3, 1..));

        a[(0, 0)] += 100;
        a[(0, 1)] += 100;
        a[(1, 2)] += 100;
        assert_eq!(a[(0, 0)], 106);
        assert_eq!(a[(0, 1)], 207);
        assert_eq!(a[(1, 2)], 112);
    }

    #[test]
    fn vec2d_add() {
        assert_eq!(Vec2d::<i32>(vec![]) + Vec2d::<i32>(vec![]), Vec2d::<i32>(vec![]));
        assert_eq!(Vec2d(vec![vec![1]]) + Vec2d(vec![vec![2]]), Vec2d(vec![vec![3]]));
        assert_eq!(Vec2d(vec![vec![1.1]]) + Vec2d(vec![vec![2.2]]), Vec2d(vec![vec![1.1 + 2.2]]));
        assert_eq!(Vec2d(
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
            ]
        ) + Vec2d(
            vec![
                vec![ 7,  8,  9],
                vec![10, 11, 12],
            ]
        ), Vec2d(
            vec![
                vec![ 8, 10, 12],
                vec![14, 16, 18],
            ]
        ));

        assert_eq!(Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        ).slice((..2, ..3)) + Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        ).slice((2.., 1..)), Vec2d(
            vec![
                vec![11, 13, 15],
                vec![19, 21, 23],
            ]
        ));
    }

    #[test]
    fn vec2d_add_overloading() {
        assert_eq!(Vec2d(vec![vec![1]]) + Vec2d(vec![vec![2]]), Vec2d(vec![vec![3]]));
        assert_eq!(Vec2d(vec![vec![1]]) + &Vec2d(vec![vec![2]]), Vec2d(vec![vec![3]]));
        assert_eq!(&Vec2d(vec![vec![1]]) + Vec2d(vec![vec![2]]), Vec2d(vec![vec![3]]));
        assert_eq!(&Vec2d(vec![vec![1]]) + &Vec2d(vec![vec![2]]), Vec2d(vec![vec![3]]));
        assert_eq!(&Vec2d(vec![vec![1]]) + &Vec2d(vec![vec![2]]) + &Vec2d(vec![vec![3]]) + &Vec2d(vec![vec![4]]), Vec2d(vec![vec![10]]));
        let a = &mut Vec2d(vec![vec![1]]);
        let b = &mut Vec2d(vec![vec![2]]);
        assert_eq!(&*a + &*b, Vec2d(vec![vec![3]]));
    }

    #[test]
    fn vec2d_add_assign() {
        let mut a = Vec2d::<i32>(vec![]);
        a += Vec2d::<i32>(vec![]);
        assert_eq!(a, Vec2d::<i32>(vec![]));

        let mut a = Vec2d(vec![vec![1]]);
        a += Vec2d(vec![vec![2]]);
        assert_eq!(a, Vec2d(vec![vec![3]]));

        let mut a = Vec2d(vec![vec![1.1]]);
        a += Vec2d(vec![vec![2.2]]);
        assert_eq!(a, Vec2d(vec![vec![1.1 + 2.2]]));

        let mut a = Vec2d(
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
            ]
        );
        a += Vec2d(
            vec![
                vec![ 7,  8,  9],
                vec![10, 11, 12],
            ]
        );
        assert_eq!(a, Vec2d(
            vec![
                vec![ 8, 10, 12],
                vec![14, 16, 18],
            ]
        ));

        let mut a = Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        );
        let mut a = a.as_slice2d_mut();
        let mut a = a.slice_mut((..2, ..3));
        a += Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        ).slice((2.., 1..));
        assert_eq!(a, Slice2dMut::new(
            &mut [
                [11, 13, 15],
                [19, 21, 23],
            ]
        ));
    }

    #[test]
    fn vec2d_add_assign_overloading() {
        let mut a = Vec2d(vec![vec![1]]);
        a += Vec2d(vec![vec![2]]);
        assert_eq!(a, Vec2d(vec![vec![3]]));

        let mut a = Vec2d(vec![vec![1]]);
        a += &Vec2d(vec![vec![2]]);
        assert_eq!(a, Vec2d(vec![vec![3]]));

        let mut a = &mut Vec2d(vec![vec![1]]);
        a += Vec2d(vec![vec![2]]);
        assert_eq!(a, &mut Vec2d(vec![vec![3]]));

        let mut a = &mut Vec2d(vec![vec![1]]);
        a += &Vec2d(vec![vec![2]]);
        assert_eq!(a, &mut Vec2d(vec![vec![3]]));

        let mut a = Vec2d(vec![vec![1]]);
        a += &Vec2d(vec![vec![2]]) + &Vec2d(vec![vec![3]]) + &Vec2d(vec![vec![4]]);
        assert_eq!(a, Vec2d(vec![vec![10]]));
    }

    #[test]
    fn vec2d_sub() {
        assert_eq!(Vec2d::<i32>(vec![]) - Vec2d::<i32>(vec![]), Vec2d::<i32>(vec![]));
        assert_eq!(Vec2d(vec![vec![1]]) - Vec2d(vec![vec![2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(Vec2d(vec![vec![1.1]]) - Vec2d(vec![vec![2.2]]), Vec2d(vec![vec![1.1 - 2.2]]));
        assert_eq!(Vec2d(
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
            ]
        ) - Vec2d(
            vec![
                vec![ 7,  8,  9],
                vec![10, 11, 12],
            ]
        ), Vec2d(
            vec![
                vec![-6, -6, -6],
                vec![-6, -6, -6],
            ]
        ));

        assert_eq!(Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        ).slice((..2, ..3)) - Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        ).slice((2.., 1..)), Vec2d(
            vec![
                vec![-9, -9, -9],
                vec![-9, -9, -9],
            ]
        ));
    }

    #[test]
    fn vec2d_sub_overloading() {
        assert_eq!(Vec2d(vec![vec![1]]) - Vec2d(vec![vec![2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(Vec2d(vec![vec![1]]) - &Vec2d(vec![vec![2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(&Vec2d(vec![vec![1]]) - Vec2d(vec![vec![2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(&Vec2d(vec![vec![1]]) - &Vec2d(vec![vec![2]]), Vec2d(vec![vec![-1]]));
        assert_eq!(&Vec2d(vec![vec![1]]) - &Vec2d(vec![vec![2]]) - &Vec2d(vec![vec![3]]) - &Vec2d(vec![vec![4]]), Vec2d(vec![vec![-8]]));
        let a = &mut Vec2d(vec![vec![1]]);
        let b = &mut Vec2d(vec![vec![2]]);
        assert_eq!(&*a - &*b, Vec2d(vec![vec![-1]]));
    }

    #[test]
    fn vec2d_sub_assign() {
        let mut a = Vec2d::<i32>(vec![]);
        a -= Vec2d::<i32>(vec![]);
        assert_eq!(a, Vec2d::<i32>(vec![]));

        let mut a = Vec2d(vec![vec![1]]);
        a -= Vec2d(vec![vec![2]]);
        assert_eq!(a, Vec2d(vec![vec![-1]]));

        let mut a = Vec2d(vec![vec![1.1]]);
        a -= Vec2d(vec![vec![2.2]]);
        assert_eq!(a, Vec2d(vec![vec![1.1 - 2.2]]));

        let mut a = Vec2d(
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
            ]
        );
        a -= Vec2d(
            vec![
                vec![ 7,  8,  9],
                vec![10, 11, 12],
            ]
        );
        assert_eq!(a, Vec2d(
            vec![
                vec![-6, -6, -6],
                vec![-6, -6, -6],
            ]
        ));

        let mut a = Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        );
        let mut a = a.as_slice2d_mut();
        let mut a = a.slice_mut((..2, ..3));
        a -= Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        ).slice((2.., 1..));
        assert_eq!(a, Slice2dMut::new(
            &mut [
                [-9, -9, -9],
                [-9, -9, -9],
            ]
        ));
    }

    #[test]
    fn vec2d_sub_assign_overloading() {
        let mut a = Vec2d(vec![vec![1]]);
        a -= Vec2d(vec![vec![2]]);
        assert_eq!(a, Vec2d(vec![vec![-1]]));

        let mut a = Vec2d(vec![vec![1]]);
        a -= &Vec2d(vec![vec![2]]);
        assert_eq!(a, Vec2d(vec![vec![-1]]));

        let mut a = &mut Vec2d(vec![vec![1]]);
        a -= Vec2d(vec![vec![2]]);
        assert_eq!(a, &mut Vec2d(vec![vec![-1]]));

        let mut a = &mut Vec2d(vec![vec![1]]);
        a -= &Vec2d(vec![vec![2]]);
        assert_eq!(a, &mut Vec2d(vec![vec![-1]]));

        let mut a = Vec2d(vec![vec![1]]);
        a -= &Vec2d(vec![vec![2]]) - &Vec2d(vec![vec![3]]) - &Vec2d(vec![vec![4]]);
        assert_eq!(a, Vec2d(vec![vec![6]]));
    }
}