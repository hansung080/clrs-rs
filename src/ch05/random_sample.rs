use std::collections::HashSet;
use rand::Rng;

// Create and return the `m`-sized random sample drawn from the set {1, 2, ..., `n`}.
pub fn random_sample(m: usize, n: usize) -> HashSet<usize> {
    let mut s = HashSet::with_capacity(m);
    for k in n - m + 1..=n {
        let i = rand::rng().random_range(1..=k);
        if s.contains(&i) {
            s.insert(k);
        } else {
            s.insert(i);
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_sample_test() {
        let cases = [
            (0, 10),
            (1, 10),
            (5, 10),
            (10, 10),
        ];

        for (m, n) in cases {
            let population: HashSet<usize> = (1..=n).collect();
            let sample = random_sample(m, n);
            assert_eq!(sample.len(), m);
            assert!(sample.is_subset(&population), "assertion failed: {sample:?} is not subset of {population:?}");
        }
    }
}