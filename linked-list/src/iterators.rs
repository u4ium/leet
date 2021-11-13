use super::LinkedListNodeRef;

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
