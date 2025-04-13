use std::f64;

pub fn online_maximum(scores: &[u32]) -> usize {
    online_maximum_aux((scores.len() as f64 / f64::consts::E) as usize, scores)
}

pub fn online_maximum_aux(k: usize, scores: &[u32]) -> usize {
    let mut best_score = 0;
    for i in 0..k {
        if scores[i] > best_score {
            best_score = scores[i];
        }
    }

    let n = scores.len();
    for i in k..n {
        if scores[i] > best_score {
            return i;
        }
    }
    n - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn online_maximum_test() {
        let cases = [
            ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3),
            ([10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 9),
            ([5, 2, 1, 8, 4, 7, 10, 9, 3, 6], 3),
            ([5, 2, 9, 8, 4, 7, 10, 1, 3, 6], 6),
        ];

        for (scores, expected) in cases.iter() {
            assert_eq!(online_maximum(scores), *expected);
        }
    }
}