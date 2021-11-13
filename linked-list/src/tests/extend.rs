use super::*;

fn test<T: std::fmt::Debug + PartialEq + Clone, const N: usize, const M: usize>(
    start: [T; N],
    to_add: [T; M],
) {
    let expect: LinkedList<T> = start.iter().chain(to_add.iter()).cloned().collect();
    let mut list: LinkedList<T> = start.iter().cloned().collect();
    list.extend(to_add);
    assert_eq!(expect, list);
}

#[test]
fn empty_extend_one() {
    test([], [1]);
}

#[test]
fn empty_extend_many() {
    test([], ['a', 'b', 'c']);
}

#[test]
fn start_with_one_extend_one() {
    test([1], [2]);
}

#[test]
fn start_with_one_extend_many() {
    test(['z'], ['y', 'x', 'w']);
}

#[test]
fn start_with_many_extend_one() {
    test([1, 2, 3], [4]);
}

#[test]
fn start_with_many_extend_many() {
    test(['a', 'b', 'c'], ['x', 'y', 'z']);
}
