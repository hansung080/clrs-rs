mod ch02;
mod ch04;
pub mod utils;

pub mod prelude {
    pub use crate::ch02::insertion_sort::insertion_sort;
    pub use crate::ch02::merge_sort::merge_sort;
    pub use crate::ch02::bubble_sort::bubble_sort;

    pub use crate::ch04::matrix_multiply::matrix_multiply;
    pub use crate::ch04::matrix_multiply_recursive::matrix_multiply_recursive;
    pub use crate::ch04::matrix_multiply_strassen::matrix_multiply_strassen;
}