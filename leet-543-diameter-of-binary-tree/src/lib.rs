use std::cell::RefCell;
use std::rc::Rc;

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

use std::cmp::max;

fn max_height_and_diameter(node: &Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
    match node {
        None => [0, 0],
        Some(node) => {
            let [left_height, left_diameter] = max_height_and_diameter(&node.borrow().left);
            let [right_height, right_diameter] = max_height_and_diameter(&node.borrow().right);
            let diameter_through_node = left_height + right_height;
            let max_child_diameter = max(left_diameter, right_diameter);
            [
                max(left_height, right_height) + 1,
                max(diameter_through_node, max_child_diameter),
            ]
        }
    }
}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_height_and_diameter(&root)[1]
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
        let tree = Tree::from(&[Some(1), Some(2), Some(3), Some(4), Some(5)][..]);
        assert_eq!(3, Solution::diameter_of_binary_tree(tree.root));
    }

    #[test]
    fn example_2() {
        let tree = Tree::from(&[Some(1), Some(2)][..]);
        assert_eq!(1, Solution::diameter_of_binary_tree(tree.root));
    }

    #[test]
    fn extra() {
        let tree = Tree::from(
            &[
                Some(4),
                Some(-7),
                Some(-3),
                None,
                None,
                Some(-9),
                Some(-3),
                Some(9),
                Some(-7),
                Some(-4),
                None,
                Some(6),
                None,
                Some(-6),
                Some(-6),
                None,
                None,
                Some(0),
                Some(6),
                Some(5),
                None,
                Some(9),
                None,
                None,
                Some(-1),
                Some(-4),
                None,
                None,
                None,
                Some(-2),
            ][..],
        );
        println!("{:?}", tree.root);
        assert_eq!(8, Solution::diameter_of_binary_tree(tree.root));
        // This is not a valid Tree... WTF
        // [4,-7,-3,null,null,-9,-3,9,-7,-4,null,6,null,-6,-6,null,null,0,6,5,null,9,null,null,-1,-4,null,null,null,-2]
        // -> 8
    }
}
