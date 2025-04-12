use rand::Rng;

pub fn randomly_permute<T: Copy>(a: &mut [T]) {
    let n = a.len();
    for i in 0..n {
        let index = rand::rng().random_range(i..n);
        (a[i], a[index]) = (a[index], a[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomly_permute_test() {
        for _ in 0..3 {
            let mut a: Vec<i32> = (1..=5).collect();
            randomly_permute(&mut a);
            println!("# randomly_permute_test: {a:?}");
        }
    }
}