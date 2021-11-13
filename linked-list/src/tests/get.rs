use super::*;

fn check_values<T: PartialEq + std::fmt::Debug>(list: LinkedList<T>, values: Vec<T>) {
    for (i, value) in values.iter().enumerate() {
        assert_eq!(Some(value), list.get(i));
    }
}

#[test]
fn empty() {
    let values = vec![];
    let list: LinkedList<char> = values.iter().cloned().collect();
    check_values(list, values);
}

#[test]
fn single_element() {
    let values = vec!['q'];
    let list: LinkedList<char> = values.iter().cloned().collect();
    check_values(list, values);
}

#[test]
fn two_elements() {
    let values = vec![1.5, 2.9];
    let list: LinkedList<f64> = values.iter().cloned().collect();
    check_values(list, values);
}

#[test]
fn multiple_elements() {
    let values: Vec<char> = ('a'..='z').collect();
    let list: LinkedList<char> = values.iter().cloned().collect();
    check_values(list, values);
}

#[test]
fn index_zero_on_empty_list() {
    let list: LinkedList<u32> = vec![].into_iter().collect();
    assert_eq!(None, list.get(0));
}

#[test]
fn index_out_of_bounds_at_len() {
    let list: LinkedList<u32> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(None, list.get(3));
}

#[test]
fn index_far_out_of_bounds() {
    let list: LinkedList<u32> = (1..=100).into_iter().collect();
    assert_eq!(None, list.get(200));
}
