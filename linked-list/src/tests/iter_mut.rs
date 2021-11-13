use super::*;

#[test]
fn empty_map_plus_one() {
    let mut list: LinkedList<i32> = vec![].into_iter().collect();
    let expect: LinkedList<i32> = vec![].into_iter().collect();
    for elem in list.iter_mut() {
        *elem += 1;
    }
    assert_eq!(expect, list);
}
#[test]
fn full_map_plus_one() {
    let mut list: LinkedList<i32> = vec![0, 1, 2].into_iter().collect();
    let expect: LinkedList<i32> = vec![1, 2, 3].into_iter().collect();
    for elem in list.iter_mut() {
        *elem += 1;
    }
    assert_eq!(expect, list);
}
