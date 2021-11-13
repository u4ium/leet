use super::*;

fn test<T: std::fmt::Debug + PartialEq + Clone, const N: usize, const M: usize>(
    start: [T; N],
    to_add: [T; M],
) {
    let mut list: LinkedList<T> = start.iter().cloned().collect();
    for value in to_add.iter() {
        list.push(value.clone());
    }
    let expect: LinkedList<T> = start.iter().chain(to_add.iter()).cloned().collect();
    assert_eq!(expect, list);
}

#[test]
fn empty_push_one() {
    test([], [1]);
}

#[test]
fn empty_push_many() {
    test([], ['a', 'b', 'c']);
}

#[test]
fn start_with_one_push_one() {
    test([1], [2]);
}

#[test]
fn start_with_one_push_many() {
    test(['z'], ['y', 'x', 'w']);
}

#[test]
fn start_with_many_push_one() {
    test([1, 2, 3], [4]);
}

#[test]
fn start_with_many_push_many() {
    test(['a', 'b', 'c'], ['x', 'y', 'z']);
}
