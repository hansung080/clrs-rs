pub trait Slice<Rng: ?Sized> {
    type Output: ?Sized;

    fn slice(&self, range: Rng) -> Self::Output;
}

pub trait SliceMut<'a, Rng: ?Sized> {
    type Output: ?Sized;

    fn slice(&'a mut self, range: Rng) -> Self::Output;
}