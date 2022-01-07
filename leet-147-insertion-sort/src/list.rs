pub struct List {
    pub head: Option<Box<ListNode>>,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert_ordered(&mut self, new_node: Box<ListNode>) {
        self.head = insert_ordered(self.head.take(), new_node);
    }
}

pub fn insert_ordered(
    current: Option<Box<ListNode>>,
    mut new_node: Box<ListNode>,
) -> Option<Box<ListNode>> {
    match current {
        Some(mut node) if node.val < new_node.val => {
            node.next = insert_ordered(node.next.take(), new_node);
            Some(node)
        }
        _ => {
            new_node.next = current;
            Some(new_node)
        }
    }
}

impl FromIterator<i32> for List {
    fn from_iter<I: IntoIterator<Item = i32>>(iterable: I) -> Self {
        let mut head = None;
        let mut current = &mut head;

        for val in iterable {
            *current = Some(Box::new(ListNode { val, next: None }));
            current = &mut current.as_mut().unwrap().next;
        }

        List { head }
    }
}

// Definition for singly-linked list.
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
