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

struct Tree {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl From<&[Option<i32>]> for Tree {
    fn from(node_list: &[Option<i32>]) -> Self {
        fn make_node(node_list: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if index >= node_list.len() {
                return None;
            }
            node_list[index].and_then(|val| {
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: make_node(node_list, index * 2 + 1),
                    right: make_node(node_list, index * 2 + 2),
                })))
            })
        }
        Tree {
            root: make_node(node_list, 0),
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, is_left_child: bool) -> i32 {
            match node {
                Some(node_cell) => match &*node_cell.borrow() {
                    TreeNode {
                        val,
                        left: None,
                        right: None,
                    } if is_left_child => *val,
                    TreeNode { left, right, .. } => helper(left, true) + helper(right, false),
                },
                None => 0,
            }
        }
        helper(&root, false)
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
        let root =
            Tree::from(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)][..]).root;
        assert_eq!(24, Solution::sum_of_left_leaves(root));
    }

    #[test]
    fn example_2() {
        let root = Tree::from(&[Some(1)][..]).root;
        assert_eq!(0, Solution::sum_of_left_leaves(root));
    }
}
