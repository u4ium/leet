//! A Linked List implementation
//!
//! **NOTE**: This is *not* an efficient data structure. Do *not* use in production code.

#[cfg(test)]
mod tests;

mod node;
use node::*;

mod iterators;
use iterators::*;

mod traits;

/// A growable linked list data structure, supporting push, pop, indexing and iteration.
///
/// **NOTE**: This is *not* an efficient data structure. Do *not* use in production code.
///
/// # Examples
///
/// ## As a Stack
/// ```rust
/// # use linked_list::LinkedList;
/// let mut stack = LinkedList::new();
/// stack.push_front(5);
/// assert_eq!(Some(5), stack.pop_front());
/// assert_eq!(None, stack.pop_front());
/// ```
///
/// ## As an Iterable
/// ```rust
/// # use linked_list::LinkedList;
/// let list: LinkedList<char> = ('a'..='z').collect();
/// for (index, elem) in list.into_iter().enumerate() {
///     println!("Element {}: {:?}", index, elem); // Prints a-z
///     # assert_eq!(elem, (b'a' + index as u8) as char);
/// }
/// ```
///
/// ## Using indices
/// ```rust
/// # use linked_list::LinkedList;
/// const IRRATIONALS: [f64; 3] = [1.61803, 2.71828, 3.14159];
/// let mut list: LinkedList<f64> = IRRATIONALS.iter().cloned().collect();
/// assert_eq!(list[0], 1.61803);
/// list[0] = 0.83462;
/// assert_eq!(list[0], 0.83462);
/// ```
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct LinkedList<T> {
    /// The head of this LinkedList
    head: LinkedListNodeRef<T>,
    /// The current length of this LinkedList
    length: usize,
    // TODO: add tail pointer (using Rc) to enable O(1) push
}

// TODO: add size check to prevent usize overflow?

impl<T> LinkedList<T> {
    /// Create new (empty) LinkedList
    #[inline]
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    /// Return the number of elements in this list
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    /// Return an iterator (of references) over the elements in this list
    #[inline]
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator::new(&self.head)
    }

    /// Add given value to the front of the list
    pub fn push_front(&mut self, value: T) {
        self.head = LinkedListNode::new(value, self.head.take());
        self.length += 1;
    }

    /// Remove and return the first value in the list, if it exists, else None
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().and_then(|node| {
            self.head = node.next;
            self.length -= 1;
            Some(node.val)
        })
    }

    /// Add given value to the back of the list
    pub fn push(&mut self, value: T) {
        *self.get_end_mut() = LinkedListNode::new(value, None);
        self.length += 1;
    }

    /// Remove and return the last value in the list, if it exists, else None
    pub fn pop(&mut self) -> Option<T> {
        self.get_last_node_mut()
            .and_then(|node_ref| node_ref.take())
            .map(|node| {
                self.length -= 1;
                node.val
            })
    }

    /// Return a reference to list[index], if it exists, else None
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None; // TODO: factor out index checking into macro
        }
        Some(get_element(&self.head, index))
    }

    /// Return a mutable reference to list[index], if it exists, else None
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            return None;
        }
        Some(get_element_mut(&mut self.head, index))
    }

    /// Remove the element at the given index, shifting following elements to the left
    ///
    /// # Panics
    /// If index is greater than or equal to the size of this list
    pub fn remove(&mut self, index: usize) -> T {
        index_check(index, self.length);
        let node_to_remove_ref = get_node_mut(&mut self.head, index);
        let removed_node = node_to_remove_ref.take().unwrap();
        *node_to_remove_ref = removed_node.next;
        self.length -= 1;
        return removed_node.val;
    }

    /// Extend this linked list with another, to be added at the end
    pub fn extend_with_linked_list(&mut self, other: LinkedList<T>) {
        *self.get_end_mut() = other.head;
        self.length += other.length;
    }

    #[inline]
    fn get_end_mut(&mut self) -> &mut LinkedListNodeRef<T> {
        get_node_mut(&mut self.head, self.length)
    }

    #[inline]
    fn get_last_node_mut(&mut self) -> Option<&mut LinkedListNodeRef<T>> {
        if self.length == 0 {
            return None;
        }
        Some(get_node_mut(&mut self.head, self.length - 1))
    }
}

#[inline]
fn index_check(index: usize, length: usize) {
    if index >= length {
        panic!("LinkedList: index out of bounds ({} >= {})", index, length);
    }
}
