use clrs::prelude::insertion_sort;
use clrs::prelude::merge_sort;

#[test]
fn insertion_sort_test() {
    let mut array = [5, 2, 4, 6, 1, 3];
    insertion_sort(&mut array);
    assert_eq!(array, [1, 2, 3, 4, 5, 6]);
}

#[test]
fn merge_sort_test() {
    let mut array = [2, 4, 6, 7, 1, 2, 3, 5];
    merge_sort(&mut array);
    assert_eq!(array, [1, 2, 2, 3, 4, 5, 6, 7]);

    let mut array = [12, 3, 7, 9, 14, 6, 11, 2];
    merge_sort(&mut array);
    assert_eq!(array, [2, 3, 6, 7, 9, 11, 12, 14]);
}