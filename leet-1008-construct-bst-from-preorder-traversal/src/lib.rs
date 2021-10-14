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
    root: Option<TreeNodeCell>,
}

impl From<&[Option<i32>]> for Tree {
    fn from(array: &[Option<i32>]) -> Self {
        fn generate_elem(array: &[Option<i32>], index: usize) -> Option<TreeNodeCell> {
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

use std::cell::RefCell;
use std::rc::Rc;

type TreeNodeCell = Rc<RefCell<TreeNode>>;

impl TreeNode {
    pub fn new_cell(val: i32) -> TreeNodeCell {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }
}

impl Solution {
    /// Construct the tree and return its root, given an array of integers preorder,
    /// which represents the preorder traversal of a BST (i.e., binary search tree).
    /// REQ:
    ///     -> 1 <= preorder.length <= 100
    ///     -> 1 <= preorder[i] <= 10^8
    ///     -> All the values of preorder are unique.
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values: Vec<i32> = preorder.into_iter().rev().collect();
        let first = values.pop().unwrap();
        let root = TreeNode::new_cell(first);
        bst_from_preorder(root, &mut values, None)
    }
}

fn bst_from_preorder(
    root: TreeNodeCell,
    values: &mut Vec<i32>,
    max: Option<i32>,
) -> Option<TreeNodeCell> {
    let root_val = root.borrow().val;
    while let Some(first) = values.pop() {
        if first < root_val {
            root.borrow_mut().left =
                bst_from_preorder(TreeNode::new_cell(first), values, Some(root_val));
        } else if max.is_none() || first < max.unwrap() {
            root.borrow_mut().right = bst_from_preorder(TreeNode::new_cell(first), values, max);
        } else {
            values.push(first);
            break;
        }
    }
    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_single_elem_tree() {
        assert_eq!(
            Some(TreeNode::new_cell(1)),
            Solution::bst_from_preorder(vec![1])
        );
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            Tree::from(&[Some(8), Some(5), Some(10), Some(1), Some(7), None, Some(12)][..]).root,
            Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12])
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Tree::from(&[Some(1), None, Some(3)][..]).root,
            Solution::bst_from_preorder(vec![1, 3])
        );
    }

    #[test]
    fn test_Y_tree() {
        assert_eq!(
            Tree::from(
                &[
                    Some(4),
                    Some(3),
                    Some(5),
                    Some(2),
                    None,
                    None,
                    Some(6),
                    Some(1),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(7)
                ][..]
            )
            .root,
            Solution::bst_from_preorder(vec![4, 3, 2, 1, 5, 6, 7])
        );
    }
}
