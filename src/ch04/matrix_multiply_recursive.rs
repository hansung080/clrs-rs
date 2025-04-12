use std::ops::{AddAssign, Mul};
use crate::utils;
use crate::utils::matrix::Mat;

/*
    Divide and Conquer Algorithm for Matrix Multiplication
    A = [[A00, A01], [A10, A11]]
    B = [[B00, B01], [B10, B11]]
    C = [[C00, C01], [C10, C11]]

    C00 += A00 * B00 + A01 * B10
    C01 += A00 * B01 + A01 * B11
    C10 += A10 * B00 + A11 * B10
    C11 += A10 * B01 + A11 * B11
*/
pub fn matrix_multiply_recursive<T, const N: usize>(a: &Mat<T, N, N>, b: &Mat<T, N, N>) -> Mat<T, N, N>
where
    T: Mul<Output = T> + AddAssign + Default + Copy,
{
    if N == 0 { return Mat([[T::default(); N]; N]); }
    if !utils::is_power_of_two(N) { panic!("matrix dimension {N} is not an exact power of 2"); }

    let mut c = Mat([[T::default(); N]; N]);
    matrix_multiply_recursive_aux(a, b, &mut c, N, 0, 0, 0, 0, 0, 0);
    c
}

fn matrix_multiply_recursive_aux<T, const N: usize>(
    a: &Mat<T, N, N>,
    b: &Mat<T, N, N>,
    c: &mut Mat<T, N, N>,
    n: usize,
    a_i: usize,
    a_j: usize,
    b_i: usize,
    b_j: usize,
    c_i: usize,
    c_j: usize)
where
    T: Mul<Output = T> + AddAssign + Copy,
{
    if n == 1 {
        c[(c_i, c_j)] += a[(a_i, a_j)] * b[(b_i, b_j)];
        return;
    }
    let h = n / 2;
    matrix_multiply_recursive_aux(a, b, c, h, a_i, a_j, b_i, b_j, c_i, c_j);
    matrix_multiply_recursive_aux(a, b, c, h, a_i, a_j, b_i, b_j + h, c_i, c_j + h);
    matrix_multiply_recursive_aux(a, b, c, h, a_i + h, a_j, b_i, b_j, c_i + h, c_j);
    matrix_multiply_recursive_aux(a, b, c, h, a_i + h, a_j, b_i, b_j + h, c_i + h, c_j + h);
    matrix_multiply_recursive_aux(a, b, c, h, a_i, a_j + h, b_i + h, b_j, c_i, c_j);
    matrix_multiply_recursive_aux(a, b, c, h, a_i, a_j + h, b_i + h, b_j + h, c_i, c_j + h);
    matrix_multiply_recursive_aux(a, b, c, h, a_i + h, a_j + h, b_i + h, b_j, c_i + h, c_j);
    matrix_multiply_recursive_aux(a, b, c, h, a_i + h, a_j + h, b_i + h, b_j + h, c_i + h, c_j + h);
}

#[cfg(test)]
mod tests {
    use crate::ch04;
    use super::*;

    #[test]
    fn matrix_multiply_recursive_test() {
        ch04::tests::matrix_multiply_i32_n0(matrix_multiply_recursive);
        ch04::tests::matrix_multiply_i32_n1(matrix_multiply_recursive);
        ch04::tests::matrix_multiply_i32_n2(matrix_multiply_recursive);
        ch04::tests::matrix_multiply_i32_n4(matrix_multiply_recursive);
        ch04::tests::matrix_multiply_i32_n8(matrix_multiply_recursive);
        ch04::tests::matrix_multiply_f64_n4(matrix_multiply_recursive);
    }

    #[test]
    #[should_panic(expected = "matrix dimension 3 is not an exact power of 2")]
    fn matrix_multiply_recursive_error() {
        ch04::tests::matrix_multiply_i32_n3(matrix_multiply_recursive);
    }
}