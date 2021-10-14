struct Solution {}

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

type TreeNodeCell = Rc<RefCell<TreeNode>>;

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
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_valid_bst_helper(&root, None, None)
    }
}

fn is_valid_bst_helper(root: &Option<TreeNodeCell>, min: Option<i32>, max: Option<i32>) -> bool {
    match root {
        None => true,
        Some(node) => {
            let tree_node = node.borrow();
            if let Some(max_value) = max {
                if tree_node.val >= max_value {
                    return false;
                }
            }
            if let Some(min_value) = min {
                if tree_node.val <= min_value {
                    return false;
                }
            }
            if let Some(_) = &tree_node.left {
                let max = Some(tree_node.val);
                if !is_valid_bst_helper(&tree_node.left, min, max) {
                    return false;
                }
            }
            if let Some(_) = &tree_node.right {
                let min = Some(tree_node.val);
                if !is_valid_bst_helper(&tree_node.right, min, max) {
                    return false;
                }
            }
            true
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
    fn test_initialization_1() {
        let tree = Tree::from(&[Some(2), Some(1), Some(3)][..]);
        let root_node = tree.root.unwrap();
        let root_node = root_node.borrow();
        assert_eq!(root_node.val, 2);
        let left_node = root_node.left.as_ref();
        assert_eq!(left_node.unwrap().borrow().val, 1);
        let right_node = root_node.right.as_ref();
        assert_eq!(right_node.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_initialization_2() {
        let tree = Tree::from(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)][..]);
        let root_node = tree.root.unwrap();
        let root_node = root_node.borrow();
        assert_eq!(root_node.val, 5);

        let left_node = root_node.left.as_ref();
        let left_node = left_node.unwrap().borrow();
        assert_eq!(left_node.val, 1);
        assert_eq!(left_node.left, None);
        assert_eq!(left_node.right, None);

        let right_node = root_node.right.as_ref();
        let right_node = right_node.unwrap().borrow();
        assert_eq!(right_node.val, 4);
        let right_right_node = right_node.right.as_ref();
        let right_right_node = right_right_node.unwrap().borrow();
        assert_eq!(right_right_node.val, 6);
        let left_right_node = right_node.left.as_ref();
        let left_right_node = left_right_node.unwrap().borrow();
        assert_eq!(left_right_node.val, 3);
    }

    #[test]
    fn example_1() {
        let tree = Tree::from(&[Some(2), Some(1), Some(3)][..]);
        assert_eq!(true, Solution::is_valid_bst(tree.root));
    }

    #[test]
    fn example_2() {
        let tree = Tree::from(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)][..]);
        assert_eq!(false, Solution::is_valid_bst(tree.root));
    }

    #[test]
    fn my_example() {
        let tree = Tree::from(&[Some(2), Some(1), Some(4), None, None, Some(3), Some(6)][..]);
        assert_eq!(true, Solution::is_valid_bst(tree.root));
    }

    #[test]
    fn my_example_2() {
        let tree = Tree::from(&[Some(2), Some(1), Some(4), None, None, Some(7), Some(6)][..]);
        assert_eq!(false, Solution::is_valid_bst(tree.root));
    }

    #[test]
    fn extra() {
        let tree = Tree::from(&[Some(5), Some(4), Some(6), None, None, Some(3), Some(7)][..]);
        assert_eq!(false, Solution::is_valid_bst(tree.root));
    }
}
