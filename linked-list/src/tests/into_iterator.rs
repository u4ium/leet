use super::*;

fn check_values<T: PartialEq + std::fmt::Debug>(list: LinkedList<T>, values: Vec<T>) {
    let mut index = 0;
    for value in list {
        assert_eq!(values[index], value);
        index += 1;
    }
    assert_eq!(values.len(), index);
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
