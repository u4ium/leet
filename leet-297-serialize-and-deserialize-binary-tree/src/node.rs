// Definition for a binary tree node.
use std::ops::Deref;

use std::cell::RefCell;
use std::rc::Rc;

pub type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn new_node(node_list: &[Option<i32>], index: usize) -> TreeNodeRef {
    if index >= node_list.len() {
        None
    } else if let Some(val) = node_list[index] {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: new_node(node_list, index * 2 + 1),
            right: new_node(node_list, index * 2 + 2),
        })))
    } else {
        None
    }
}

pub struct Tree {
    pub root: TreeNodeRef,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }
}

impl Deref for Tree {
    type Target = TreeNodeRef;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.root
    }
}

impl From<TreeNodeRef> for Tree {
    fn from(root: TreeNodeRef) -> Self {
        Tree { root }
    }
}

impl From<&[Option<i32>]> for Tree {
    fn from(node_list: &[Option<i32>]) -> Self {
        Tree {
            root: new_node(node_list, 0),
        }
    }
}
