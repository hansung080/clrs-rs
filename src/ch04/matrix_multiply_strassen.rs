use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};
use crate::utils;
use crate::utils::matrix::{Mat, Slice2d, Slice2dMut, Vec2d};
use crate::utils::ops::{Slice, SliceMut};

/*
    Strassen Algorithm for Matrix Multiplication
    A = [[A00, A01], [A10, A11]]
    B = [[B00, B01], [B10, B11]]
    C = [[C00, C01], [C10, C11]]

    S1  = B01 - B11
    S2  = A00 + A01
    S3  = A10 + A11
    S4  = B10 - B00
    S5  = A00 + A11
    S6  = B00 + B11
    S7  = A01 - A11
    S8  = B10 + B11
    S9  = A00 - A10
    S10 = B00 + B01

    P1 = A00 * S1 (= A00 * B01 - A00 * B11)
    P2 = S2 * B11 (= A00 * B11 + A01 * B11)
    P3 = S3 * B00 (= A10 * B00 + A11 * B00)
    P4 = A11 * S4 (= A11 * B10 - A11 * B00)
    P5 = S5 * S6  (= A00 * B00 + A00 * B11 + A11 * B00 + A11 * B11)
    P6 = S7 * S8  (= A01 * B10 + A01 * B11 - A11 * B10 - A11 * B11)
    P7 = S9 * S10 (= A00 * B00 + A00 * B01 - A10 * B00 - A10 * B01)

    C00 += P5 + P4 - P2 + P6 (= A00 * B00 + A01 * B10)
    C01 += P1 + P2           (= A00 * B01 + A01 * B11)
    C10 += P3 + P4           (= A10 * B00 + A11 * B10)
    C11 += P5 + P1 - P3 - P7 (= A10 * B01 + A11 * B11)
*/
pub fn matrix_multiply_strassen<T, const N: usize>(a: &Mat<T, N, N>, b: &Mat<T, N, N>) -> Mat<T, N, N>
where
    T: Mul<Output = T> + Add<Output = T> + AddAssign + Sub<Output = T> + Default + Copy,
{
    if N == 0 { return Mat([[T::default(); N]; N]); }
    if !utils::is_power_of_two(N) { panic!("matrix dimension {N} is not an exact power of 2"); }
    let mut c = Mat([[T::default(); N]; N]);
    matrix_multiply_strassen_aux(a.as_slice2d(), b.as_slice2d(), c.as_slice2d_mut(), N);
    c
}

fn matrix_multiply_strassen_aux<A, B, C>(a: Slice2d<A>, b: Slice2d<B>, mut c: Slice2dMut<C>, n: usize)
where
    A: Index<usize>,
    <A as Index<usize>>::Output: Mul<<B as Index<usize>>::Output, Output = <C as Index<usize>>::Output> + Add<Output = <A as Index<usize>>::Output> + Sub<Output = <A as Index<usize>>::Output> + Copy,
    B: Index<usize>,
    <B as Index<usize>>::Output: Add<Output = <B as Index<usize>>::Output> + Sub<Output = <B as Index<usize>>::Output> + Copy,
    C: IndexMut<usize>,
    <C as Index<usize>>::Output: Add<Output = <C as Index<usize>>::Output> + AddAssign + Sub<Output = <C as Index<usize>>::Output> + Default + Copy,
{
    if n == 1 {
        c[(0, 0)] += a[(0, 0)] * b[(0, 0)];
        return;
    }

    let h = n / 2;
    let a00 = a.slice((0..h, 0..h));
    let a01 = a.slice((0..h, h..n));
    let a10 = a.slice((h..n, 0..h));
    let a11 = a.slice((h..n, h..n));

    let b00 = b.slice((0..h, 0..h));
    let b01 = b.slice((0..h, h..n));
    let b10 = b.slice((h..n, 0..h));
    let b11 = b.slice((h..n, h..n));

    let s1 = &b01 - &b11;
    let s2 = &a00 + &a01;
    let s3 = &a10 + &a11;
    let s4 = &b10 - &b00;
    let s5 = &a00 + &a11;
    let s6 = &b00 + &b11;
    let s7 = &a01 - &a11;
    let s8 = &b10 + &b11;
    let s9 = &a00 - &a10;
    let s10 = &b00 + &b01;

    let mut p1 = Vec2d::defaults((h, h));
    let mut p2 = Vec2d::defaults((h, h));
    let mut p3 = Vec2d::defaults((h, h));
    let mut p4 = Vec2d::defaults((h, h));
    let mut p5 = Vec2d::defaults((h, h));
    let mut p6 = Vec2d::defaults((h, h));
    let mut p7 = Vec2d::defaults((h, h));

    matrix_multiply_strassen_aux(a00, s1.as_slice2d(), p1.as_slice2d_mut(), h);
    matrix_multiply_strassen_aux(s2.as_slice2d(), b11, p2.as_slice2d_mut(), h);
    matrix_multiply_strassen_aux(s3.as_slice2d(), b00, p3.as_slice2d_mut(), h);
    matrix_multiply_strassen_aux(a11, s4.as_slice2d(), p4.as_slice2d_mut(), h);
    matrix_multiply_strassen_aux(s5.as_slice2d(), s6.as_slice2d(), p5.as_slice2d_mut(), h);
    matrix_multiply_strassen_aux(s7.as_slice2d(), s8.as_slice2d(), p6.as_slice2d_mut(), h);
    matrix_multiply_strassen_aux(s9.as_slice2d(), s10.as_slice2d(), p7.as_slice2d_mut(), h);

    let mut c00 = c.slice_mut((0..h, 0..h));
    c00 += &p5 + &p4 - &p2 + &p6;
    let mut c01 = c.slice_mut((0..h, h..n));
    c01 += &p1 + &p2;
    let mut c10 = c.slice_mut((h..n, 0..h));
    c10 += &p3 + &p4;
    let mut c11 = c.slice_mut((h..n, h..n));
    c11 += &p5 + &p1 - &p3 - &p7;
}

#[cfg(test)]
mod tests {
    use crate::ch04;
    use super::*;

    #[test]
    fn matrix_multiply_strassen_test() {
        ch04::tests::matrix_multiply_i32_n0(matrix_multiply_strassen);
        ch04::tests::matrix_multiply_i32_n1(matrix_multiply_strassen);
        ch04::tests::matrix_multiply_i32_n2(matrix_multiply_strassen);
        ch04::tests::matrix_multiply_i32_n4(matrix_multiply_strassen);
        ch04::tests::matrix_multiply_i32_n8(matrix_multiply_strassen);
        ch04::tests::matrix_multiply_f64_n4(matrix_multiply_strassen);
    }

    #[test]
    #[should_panic(expected = "matrix dimension 3 is not an exact power of 2")]
    fn matrix_multiply_strassen_error() {
        ch04::tests::matrix_multiply_i32_n3(matrix_multiply_strassen);
    }
}