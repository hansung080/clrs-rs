pub fn merge_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    if a.len() == 0 { return; }
    _merge_sort(a, 0, a.len() - 1);
}

fn _merge_sort<T: PartialOrd + Copy>(a: &mut [T], p: usize, r: usize) {
    if p >= r { return; }
    let q = (p + r) / 2;
    _merge_sort(a, p, q);
    _merge_sort(a, q + 1, r);
    merge(a, p, q, r);
}

fn merge<T: PartialOrd + Copy>(a: &mut [T], p: usize, q: usize, r: usize) {
    let left = Vec::from(&a[p..=q]);
    let right = Vec::from(&a[q + 1..=r]);

    let n_left = left.len();
    let n_right = right.len();
    let mut i = 0;
    let mut j = 0;
    let mut k = p;
    while i < n_left && j < n_right {
        if left[i] < right[j] {
            a[k] = left[i];
            i += 1;
        } else {
            a[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < n_left {
        a[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < n_right {
        a[k] = right[j];
        j += 1;
        k += 1;
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_i32() {
        crate::ch02::tests::sort_i32(merge_sort);
    }

    #[test]
    fn merge_sort_f64() {
        crate::ch02::tests::sort_f64(merge_sort);
    }

    #[test]
    fn merge_sort_char() {
        crate::ch02::tests::sort_char(merge_sort);
    }

    #[test]
    fn merge_sort_str() {
        crate::ch02::tests::sort_str(|a| merge_sort(a));
    }
}