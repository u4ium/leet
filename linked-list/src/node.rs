/// An alias for a pointer to a LinkedListNode
pub type LinkedListNodeRef<T> = Option<Box<LinkedListNode<T>>>;

/// A node in a linked list that stores a single value and a pointer to the next node
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinkedListNode<T> {
    /// The value stored in this node
    pub val: T,

    /// A pointer to the next node
    pub next: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedListNode<T> {
    /// Create a new LinkedListNode
    #[inline]
    pub fn new(val: T, next: LinkedListNodeRef<T>) -> Option<Box<Self>> {
        Some(Box::new(LinkedListNode { next, val }))
    }
}

#[inline]
pub fn get_element<T>(current: &LinkedListNodeRef<T>, index: usize) -> &T {
    &get_node(current, index).as_ref().unwrap().val
}

#[inline]
pub fn get_node<T>(mut current: &LinkedListNodeRef<T>, index: usize) -> &LinkedListNodeRef<T> {
    for _ in 0..index {
        current = &current.as_ref().unwrap().next;
    }
    current
}

#[inline]
pub fn get_element_mut<T>(current: &mut LinkedListNodeRef<T>, index: usize) -> &mut T {
    &mut get_node_mut(current, index).as_mut().unwrap().val
}

#[inline]
pub fn get_node_mut<T>(
    mut current: &mut LinkedListNodeRef<T>,
    index: usize,
) -> &mut LinkedListNodeRef<T> {
    for _ in 0..index {
        current = &mut current.as_mut().unwrap().next;
    }
    current
}
