use crate::ch05;

pub fn randomized_hire_assistant(ranks: &mut [u32]) -> Vec<usize> {
    ch05::randomly_permute(ranks);
    ch05::hire_assistant(ranks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomized_hire_assistant_test() {
        for _ in 0..3 {
            let mut ranks = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // worst case
            let hired = randomized_hire_assistant(&mut ranks);
            println!("# randomized_hire_assistant_test: ranks: {ranks:?}, hired: {hired:?}");
        }
    }
}