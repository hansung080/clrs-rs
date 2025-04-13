pub fn hire_assistant(ranks: &[u32]) -> Vec<usize> {
    let mut hired = Vec::new();
    let mut best_rank = 0;
    for i in 0..ranks.len() {
        if ranks[i] > best_rank {
            best_rank = ranks[i];
            hired.push(i);
        }
    }
    hired
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hire_assistant_test() {
        let cases = [
            ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), // worst case
            ([10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![0]), // best case
            ([5, 2, 1, 8, 4, 7, 10, 9, 3, 6], vec![0, 3, 6]), // average case
        ];

        for (ranks, expected) in cases.iter() {
            assert_eq!(&hire_assistant(ranks), expected);
        }
    }
}