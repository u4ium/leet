use super::*;

#[test]
fn empty() {
    let l1: LinkedList<i32> = vec![].into_iter().collect();
    let l2: LinkedList<i32> = vec![].into_iter().collect();
    assert_eq!(l1, l2);
}

#[test]
fn single_element() {
    let l1: LinkedList<i32> = vec![23].into_iter().collect();
    let l2: LinkedList<i32> = vec![23].into_iter().collect();
    assert_eq!(l1, l2);
}

#[test]
fn single_element_unequal() {
    let l1: LinkedList<i32> = vec![23].into_iter().collect();
    let l2: LinkedList<i32> = vec![2].into_iter().collect();
    assert_ne!(l1, l2);
}

#[test]
fn multiple_elements() {
    let l1: LinkedList<i32> = vec![1, -2, 3].into_iter().collect();
    let l2: LinkedList<i32> = vec![1, -2, 3].into_iter().collect();
    assert_eq!(l1, l2);
}

#[test]
fn different_lengths_unequal() {
    let l1: LinkedList<i32> = vec![1, -2, 3].into_iter().collect();
    let l2: LinkedList<i32> = vec![1, -2].into_iter().collect();
    assert_ne!(l1, l2);

    let l1: LinkedList<i32> = vec![].into_iter().collect();
    let l2: LinkedList<i32> = vec![1, -2].into_iter().collect();
    assert_ne!(l1, l2);
}
