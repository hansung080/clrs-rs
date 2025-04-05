use std::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Mul, Sub};
use crate::utils;
use crate::utils::matrix::{Mat, Slice2d, Slice2dMut, Vec2d};
use crate::utils::ops::{Len, Slice};

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
    matrix_multiply_strassen_aux(Slice2d::new(a.deref()), Slice2d::new(b.deref()), Slice2dMut::new(c.deref_mut()), N);
    c
}

fn matrix_multiply_strassen_aux<A, B, C>(a: Slice2d<A>, b: Slice2d<B>, mut c: Slice2dMut<C>, n: usize)
where
    A: IndexMut<usize> + Len,
    <A as Index<usize>>::Output: Mul<<B as Index<usize>>::Output, Output = <C as Index<usize>>::Output> + Add<Output = <A as Index<usize>>::Output> + AddAssign + Sub<Output = <A as Index<usize>>::Output> + Default + Copy,
    B: IndexMut<usize> + Len,
    <B as Index<usize>>::Output: Add<Output = <B as Index<usize>>::Output> + AddAssign + Sub<Output = <B as Index<usize>>::Output> + Default + Copy,
    C: IndexMut<usize> + Len,
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
    // let s2 = &a00 + &a01;
    // let s3 = &a10 + &a11;
    // let s4 = &b10 - &b00;
    // let s5 = &a00 + &a11;
    // let s6 = &b00 + &b11;
    // let s7 = &a01 - &a11;
    // let s8 = &b10 + &b11;
    // let s9 = &a00 - &a10;
    // let s10 = &b00 + &b01;

    let mut p1 = Vec2d::defaults((h, h));
    // let mut p2 = Vec2d::defaults((h, h));
    // let mut p3 = Vec2d::defaults((h, h));
    // let mut p4 = Vec2d::defaults((h, h));
    // let mut p5 = Vec2d::defaults((h, h));
    // let mut p6 = Vec2d::defaults((h, h));
    // let mut p7 = Vec2d::defaults((h, h));

    matrix_multiply_strassen_aux(a00, Slice2d::new(s1.deref()), Slice2dMut::new(p1.deref_mut()), h);
}