use crate::{LinkedListNode, LinkedListNodeRef};

/// An iterator over a LinkedList
pub struct LinkedListIntoIterator<T> {
    current: LinkedListNodeRef<T>,
}

impl<T> LinkedListIntoIterator<T> {
    #[inline]
    pub fn new(current: LinkedListNodeRef<T>) -> Self {
        LinkedListIntoIterator { current }
    }
}

impl<T> Iterator for LinkedListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().and_then(|node| {
            self.current = node.next;
            Some(node.val)
        })
    }
}

/// An iterator over a LinkedList, by reference
pub struct LinkedListIterator<'a, T> {
    current: &'a LinkedListNodeRef<T>,
}

impl<'a, T> LinkedListIterator<'a, T> {
    #[inline]
    pub fn new(current: &'a LinkedListNodeRef<T>) -> Self {
        LinkedListIterator { current }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.as_ref().and_then(|node| {
            self.current = &node.next;
            Some(&node.val)
        })
    }
}

/// An iterator over a LinkedList, by reference
pub struct LinkedListIteratorMut<'a, T> {
    current: Option<&'a mut LinkedListNode<T>>,
}

impl<'a, T> LinkedListIteratorMut<'a, T> {
    #[inline]
    pub fn new(current: Option<&'a mut LinkedListNode<T>>) -> Self {
        LinkedListIteratorMut { current }
    }
}

impl<'a, T> Iterator for LinkedListIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().and_then(|node| {
            self.current = node.next.as_mut().map(|n| &mut **n);
            Some(&mut node.val)
        })
    }
}
