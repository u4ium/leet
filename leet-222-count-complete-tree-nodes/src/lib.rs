struct Solution {}

// Definition for a binary tree node.
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

use std::cell::RefCell;
use std::rc::Rc;
type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

fn binary_tree_size(node: &TreeNodeRef) -> usize {
    match node {
        Some(node_ref) => {
            1   // recurse
                + binary_tree_size(&node_ref.borrow().left)
                + binary_tree_size(&node_ref.borrow().right)
        }
        None => 0, // base-case
    }
}

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        binary_tree_size(&root) as i32
    }
}

struct Tree {
    root: TreeNodeRef,
}

impl From<&[Option<i32>]> for Tree {
    fn from(array: &[Option<i32>]) -> Self {
        fn generate_elem(array: &[Option<i32>], index: usize) -> TreeNodeRef {
            if index >= array.len() {
                None
            } else if let Some(val) = array[index] {
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: generate_elem(array, index * 2 + 1),
                    right: generate_elem(array, index * 2 + 2),
                })))
            } else {
                None
            }
        }
        Tree {
            root: generate_elem(array, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn example_1() {
        let input = Tree::from(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)][..]);
        let output = 6;
        assert_eq!(output, Solution::count_nodes(input.root));
    }

    #[test]
    fn example_2() {
        let input = Tree::from(&[][..]);
        let output = 0;
        assert_eq!(output, Solution::count_nodes(input.root));
    }

    #[test]
    fn example_3() {
        let input = Tree::from(&[Some(1)][..]);
        let output = 1;
        assert_eq!(output, Solution::count_nodes(input.root));
    }
}
