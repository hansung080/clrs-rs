use std::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Sub, SubAssign};
use crate::utils::matrix::{Shape, Slice2d, Slice2dMut, Vec2d};
use crate::utils::ops::{IntoRange, Slice};

#[derive(Debug, PartialEq, Clone)]
pub struct Mat<T, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<T, const M: usize, const N: usize> Mat<T, M, N> {
    pub fn as_slice2d(&self) -> Slice2d<[T; N]> {
        Slice2d::new(&self.0)
    }

    pub fn as_slice2d_mut(&mut self) -> Slice2dMut<[T; N]> {
        Slice2dMut::new(&mut self.0)
    }

    pub fn to_vec2d(&self) -> Vec2d<T>
    where
        T: Copy,
    {
        self.as_slice2d().to_vec2d()
    }
}

impl<T, const M: usize, const N: usize> Deref for Mat<T, M, N> {
    type Target = [[T; N]; M];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const M: usize, const N: usize> DerefMut for Mat<T, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const M: usize, const N: usize> Shape for Mat<T, M, N> {
    fn shape(&self) -> (usize, usize) {
        (M, N)
    }
}

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Mat<T, M, N> {
    type Output = T;

    fn index(&self, (row_idx, col_idx): (usize, usize)) -> &Self::Output {
        &self.0[row_idx][col_idx]
    }
}

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for &Mat<T, M, N> {
    type Output = T;

    fn index(&self, (row_idx, col_idx): (usize, usize)) -> &Self::Output {
        &self.0[row_idx][col_idx]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Mat<T, M, N> {
    fn index_mut(&mut self, (row_idx, col_idx): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row_idx][col_idx]
    }
}

impl<'a, T, const M: usize, const N: usize, Rng1, Rng2> Slice<'a, (Rng1, Rng2)> for Mat<T, M, N>
where
    T: 'a,
    Rng1: IntoRange<usize>,
    Rng2: IntoRange<usize>,
{
    type Output = Slice2d<'a, [T; N]>;

    fn slice(&'a self, range: (Rng1, Rng2)) -> Self::Output {
        self.as_slice2d().slice(range)
    }
}

impl<T, const M: usize, const N: usize, Rhs> Add<Rhs> for Mat<T, M, N>
where
    T: Add<Output = T> + Default + Copy,
    Rhs: AsRef<Mat<T, M, N>>,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Rhs) -> Self::Output {
        &self + rhs
    }
}

impl<T, const M: usize, const N: usize, Rhs> Add<Rhs> for &Mat<T, M, N>
where
    T: Add<Output = T> + Default + Copy,
    Rhs: AsRef<Mat<T, M, N>>,
{
    type Output = Mat<T, M, N>;

    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.as_ref();
        let mut result = Mat([[T::default(); N]; M]);
        for i in 0..M {
            for j in 0..N {
                result[(i, j)] = self[(i, j)] + rhs[(i, j)];
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize, Rhs> AddAssign<Rhs> for Mat<T, M, N>
where
    T: AddAssign + Copy,
    Rhs: AsRef<Mat<T, M, N>>,
{
    #[inline(always)]
    fn add_assign(mut self: &mut Self, rhs: Rhs) {
        self += rhs;
    }
}

impl<T, const M: usize, const N: usize, Rhs> AddAssign<Rhs> for &mut Mat<T, M, N>
where
    T: AddAssign + Copy,
    Rhs: AsRef<Mat<T, M, N>>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.as_ref();
        for i in 0..M {
            for j in 0..N {
                self[(i, j)] += rhs[(i, j)];
            }
        }
    }
}

impl<T, const M: usize, const N: usize, Rhs> Sub<Rhs> for Mat<T, M, N>
where
    T: Sub<Output = T> + Default + Copy,
    Rhs: AsRef<Mat<T, M, N>>,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Rhs) -> Self::Output {
        &self - rhs
    }
}

