use super::*;

#[test]
fn modify_only_element() {
    let mut list: LinkedList<bool> = vec![true].into_iter().collect();
    list[0] = false;
    assert_eq!(list.head.unwrap().val, false);
}

#[test]
fn modify_middle_element() {
    let mut list: LinkedList<char> = ('a'..='c').collect();
    list[1] = '1';
    assert_eq!(list.head.unwrap().next.unwrap().val, '1');
}

#[test]
#[should_panic]
fn accessing_index_zero_on_empty_list() {
    let mut list: LinkedList<u32> = vec![].into_iter().collect();
    list[0] = 0;
}

#[test]
#[should_panic]
fn accessing_index_len() {
    let mut list: LinkedList<u32> = vec![1, 2, 3].into_iter().collect();
    list[3] = 0;
}

#[test]
#[should_panic]
fn accessing_large_index() {
    let mut list: LinkedList<u32> = (1..=100).into_iter().collect();
    list[200] = 0;
}
