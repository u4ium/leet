mod linked_list;
use linked_list::ListNode;
struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = &mut head;
        while let Some(node) = current {
            if node.val == val {
                *current = current.take().unwrap().next;
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::{linked_list::LinkedList, Solution};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn example_1() {
        let mut actual: LinkedList = [1, 2, 6, 3, 4, 5, 6].iter().cloned().collect();
        actual.head = Solution::remove_elements(actual.head, 6);
        let expect: LinkedList = [1, 2, 3, 4, 5].iter().cloned().collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn example_2() {
        let mut actual: LinkedList = [].iter().cloned().collect();
        actual.head = Solution::remove_elements(actual.head, 1);
        let expect: LinkedList = [].iter().cloned().collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn example_3() {
        let mut actual: LinkedList = [7, 7, 7, 7].iter().cloned().collect();
        actual.head = Solution::remove_elements(actual.head, 7);
        let expect: LinkedList = [].iter().cloned().collect();
        assert_eq!(expect, actual);
    }
}
