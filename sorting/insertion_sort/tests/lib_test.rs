extern crate insertion_sort;

#[test]
fn test_insertion_sort_001() {
    let mut unordered_seq = [5, 9, 4, 3, 1, 2];
    let ordered_seq = [1, 2, 3, 4, 5, 9];
    insertion_sort::insertion_sort(&mut unordered_seq);

    assert_eq!(ordered_seq, unordered_seq);
}

#[test]
fn test_insertion_sort_002() {
    let mut unordered_seq = [3, 4, 5, 6, 1];
    let ordered_seq = [1, 3, 4, 5, 6];
    insertion_sort::insertion_sort(&mut unordered_seq);

    assert_eq!(ordered_seq, unordered_seq);
}
