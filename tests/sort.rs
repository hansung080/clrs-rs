use clrs::sort::insertion_sort;

#[test]
fn insertion_sort_test() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
}