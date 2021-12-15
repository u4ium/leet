mod list;
use list::*;

pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list = List::new();

        while let Some(mut node) = head {
            head = node.next.take();
            new_list.insert_ordered(node);
        }

        new_list.head
    }
}

#[cfg(test)]
mod tests {
    use super::{List, Solution};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn example_1() {
        let list = List::from_iter([4, 2, 1, 3]);
        let expect = List::from_iter([1, 2, 3, 4]).head;
        assert_eq!(expect, Solution::insertion_sort_list(list.head))
    }

    #[test]
    fn example_2() {
        let list = List::from_iter([-1, 5, 3, 4, 0]);
        let expect = List::from_iter([-1, 0, 3, 4, 5]).head;
        assert_eq!(expect, Solution::insertion_sort_list(list.head))
    }
}
