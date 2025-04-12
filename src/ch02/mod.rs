mod insertion_sort;
mod merge_sort;
mod bubble_sort;

pub use insertion_sort::*;
pub use merge_sort::*;
pub use bubble_sort::*;

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    pub fn sort_i32(sort: fn(&mut [i32])) {
        let cases: [(&mut [i32], &[i32]); 7] = [
            (&mut [], &[]),
            (&mut [1], &[1]),
            (&mut [1, 2, 3, 4, 5, 6], &[1, 2, 3, 4, 5, 6]),
            (&mut [6, 5, 4, 3, 2, 1], &[1, 2, 3, 4, 5, 6]),
            (&mut [5, 2, 4, 6, 1, 3], &[1, 2, 3, 4, 5, 6]),
            (&mut [-3, 5, 4, -1, 2, 2, -6], &[-6, -3, -1, 2, 2, 4, 5]),
            (&mut [7, 2, 4, 5, 8, 3, 6, 1], &[1, 2, 3, 4, 5, 6, 7, 8]),
        ];
        for (a, expected) in cases {
            sort(a);
            assert_eq!(a, expected);
        }
    }

    pub fn sort_f64(sort: fn(&mut [f64])) {
        let mut rng = rand::rng();
        let mut a = [-0.1, 0.2, 0.2, -0.3, 0.4, 0.5, -0.6];
        a.shuffle(&mut rng);
        sort(&mut a);
        assert_eq!(a, [-0.6, -0.3, -0.1, 0.2, 0.2, 0.4, 0.5]);
    }

    pub fn sort_char(sort: fn(&mut [char])) {
        let mut rng = rand::rng();
        let mut a = ['a', 'b', 'b', 'c', 'd', 'e', 'f'];
        a.shuffle(&mut rng);
        sort(&mut a);
        assert_eq!(a, ['a', 'b', 'b', 'c', 'd', 'e', 'f']);
    }

    pub fn sort_str(sort: fn(&mut [&str])) {
        let mut rng = rand::rng();
        let mut a = ["a", "b", "b", "c", "d", "e", "ee", "f"];
        a.shuffle(&mut rng);
        sort(&mut a);
        assert_eq!(a, ["a", "b", "b", "c", "d", "e", "ee", "f"]);
    }
}