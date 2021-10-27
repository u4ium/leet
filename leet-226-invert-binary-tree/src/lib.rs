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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let mut node_ref = node.borrow_mut();
            let left = node_ref.left.take();
            let right = node_ref.right.take();
            node_ref.right = Solution::invert_tree(left);
            node_ref.left = Solution::invert_tree(right);
        }
        root
    }
}

type TreeNodeCell = Option<Rc<RefCell<TreeNode>>>;
struct Tree {
    root: TreeNodeCell,
}

fn make_node(values: &[Option<i32>], index: usize) -> TreeNodeCell {
    if index >= values.len() {
        return None;
    }
    values[index].and_then(|val| {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: make_node(values, index * 2 + 1),
            right: make_node(values, index * 2 + 2),
        })))
    })
}

impl From<&[Option<i32>]> for Tree {
    fn from(values: &[Option<i32>]) -> Self {
        Tree {
            root: make_node(values, 0),
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
        let input = Tree::from(
            &[
                Some(4),
                Some(2),
                Some(7),
                Some(1),
                Some(3),
                Some(6),
                Some(9),
            ][..],
        );
        let output = Tree::from(
            &[
                Some(4),
                Some(7),
                Some(2),
                Some(9),
                Some(6),
                Some(3),
                Some(1),
            ][..],
        );
        assert_eq!(output.root, Solution::invert_tree(input.root));
    }

    #[test]
    fn example_2() {
        let input = Tree::from(&[Some(2), Some(1), Some(3)][..]);
        let output = Tree::from(&[Some(2), Some(3), Some(1)][..]);
        assert_eq!(output.root, Solution::invert_tree(input.root));
    }

    #[test]
    fn example_3() {
        let input = Tree::from(&[][..]);
        let output = Tree::from(&[][..]);
        assert_eq!(output.root, Solution::invert_tree(input.root));
    }
}
