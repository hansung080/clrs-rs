pub trait Slice<Rng: ?Sized> {
    type Output: ?Sized;

    fn slice(&self, range: Rng) -> Self::Output;
}