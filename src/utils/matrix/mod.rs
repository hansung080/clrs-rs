mod mat;
mod vec2d;
mod slice2d;
mod slice2d_mut;

pub use mat::*;
pub use vec2d::*;
pub use slice2d::*;
pub use slice2d_mut::*;

pub trait Shape {
    fn shape(&self) -> (usize, usize);
}

// Blanket implementation of `Shape`
impl<T: Shape> Shape for &T {
    fn shape(&self) -> (usize, usize) {
        T::shape(self)
    }
}
