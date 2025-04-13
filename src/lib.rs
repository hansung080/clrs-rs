mod ch02;
mod ch04;
mod ch05;
mod utils;

pub mod prelude {
    pub use crate::ch02::insertion_sort;
    pub use crate::ch02::merge_sort;
    pub use crate::ch02::bubble_sort;

    pub use crate::ch04::matrix_multiply;
    pub use crate::ch04::matrix_multiply_recursive;
    pub use crate::ch04::matrix_multiply_strassen;

    pub use crate::ch05::hire_assistant;
    pub use crate::ch05::randomly_permute;
    pub use crate::ch05::randomized_hire_assistant;
    pub use crate::ch05::random_sample;
    pub use crate::ch05::online_maximum;
}