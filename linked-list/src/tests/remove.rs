use super::*;

#[test]
#[should_panic]
fn empty() {
    let mut list = LinkedList::<i32>::new();
    list.remove(0);
}

#[test]
#[should_panic]
fn empty_with_non_zero_index() {
    let mut list = LinkedList::<i32>::new();
    list.remove(15);
}

#[test]
#[should_panic]
fn index_out_of_bounds_at_len() {
    let mut list: LinkedList<i32> = vec![1, -2, 3].into_iter().collect();
    list.remove(3);
}

#[test]
#[should_panic]
fn index_far_out_of_bounds() {
    let mut list: LinkedList<i32> = vec![1, -2, 3].into_iter().collect();
    list.remove(200);
}

#[test]
fn only_element() {
    let mut list: LinkedList<i32> = vec![1].into_iter().collect();
    let expect: LinkedList<i32> = vec![].into_iter().collect();
    list.remove(0);
    assert_eq!(expect, list);
}

#[test]
fn from_start() {
    let mut list: LinkedList<i32> = vec![1, -2, 3].into_iter().collect();
    let expect: LinkedList<i32> = vec![-2, 3].into_iter().collect();
    list.remove(0);
    assert_eq!(expect, list);
}

#[test]
fn from_middle() {
    let mut list: LinkedList<i32> = vec![1, -2, 3].into_iter().collect();
    let expect: LinkedList<i32> = vec![1, 3].into_iter().collect();
    list.remove(1);
    assert_eq!(expect, list);
}

#[test]
fn from_end() {
    let mut list: LinkedList<i32> = vec![1, -2, 3].into_iter().collect();
    let expect: LinkedList<i32> = vec![1, -2].into_iter().collect();
    list.remove(2);
    assert_eq!(expect, list);
}
