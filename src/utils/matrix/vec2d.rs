use std::ops::{Add, AddAssign, Deref, DerefMut, Range, Sub, SubAssign};
use crate::utils::ops::{IntoRange, Slice};

#[derive(Debug, PartialEq, Clone)]
pub struct Vec2d<T>(pub Vec<Vec<T>>);

impl<T> Vec2d<T> {
    pub fn shape(&self) -> (usize, usize) {
        (self.len(), if self.len() == 0 { 0 } else { self[0].len() })
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

impl<T, Rng1, Rng2> Slice<(Rng1, Rng2)> for Vec2d<T>
where
    T: Copy,
    Rng1: IntoRange<usize>,
    Rng2: IntoRange<usize>,
{
    type Output = Self;

    fn slice(&self, (row, col): (Rng1, Rng2)) -> Self::Output {
        let (row_len, col_len) = self.shape();
        let (row, col) =
            (row.into_range(Range { start: 0, end: row_len }),
             col.into_range(Range { start: 0, end: col_len }));
        assert!(row.start <= row.end, "row range index starts at {} but ends at {}", row.start, row.end);
        assert!(col.start <= col.end, "column range index starts at {} but ends at {}", col.start, col.end);
        assert!(row.end <= row_len, "row range end index {} out of range for row of length {}", row.end, row_len);
        assert!(col.end <= col_len, "column range end index {} out of range for column of length {}", col.end, col_len);

        let mut result = Vec2d(Vec::with_capacity(row.len()));
        let col_len = col.len();
        for i in row {
            let mut result_row = Vec::with_capacity(col_len);
            for j in col.clone() {
                result_row.push(self[i][j]);
            }
            result.push(result_row);
        }
        result
    }
}

impl<T> Add for Vec2d<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for i in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for j in 0..col_len {
                result_row.push(self[i][j] + rhs[i][j]);
            }
            result.push(result_row);
        }
        result
    }
}

impl<T> AddAssign for Vec2d<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        for i in 0..row_len {
            for j in 0..col_len {
                self[i][j] += rhs[i][j];
            }
        }
    }
}

impl<T> Sub for Vec2d<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        let mut result = Vec2d(Vec::with_capacity(row_len));
        for i in 0..row_len {
            let mut result_row = Vec::with_capacity(col_len);
            for j in 0..col_len {
                result_row.push(self[i][j] - rhs[i][j]);
            }
            result.push(result_row);
        }
        result
    }
}

impl<T> SubAssign for Vec2d<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        let (row_len, col_len) = self.shape();
        assert_eq!((row_len, col_len), rhs.shape(), "mismatched shape");
        for i in 0..row_len {
            for j in 0..col_len {
                self[i][j] -= rhs[i][j];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(a[0][0], 1);
        assert_eq!(a[1][2], 7);
        assert_eq!(a[3][3], 16);

        assert_eq!(a.slice((1..3, 1..3)), Vec2d(vec![vec![6, 7], vec![10, 11]]));
        assert_eq!(a.slice((1..=2, 1..=2)), Vec2d(vec![vec![6, 7], vec![10, 11]]));
        assert_eq!(a.slice((1.., 1..)), Vec2d(vec![vec![6, 7, 8], vec![10, 11, 12], vec![14, 15, 16]]));
        assert_eq!(a.slice((..3, ..3)), Vec2d(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]]));
        assert_eq!(a.slice((.., ..)), Vec2d(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]]));

        let a = a.slice((1..3, 1..));
        let b = Vec2d(
            vec![
                vec![ 6,  7,  8],
                vec![10, 11, 12],
            ]
        );
        assert_eq!(a, b);

        assert_eq!(a[0][0], 6);
        assert_eq!(a[1][2], 12);

        assert_eq!(a.slice((1..2, 2..3)), b.slice((1..2, 2..3)));
        assert_eq!(a.slice((1..=1, 2..=2)), b.slice((1..=1, 2..=2)));
        assert_eq!(a.slice((1.., 2..)), b.slice((1.., 2..)));
        assert_eq!(a.slice((..2, ..3)), b.slice((..2, ..3)));
        assert_eq!(a.slice((.., ..)), b.slice((.., ..)));
    }

    #[test]
    fn vec2d_add() {
        assert_eq!(Vec2d(vec![]) + Vec2d(vec![]), Vec2d::<i32>(vec![]));
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
    fn vec2d_add_assign() {
        let mut a = Vec2d(vec![]);
        a += Vec2d(vec![]);
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
        ).slice((..2, ..3));
        a += Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        ).slice((2.., 1..));
        assert_eq!(a, Vec2d(
            vec![
                vec![11, 13, 15],
                vec![19, 21, 23],
            ]
        ));
    }

    #[test]
    fn vec2d_sub() {
        assert_eq!(Vec2d(vec![]) - Vec2d(vec![]), Vec2d::<i32>(vec![]));
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
    fn vec2d_sub_assign() {
        let mut a = Vec2d(vec![]);
        a -= Vec2d(vec![]);
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
        ).slice((..2, ..3));
        a -= Vec2d(
            vec![
                vec![ 1,  2,  3,  4],
                vec![ 5,  6,  7,  8],
                vec![ 9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]
        ).slice((2.., 1..));
        assert_eq!(a, Vec2d(
            vec![
                vec![-9, -9, -9],
                vec![-9, -9, -9],
            ]
        ));
    }

    #[test]
    fn vec2d_clone() {
        let a = Vec2d(vec![vec![1]]);
        let b = Vec2d(vec![vec![2]]);
        let c = a.clone() + b.clone();
        assert_eq!(a, Vec2d(vec![vec![1]]));
        assert_eq!(b, Vec2d(vec![vec![2]]));
        assert_eq!(c, Vec2d(vec![vec![3]]));

        let a = Vec2d(vec![vec![1]]);
        let b = Vec2d(vec![vec![2]]);
        let c = a.clone() - b.clone();
        assert_eq!(a, Vec2d(vec![vec![1]]));
        assert_eq!(b, Vec2d(vec![vec![2]]));
        assert_eq!(c, Vec2d(vec![vec![-1]]));
    }
}

