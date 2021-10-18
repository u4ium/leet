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

#[derive(Debug)]
enum Found {
    U, // Uncle
    C, // Cousin
    X(u32),
    Y(u32),
    S, // Sibling
    N, // Not Found
}
use Found::*;

impl Found {
    #[inline]
    fn is_complete(&self) -> bool {
        match self {
            C | S | U => true,
            _ => false,
        }
    }
}

fn find_depth_of_two_values(node: &Option<TreeNodeCell>, x: i32, y: i32, depth: u32) -> Found {
    if let Some(node) = node {
        let value = node.borrow().val;
        if x == value {
            return X(depth);
        }
        if y == value {
            return Y(depth);
        }
        let left = find_depth_of_two_values(&node.borrow().left, x, y, depth + 1);
        if left.is_complete() {
            return left;
        }
        let right = find_depth_of_two_values(&node.borrow().right, x, y, depth + 1);
        if right.is_complete() {
            return right;
        }
        match (left, right) {
            (X(x_depth), Y(y_depth)) | (Y(y_depth), X(x_depth)) => {
                if x_depth == y_depth {
                    if x_depth == depth + 1 {
                        S
                    } else {
                        C
                    }
                } else {
                    U
                }
            }
            (x @ X(_), N) | (N, x @ X(_)) => x,
            (y @ Y(_), N) | (N, y @ Y(_)) => y,
            (N, N) => N,
            (left @ _, right @ _) => panic!("Unexpected Found variants. Completed Found variants should be returned early. Both x and y should be unique: left={:?}, right={:?}", left, right),
        }
    } else {
        N
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        if let C = find_depth_of_two_values(&root, x, y, 0) {
            true
        } else {
            false
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
        // Input: root = [1,2,3,4], x = 4, y = 3
        // Output: false
        let tree = Tree::from(&[Some(1), Some(2), Some(3), Some(4)][..]);
        assert_eq!(false, Solution::is_cousins(tree.root, 4, 3));
    }

    #[test]
    fn example_2() {
        // Input: root = [1,2,3,null,4,null,5], x = 5, y = 4
        // Output: true
        let tree = Tree::from(&[Some(1), Some(2), Some(3), None, Some(4), None, Some(5)][..]);
        assert_eq!(true, Solution::is_cousins(tree.root, 5, 4));
    }

    #[test]
    fn example_3() {
        // Input: root = [1,2,3,null,4], x = 2, y = 3
        // Output: false
        let tree = Tree::from(&[Some(1), Some(2), Some(3), None, Some(4)][..]);
        assert_eq!(false, Solution::is_cousins(tree.root, 2, 3));
    }
}
