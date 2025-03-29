pub mod ops;
pub mod matrix;

pub fn is_power_of_two(n: usize) -> bool {
    n != 0 && n & (n - 1) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_power_of_two_test() {
        let cases = [
            (0, false),
            (1, true),
            (2, true),
            (3, false),
            (4, true),
            (5, false),
            (6, false),
            (7, false),
            (8, true),
            (9, false),
            (10, false),
        ];

        for (n, expected) in cases {
            assert_eq!(is_power_of_two(n), expected);
        }
    }
}