impl<T, const M: usize, const N: usize, Rhs> Sub<Rhs> for &Mat<T, M, N>
where
    T: Sub<Output = T> + Default + Copy,
    Rhs: AsRef<Mat<T, M, N>>,
{
    type Output = Mat<T, M, N>;

    fn sub(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.as_ref();
        let mut result = Mat([[T::default(); N]; M]);
        for i in 0..M {
            for j in 0..N {
                result[(i, j)] = self[(i, j)] - rhs[(i, j)];
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize, Rhs> SubAssign<Rhs> for Mat<T, M, N>
where
    T: SubAssign + Copy,
    Rhs: AsRef<Mat<T, M, N>>,
{
    #[inline(always)]
    fn sub_assign(mut self: &mut Self, rhs: Rhs) {
        self -= rhs;
    }
}

impl<T, const M: usize, const N: usize, Rhs> SubAssign<Rhs> for &mut Mat<T, M, N>
where
    T: SubAssign + Copy,
    Rhs: AsRef<Mat<T, M, N>>,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.as_ref();
        for i in 0..M {
            for j in 0..N {
                self[(i, j)] -= rhs[(i, j)];
            }
        }
    }
}

impl<T, const M: usize, const N: usize> AsRef<Mat<T, M, N>> for Mat<T, M, N> {
    fn as_ref(&self) -> &Mat<T, M, N> {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ops::SliceMut;
    use super::*;

    #[test]
    fn mat_debug() {
        assert_eq!(format!("{:?}", Mat::<i32, 0, 0>([])), String::from("Mat([])"));
        assert_eq!(format!("{:?}", Mat([[1]])), String::from("Mat([[1]])"));
        let mut a = Mat([[1, 2], [3, 4]]);
        assert_eq!(format!("{:?}", a), String::from("Mat([[1, 2], [3, 4]])"));
        assert_eq!(format!("{:?}", &a), String::from("Mat([[1, 2], [3, 4]])"));
        assert_eq!(format!("{:?}", &mut a), String::from("Mat([[1, 2], [3, 4]])"));
    }

    #[test]
    fn mat_partial_eq() {
        assert_eq!(Mat([[1]]), Mat([[1]]));
        assert_eq!(&Mat([[1]]), &Mat([[1]]));
        assert_eq!(&Mat([[1]]), &mut Mat([[1]]));
        assert_eq!(&mut Mat([[1]]), &Mat([[1]]));
        assert_eq!(&mut Mat([[1]]), &mut Mat([[1]]));
    }

    #[test]
    fn mat_index_and_slice() {
        let a = Mat(
            [
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
        let b = Mat(
            [
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
    fn mat_index_mut() {
        let mut a = Mat(
            [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
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
    fn mat_add() {
        assert_eq!(Mat::<i32, 0, 0>([]) + Mat::<i32, 0, 0>([]), Mat::<i32, 0, 0>([]));
        assert_eq!(Mat([[1]]) + Mat([[2]]), Mat([[3]]));
        assert_eq!(Mat([[1.1]]) + Mat([[2.2]]), Mat([[1.1 + 2.2]]));
        assert_eq!(Mat(
            [
                [1, 2, 3],
                [4, 5, 6],
            ]
        ) + Mat(
            [
                [ 7,  8,  9],
                [10, 11, 12],
            ]
        ), Mat(
            [
                [ 8, 10, 12],
                [14, 16, 18],
            ]
        ));

        assert_eq!(Mat(
            [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        ).slice((..2, ..3)) + Mat(
            [
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
    fn mat_add_overloading() {
        assert_eq!(Mat([[1]]) + Mat([[2]]), Mat([[3]]));
        assert_eq!(Mat([[1]]) + &Mat([[2]]), Mat([[3]]));
        assert_eq!(&Mat([[1]]) + Mat([[2]]), Mat([[3]]));
        assert_eq!(&Mat([[1]]) + &Mat([[2]]), Mat([[3]]));
        assert_eq!(&Mat([[1]]) + &Mat([[2]]) + &Mat([[3]]) + &Mat([[4]]), Mat([[10]]));
        assert_eq!(&Mat([[1]]).slice((.., ..)) + &Mat([[2]]) + &Mat([[3]]) + &Mat([[4]]), Vec2d(vec![vec![10]]));
        let a = &mut Mat([[1]]);
        let b = &mut Mat([[2]]);
        assert_eq!(&*a + &*b, Mat([[3]]));
    }

    #[test]
    fn mat_add_assign() {
        let mut a = Mat::<i32, 0, 0>([]);
        a += Mat::<i32, 0, 0>([]);
        assert_eq!(a, Mat::<i32, 0, 0>([]));

        let mut a = Mat([[1]]);
        a += Mat([[2]]);
        assert_eq!(a, Mat([[3]]));

        let mut a = Mat([[1.1]]);
        a += Mat([[2.2]]);
        assert_eq!(a, Mat([[1.1 + 2.2]]));

        let mut a = Mat(
            [
                [1, 2, 3],
                [4, 5, 6],
            ]
        );
        a += Mat(
            [
                [ 7,  8,  9],
                [10, 11, 12],
            ]
        );
        assert_eq!(a, Mat(
            [
                [ 8, 10, 12],
                [14, 16, 18],
            ]
        ));

        let mut a = Mat(
            [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        );
        let mut a = a.as_slice2d_mut();
        let mut a = a.slice_mut((..2, ..3));
        a += Mat(
            [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
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
    fn mat_add_assign_overloading() {
        let mut a = Mat([[1]]);
        a += Mat([[2]]);
        assert_eq!(a, Mat([[3]]));

        let mut a = Mat([[1]]);
        a += &Mat([[2]]);
        assert_eq!(a, Mat([[3]]));

        let mut a = &mut Mat([[1]]);
        a += Mat([[2]]);
        assert_eq!(a, &mut Mat([[3]]));

        let mut a = &mut Mat([[1]]);
        a += &Mat([[2]]);
        assert_eq!(a, &mut Mat([[3]]));

        let mut a = Mat([[1]]);
        a += &Mat([[2]]) + &Mat([[3]]) + &Mat([[4]]);
        assert_eq!(a, Mat([[10]]));

        let mut a = Mat([[1]]);
        let mut a = a.as_slice2d_mut();
        let mut a = a.slice_mut((.., ..));
        a += &Mat([[2]]) + &Mat([[3]]) + &Mat([[4]]);
        assert_eq!(a, Slice2dMut::new(&mut [[10]]));
    }

    #[test]
    fn mat_sub() {
        assert_eq!(Mat::<i32, 0, 0>([]) - Mat::<i32, 0, 0>([]), Mat::<i32, 0, 0>([]));
        assert_eq!(Mat([[1]]) - Mat([[2]]), Mat([[-1]]));
        assert_eq!(Mat([[1.1]]) - Mat([[2.2]]), Mat([[1.1 - 2.2]]));
        assert_eq!(Mat(
            [
                [1, 2, 3],
                [4, 5, 6],
            ]
        ) - Mat(
            [
                [ 7,  8,  9],
                [10, 11, 12],
            ]
        ), Mat(
            [
                [-6, -6, -6],
                [-6, -6, -6],
            ]
        ));

        assert_eq!(Mat(
            [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        ).slice((..2, ..3)) - Mat(
            [
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
    fn mat_sub_overloading() {
        assert_eq!(Mat([[1]]) - Mat([[2]]), Mat([[-1]]));
        assert_eq!(Mat([[1]]) - &Mat([[2]]), Mat([[-1]]));
        assert_eq!(&Mat([[1]]) - Mat([[2]]), Mat([[-1]]));
        assert_eq!(&Mat([[1]]) - &Mat([[2]]), Mat([[-1]]));
        assert_eq!(&Mat([[1]]) - &Mat([[2]]) - &Mat([[3]]) - &Mat([[4]]), Mat([[-8]]));
        assert_eq!(&Mat([[1]]).slice((.., ..)) - &Mat([[2]]) - &Mat([[3]]) - &Mat([[4]]), Vec2d(vec![vec![-8]]));
        let a = &mut Mat([[1]]);
        let b = &mut Mat([[2]]);
        assert_eq!(&*a - &*b, Mat([[-1]]));
    }

    #[test]
    fn mat_sub_assign() {
        let mut a = Mat::<i32, 0, 0>([]);
        a -= Mat::<i32, 0, 0>([]);
        assert_eq!(a, Mat::<i32, 0, 0>([]));

        let mut a = Mat([[1]]);
        a -= Mat([[2]]);
        assert_eq!(a, Mat([[-1]]));

        let mut a = Mat([[1.1]]);
        a -= Mat([[2.2]]);
        assert_eq!(a, Mat([[1.1 - 2.2]]));

        let mut a = Mat(
            [
                [1, 2, 3],
                [4, 5, 6],
            ]
        );
        a -= Mat(
            [
                [ 7,  8,  9],
                [10, 11, 12],
            ]
        );
        assert_eq!(a, Mat(
            [
                [-6, -6, -6],
                [-6, -6, -6],
            ]
        ));

        let mut a = Mat(
            [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
            ]
        );
        let mut a = a.as_slice2d_mut();
        let mut a = a.slice_mut((..2, ..3));
        a -= Mat(
            [
                [ 1,  2,  3,  4],
                [ 5,  6,  7,  8],
                [ 9, 10, 11, 12],
                [13, 14, 15, 16],
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
    fn mat_sub_assign_overloading() {
        let mut a = Mat([[1]]);
        a -= Mat([[2]]);
        assert_eq!(a, Mat([[-1]]));

        let mut a = Mat([[1]]);
        a -= &Mat([[2]]);
        assert_eq!(a, Mat([[-1]]));

        let mut a = &mut Mat([[1]]);
        a -= Mat([[2]]);
        assert_eq!(a, &mut Mat([[-1]]));

        let mut a = &mut Mat([[1]]);
        a -= &Mat([[2]]);
        assert_eq!(a, &mut Mat([[-1]]));

        let mut a = Mat([[1]]);
        a -= &Mat([[2]]) - &Mat([[3]]) - &Mat([[4]]);
        assert_eq!(a, Mat([[6]]));

        let mut a = Mat([[1]]);
        let mut a = a.as_slice2d_mut();
        let mut a = a.slice_mut((.., ..));
        a -= &Mat([[2]]) - &Mat([[3]]) - &Mat([[4]]);
        assert_eq!(a, Slice2dMut::new(&mut [[6]]));
    }
}