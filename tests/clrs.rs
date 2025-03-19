use clrs::prelude::insertion_sort;
use clrs::prelude::merge_sort;

#[test]
fn insertion_sort_test() {
    let mut a = [5, 2, 4, 6, 1, 3];
    insertion_sort(&mut a);
    assert_eq!(a, [1, 2, 3, 4, 5, 6]);
}

#[test]
fn merge_sort_test() {
    let mut a = [2, 4, 6, 7, 1, 2, 3, 5];
    merge_sort(&mut a);
    assert_eq!(a, [1, 2, 2, 3, 4, 5, 6, 7]);

    let mut a = [12, 3, 7, 9, 14, 6, 11, 2];
    merge_sort(&mut a);
    assert_eq!(a, [2, 3, 6, 7, 9, 11, 12, 14]);
}