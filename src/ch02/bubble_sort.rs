pub fn bubble_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    for i in (1..a.len()).rev() {
        let mut swapped = false;
        for j in 0..i {
            if a[j] > a[j + 1] {
                (a[j], a[j + 1]) = (a[j + 1], a[j]);
                swapped = true;
            }
        }
        if swapped == false {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ch02;
    use super::*;

    #[test]
    fn bubble_sort_test() {
        ch02::tests::sort_i32(bubble_sort);
        ch02::tests::sort_f64(bubble_sort);
        ch02::tests::sort_char(bubble_sort);
        ch02::tests::sort_str(|a| bubble_sort(a));
    }
}