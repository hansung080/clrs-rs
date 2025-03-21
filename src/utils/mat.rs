use std::ops::{Add, Deref, DerefMut};

#[derive(Debug, PartialEq)]
struct Mat<T, const M: usize, const N: usize>([[T; N]; M]);

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
        let mut result = [[T::default(); N]; M];
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self[i][j] + rhs[i][j];
            }
        }
        Mat(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat_add() {
        assert_eq!(Mat([[1]]) + Mat([[2]]), Mat([[3]]));
    }
}

