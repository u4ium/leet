use super::*;

#[test]
fn modify_only_element() {
    let mut list: LinkedList<bool> = vec![true].into_iter().collect();
    *list.get_mut(0).unwrap() = false;
    assert_eq!(list.head.unwrap().val, false);
}

#[test]
fn modify_middle_element() {
    let mut list: LinkedList<char> = ('a'..='c').collect();
    *list.get_mut(1).unwrap() = '1';
    assert_eq!(list.head.unwrap().next.unwrap().val, '1');
}

#[test]
fn access_index_zero_on_empty_list() {
    let mut list: LinkedList<u32> = vec![].into_iter().collect();
    assert_eq!(None, list.get_mut(0));
}

#[test]
fn access_index_len() {
    let mut list: LinkedList<u32> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(None, list.get_mut(3));
}

#[test]
fn access_large_index() {
    let mut list: LinkedList<u32> = (1..=100).into_iter().collect();
    assert_eq!(None, list.get_mut(200));
}
