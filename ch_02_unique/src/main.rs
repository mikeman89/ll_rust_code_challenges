use std::vec;

fn find_unique<T: Ord>(mut items: Vec<T>) -> Vec<T> {
    items.sort();
    items.dedup();
    items
}

fn main() {
    let input = vec![1, 2, 3, 3, 4];
    let uniques = find_unique(input);
    println!("{:?}", uniques);
}

#[test]
fn already_unique() {
    let input = vec![1, 2, 3];
    let expected = vec![1, 2, 3];
    let actual = find_unique(input);
    assert_eq!(expected, actual)
}

#[test]
fn duplicates() {
    let input = vec![1, 1, 4];
    let expected = vec![1, 4];
    let actual = find_unique(input);
    assert_eq!(expected, actual)
}

#[test]
fn keep_order() {
    let input = vec![1, 1, 4, 2, 1, 3];
    let expected = vec![1, 2, 3, 4];
    let actual = find_unique(input);
    assert_eq!(expected, actual)
}

#[test]
fn empty_list() {
    let input: Vec<i32> = vec![];
    let expected: Vec<i32> = vec![];
    let actual: Vec<i32> = find_unique(input);
    assert_eq!(expected, actual)
}
