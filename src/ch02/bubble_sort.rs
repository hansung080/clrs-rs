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
    use super::*;

    #[test]
    fn bubble_sort_i32() {
        crate::ch02::tests::sort_i32(bubble_sort);
    }

    #[test]
    fn bubble_sort_f64() {
        crate::ch02::tests::sort_f64(bubble_sort);
    }

    #[test]
    fn bubble_sort_char() {
        crate::ch02::tests::sort_char(bubble_sort);
    }

    #[test]
    fn bubble_sort_str() {
        crate::ch02::tests::sort_str(|a| bubble_sort(a));
    }
}