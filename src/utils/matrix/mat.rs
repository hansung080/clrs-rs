use std::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat<T, const M: usize, const N: usize>(pub [[T; N]; M]);

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

impl<T, const M: usize, const N: usize> Add for Mat<T, M, N>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Mat([[T::default(); N]; M]);
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self[i][j] + rhs[i][j];
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> AddAssign for Mat<T, M, N>
where
    T: AddAssign + Copy,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self[i][j] += rhs[i][j];
            }
        }
    }
}

impl<T, const M: usize, const N: usize> Sub for Mat<T, M, N>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Mat([[T::default(); N]; M]);
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self[i][j] - rhs[i][j];
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> SubAssign for Mat<T, M, N>
where
    T: SubAssign + Copy,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                self[i][j] -= rhs[i][j];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat_add() {
        assert_eq!(Mat([]) + Mat([]), Mat::<i32, 0, 0>([]));
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
    }

    #[test]
    fn mat_add_assign() {
        let mut a = Mat([]);
        a += Mat([]);
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
    }

    #[test]
    fn mat_sub() {
        assert_eq!(Mat([]) - Mat([]), Mat::<i32, 0, 0>([]));
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
    }

    #[test]
    fn mat_sub_assign() {
        let mut a = Mat([]);
        a -= Mat([]);
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
    }

    #[test]
    fn mat_copy() {
        let a = Mat([[1]]);
        let b = Mat([[2]]);
        let c = a + b;
        assert_eq!(a, Mat([[1]]));
        assert_eq!(b, Mat([[2]]));
        assert_eq!(c, Mat([[3]]));

        let a = Mat([[1]]);
        let b = Mat([[2]]);
        let c = a - b;
        assert_eq!(a, Mat([[1]]));
        assert_eq!(b, Mat([[2]]));
        assert_eq!(c, Mat([[-1]]));
    }
}