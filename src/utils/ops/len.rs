pub trait Len {
    fn len(&self) -> usize;
}

// Blanket Implementation
impl<T: Len> Len for &T {
    fn len(&self) -> usize {
        T::len(self)
    }
}

impl<T, const N: usize> Len for [T; N] {
    fn len(&self) -> usize {
        N
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<T> Len for &[T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn longer<T: Len, U: Len>(a: T, b: U) -> usize {
        if a.len() > b.len() {
            a.len()
        } else {
            b.len()
        }
    }

    #[test]
    fn len() {
        let a = [1];
        let b = [1, 2];
        assert_eq!(longer(&a, &b), 2);
        assert_eq!(longer(a, b), 2);

        let a = vec![1];
        let b = vec![1, 2];
        assert_eq!(longer(&a, &b), 2);
        assert_eq!(longer(a, b), 2);

        let a = &[1][..];
        let b = &[1, 2][..];
        assert_eq!(longer(&a, &b), 2);
        assert_eq!(longer(a, b), 2);
    }
}