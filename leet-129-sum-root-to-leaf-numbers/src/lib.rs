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
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn sum_to_leaf(node: &Option<Rc<RefCell<TreeNode>>>, parent_sum: i32) -> i32 {
            match node {
                Some(n) => {
                    let current_sum = 10 * parent_sum + n.borrow().val;
                    let right_sum = sum_to_leaf(&n.borrow().right, current_sum);
                    let left_sum = sum_to_leaf(&n.borrow().left, current_sum);
                    let children_sum = right_sum + left_sum;
                    if children_sum > 0 {
                        children_sum
                    } else {
                        current_sum
                    }
                }
                None => 0,
            }
        }
        sum_to_leaf(&root, 0)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn example_1() {
        let root = Tree::from(&[Some(1), Some(2), Some(3)][..]).root;
        assert_eq!(25, Solution::sum_numbers(root));
        // Explanation:
        // The root-to-leaf path 1->2 represents the number 12.
        // The root-to-leaf path 1->3 represents the number 13.
        // Therefore, sum = 12 + 13 = 25.
    }

    #[test]
    fn example_2() {
        let root = Tree::from(&[Some(4), Some(9), Some(0), Some(5), Some(1)][..]).root;
        assert_eq!(1026, Solution::sum_numbers(root));
        // Explanation:
        // The root-to-leaf path 4->9->5 represents the number 495.
        // The root-to-leaf path 4->9->1 represents the number 491.
        // The root-to-leaf path 4->0 represents the number 40.
        // Therefore, sum = 495 + 491 + 40 = 1026.
    }
}
