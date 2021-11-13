use super::*;

#[test]
fn empty() {
    let expect = LinkedList {
        length: 0,
        head: None,
    };
    let actual: LinkedList<i32> = [].iter().cloned().collect();
    assert_eq!(expect, actual);
}

#[test]
fn single_element() {
    let expect = LinkedList {
        length: 1,
        head: Some(Box::new(LinkedListNode {
            val: 14,
            next: None,
        })),
    };
    let actual: LinkedList<i32> = [14].iter().cloned().collect();
    assert_eq!(expect, actual);
}

#[test]
fn two_elements() {
    let expect = LinkedList {
        length: 2,
        head: Some(Box::new(LinkedListNode {
            val: 1,
            next: Some(Box::new(LinkedListNode { val: 4, next: None })),
        })),
    };
    let actual: LinkedList<i32> = [1, 4].iter().cloned().collect();
    assert_eq!(expect, actual);
}
