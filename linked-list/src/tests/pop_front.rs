use super::*;

fn test<T: Clone + PartialEq + std::fmt::Debug>(expect: Vec<T>) {
    let mut list: LinkedList<T> = expect.iter().cloned().collect();
    for value in expect {
        assert_eq!(Some(value), list.pop_front());
    }
    assert_eq!(list, LinkedList::new());
    assert_eq!(None, list.pop_front());
    assert_eq!(None, list.pop_front());
}

#[test]
fn empty() {
    test::<i32>(vec![]);
}

#[test]
fn single_element() {
    test(vec![-5]);
}

#[test]
fn multiple_elements() {
    test(vec![-5, 16]);
    test(vec!['a', 'b', 'c']);
    test((b'a'..b'z').collect());
}
