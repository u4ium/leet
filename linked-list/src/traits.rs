use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use super::*;

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut length = 0;
        let mut head = None;
        let mut current = &mut head;
        for value in iterable {
            length += 1;
            *current = LinkedListNode::new(value, None);
            current = &mut current.as_mut().unwrap().next;
        }
        LinkedList { head, length }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIntoIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIntoIterator::new(self.head)
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        index_check(index, self.length);
        get_element(&self.head, index)
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        index_check(index, self.length);
        get_element_mut(&mut self.head, index)
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut last = self.get_end_mut();
        let mut new_element_count = 0;
        for value in iterable {
            *last = LinkedListNode::new(value, None);
            last = &mut last.as_mut().unwrap().next;
            new_element_count += 1;
        }
        self.length += new_element_count;
    }
}
