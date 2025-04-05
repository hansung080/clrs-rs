pub trait Slice<'a, Rng: ?Sized> {
    type Output: ?Sized;

    fn slice(&'a self, range: Rng) -> Self::Output;
}

pub trait SliceMut<'a, Rng: ?Sized> {
    type Output: ?Sized;

    fn slice_mut(&'a mut self, range: Rng) -> Self::Output;
}