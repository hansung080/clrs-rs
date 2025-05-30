pub trait Len {
    fn len(&self) -> usize;
}

// Blanket implementation of `Len`
impl<T: Len> Len for &T {
    fn len(&self) -> usize {
        T::len(self)
    }
}

// Blanket implementation of `Len`
impl<T: Len> Len for &mut T {
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

impl<T> Len for &mut [T] {
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
        let mut a = [1];
        let mut b = [1, 2];
        assert_eq!(longer(&a, &b), 2);
        assert_eq!(longer(&mut a, &mut b), 2);
        assert_eq!(longer(a, b), 2);

        let mut a = vec![1];
        let mut b = vec![1, 2];
        assert_eq!(longer(&a, &b), 2);
        assert_eq!(longer(&mut a, &mut b), 2);
        assert_eq!(longer(a, b), 2);

        let mut a = &[1][..];
        let mut b = &[1, 2][..];
        assert_eq!(longer(&a, &b), 2);
        assert_eq!(longer(&mut a, &mut b), 2);
        assert_eq!(longer(a, b), 2);

        let mut a = &mut [1][..];
        let mut b = &mut [1, 2][..];
        assert_eq!(longer(&a, &b), 2);
        assert_eq!(longer(&mut a, &mut b), 2);
        assert_eq!(longer(a, b), 2);
    }
}