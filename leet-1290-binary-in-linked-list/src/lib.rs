mod node;
use node::ListNode;

struct LinkedListIterator {
    current: Option<Box<ListNode>>,
}

impl Iterator for LinkedListIterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        self.current.take().and_then(|node| {
            let ListNode { next, val } = *node;
            self.current = next;
            Some(val)
        })
    }
}

struct LinkedList {
    head: Option<Box<ListNode>>,
}

impl IntoIterator for LinkedList {
    type Item = i32;
    type IntoIter = LinkedListIterator;

    fn into_iter(self) -> LinkedListIterator {
        LinkedListIterator { current: self.head }
    }
}

impl FromIterator<i32> for LinkedList {
    fn from_iter<I: IntoIterator<Item = i32>>(iterable: I) -> Self {
        let mut head = None;
        {
            let mut current = &mut head;
            for val in iterable {
                *current = Some(Box::new(ListNode { val, next: None }));
                current = &mut current.as_deref_mut().unwrap().next;
            }
        }
        Self { head }
    }
}

pub struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        from_binary(LinkedList { head })
    }
}

fn from_binary<I: IntoIterator<Item = i32>>(iterable: I) -> i32 {
    iterable.into_iter().fold(0, |a, e| (a << 1) | e)
}

#[cfg(test)]
mod tests {
    use super::{LinkedList, Solution};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn example_1() {
        let list = LinkedList::from_iter(vec![1, 0, 1]);
        assert_eq!(5, Solution::get_decimal_value(list.head));
    }
    fn example_2() {
        let list = LinkedList::from_iter(vec![0]);
        assert_eq!(0, Solution::get_decimal_value(list.head));
    }

    fn example_3() {
        let list = LinkedList::from_iter(vec![1]);
        assert_eq!(1, Solution::get_decimal_value(list.head));
    }

    fn example_4() {
        let list = LinkedList::from_iter(vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(18880, Solution::get_decimal_value(list.head));
    }

    fn example_5() {
        let list = LinkedList::from_iter(vec![0, 0]);
        assert_eq!(0, Solution::get_decimal_value(list.head));
    }
}
