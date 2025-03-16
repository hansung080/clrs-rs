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
    use rand::seq::SliceRandom;

    #[test]
    fn insertion_sort_i32() {
        let cases: [(&mut [i32], &[i32]); 6] = [
            (&mut [], &[]),
            (&mut [1], &[1]),
            (&mut [1, 2, 3, 4, 5, 6], &[1, 2, 3, 4, 5, 6]),
            (&mut [6, 5, 4, 3, 2, 1], &[1, 2, 3, 4, 5, 6]),
            (&mut [5, 2, 4, 6, 1, 3], &[1, 2, 3, 4, 5, 6]),
            (&mut [-3, 5, 4, -1, 2, -6], &[-6, -3, -1, 2, 4, 5]),
        ];
        for (input, expected) in cases {
            insertion_sort(input);
            assert_eq!(input, expected);
        }
    }

    #[test]
    fn insertion_sort_f64() {
        let mut rng = rand::rng();
        let mut arr = [-0.1, 0.2, -0.3, 0.4, 0.5, -0.6];
        arr.shuffle(&mut rng);
        insertion_sort(&mut arr);
        assert_eq!(arr, [-0.6, -0.3, -0.1, 0.2, 0.4, 0.5]);
    }

    #[test]
    fn insertion_sort_char() {
        let mut rng = rand::rng();
        let mut arr = ['a', 'b', 'c', 'd', 'e', 'f'];
        arr.shuffle(&mut rng);
        insertion_sort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn insertion_sort_str() {
        let mut rng = rand::rng();
        let mut arr = ["a", "b", "c", "d", "e", "f", "ff"];
        arr.shuffle(&mut rng);
        insertion_sort(&mut arr);
        assert_eq!(arr, ["a", "b", "c", "d", "e", "f", "ff"]);
    }
}