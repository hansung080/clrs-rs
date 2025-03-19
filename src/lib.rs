mod ch02;
mod ch04;

pub mod prelude {
    pub use crate::ch02::insertion_sort::insertion_sort;
    pub use crate::ch02::merge_sort::merge_sort;
    pub use crate::ch02::bubble_sort::bubble_sort;

    pub use crate::ch04::matrix_multiply::matrix_multiply;
}