mod node;
use node::ListNode;

use rand::{self, Rng};
use std::ops::Index;

pub type ListNodeCell = Option<Box<ListNode>>;

pub struct MyLinkedList {
    head: ListNodeCell,
    length: usize,
}

impl Index<usize> for MyLinkedList {
    type Output = i32;

    fn index(&self, mut index: usize) -> &Self::Output {
        let mut current = &self.head;
        while let Some(node) = current {
            if index == 0 {
                return &node.val;
            }
            current = &node.next;
            index -= 1;
        }
        panic!("Index out of bounds");
    }
}

impl MyLinkedList {
    pub fn new(head: ListNodeCell) -> Self {
        let length = Self::compute_length(&head);
        Self { head, length }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    fn compute_length(mut current: &ListNodeCell) -> usize {
        let mut length = 0;
        while let Some(node) = current {
            current = &node.next;
            length += 1;
        }
        length
    }
}

pub struct Solution {
    list: MyLinkedList,
}

impl Solution {
    pub fn new(head: ListNodeCell) -> Self {
        let list = MyLinkedList::new(head);
        Self { list }
    }

    pub fn get_random(&self) -> i32 {
        let random_index = rand::thread_rng().gen_range(0..self.list.len());
        self.list[random_index]
    }
}

#[cfg(test)]
mod tests {
    // use super::Solution;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
