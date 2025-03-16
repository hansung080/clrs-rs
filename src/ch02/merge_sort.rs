pub fn merge_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    if a.len() == 0 { return; }
    _merge_sort(a, 0, a.len() - 1);
}

fn _merge_sort<T: PartialOrd + Copy>(a: &mut [T], p: usize, r: usize) {
    if p == r { return; }
    let q = (p + r) / 2;
    _merge_sort(a, p, q);
    _merge_sort(a, q + 1, r);
    merge(a, p, q, r);
}

fn merge<T: PartialOrd + Copy>(a: &mut [T], p: usize, q: usize, r: usize) {
    let left = Vec::from(&a[p..=q]);
    let right = Vec::from(&a[(q + 1)..=r]);

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
    use rand::seq::SliceRandom;

    #[test]
    fn merge_sort_i32() {
        let cases: [(&mut [i32], &[i32]); 7] = [
            (&mut [], &[]),
            (&mut [1], &[1]),
            (&mut [1, 2, 3, 4, 5, 6], &[1, 2, 3, 4, 5, 6]),
            (&mut [6, 5, 4, 3, 2, 1], &[1, 2, 3, 4, 5, 6]),
            (&mut [5, 2, 4, 6, 1, 3], &[1, 2, 3, 4, 5, 6]),
            (&mut [-3, 5, 4, -1, 2, 2, -6], &[-6, -3, -1, 2, 2, 4, 5]),
            (&mut [7, 2, 4, 5, 8, 3, 6, 1], &[1, 2, 3, 4, 5, 6, 7, 8]),
        ];
        for (input, expected) in cases {
            merge_sort(input);
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn merge_sort_f64() {
        let mut rng = rand::rng();
        let mut arr = [-0.1, 0.2, 0.2, -0.3, 0.4, 0.5, -0.6];
        arr.shuffle(&mut rng);
        merge_sort(&mut arr);
        assert_eq!(arr, [-0.6, -0.3, -0.1, 0.2, 0.2, 0.4, 0.5]);
    }

    #[test]
    fn merge_sort_char() {
        let mut rng = rand::rng();
        let mut arr = ['a', 'b', 'b', 'c', 'd', 'e', 'f'];
        arr.shuffle(&mut rng);
        merge_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn merge_sort_str() {
        let mut rng = rand::rng();
        let mut arr = ["a", "b", "b", "c", "d", "e", "ee", "f"];
        arr.shuffle(&mut rng);
        merge_sort(&mut arr);
        assert_eq!(arr, ["a", "b", "b", "c", "d", "e", "ee", "f"]);
    }
}