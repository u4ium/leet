use std::iter::FromIterator;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
}

impl FromIterator<i32> for LinkedList {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = i32>,
    {
        let mut head = None;
        let mut current = &mut head;
        for value in iterable {
            *current = Some(Box::new(ListNode::new(value)));
            current = &mut current.as_mut().unwrap().next;
        }
        LinkedList { head }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_from_iterator_empty() {
        let expect = LinkedList { head: None };
        let actual = [].iter().cloned().collect::<LinkedList>();
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_from_iterator_1_element() {
        let expect = LinkedList {
            head: Some(Box::new(ListNode {
                val: 14,
                next: None,
            })),
        };
        let actual = [14].iter().cloned().collect::<LinkedList>();
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_from_iterator_multiple_elements() {
        let expect = LinkedList {
            head: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };
        let actual = [1, 4].into_iter().cloned().collect::<LinkedList>();
        assert_eq!(expect, actual);
    }
}
