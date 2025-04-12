use std::ops::{AddAssign, Mul};
use crate::utils::matrix::Mat;

pub fn matrix_multiply<T, const N: usize>(a: &Mat<T, N, N>, b: &Mat<T, N, N>) -> Mat<T, N, N>
where
    T: Mul<Output = T> + AddAssign + Default + Copy,
{
    let mut c = Mat([[T::default(); N]; N]);
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                c[(i, j)] += a[(i, k)] * b[(k, j)];
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use crate::ch04;
    use super::*;

    #[test]
    fn matrix_multiply_test() {
        ch04::tests::matrix_multiply_i32_n0(matrix_multiply);
        ch04::tests::matrix_multiply_i32_n1(matrix_multiply);
        ch04::tests::matrix_multiply_i32_n2(matrix_multiply);
        ch04::tests::matrix_multiply_i32_n3(matrix_multiply);
        ch04::tests::matrix_multiply_i32_n4(matrix_multiply);
        ch04::tests::matrix_multiply_i32_n8(matrix_multiply);
        ch04::tests::matrix_multiply_f64_n4(matrix_multiply);
    }
}
