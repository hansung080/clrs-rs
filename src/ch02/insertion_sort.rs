pub fn insertion_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    for i in 1..a.len() {
        let key = a[i];
        let mut j = i;
        while j > 0 && a[j - 1] > key  {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_i32() {
        crate::ch02::tests::sort_i32(insertion_sort);
    }

    #[test]
    fn insertion_sort_f64() {
        crate::ch02::tests::sort_f64(insertion_sort);
    }

    #[test]
    fn insertion_sort_char() {
        crate::ch02::tests::sort_char(insertion_sort);
    }

    #[test]
    fn insertion_sort_str() {
        crate::ch02::tests::sort_str(|a| insertion_sort(a));
    }
}