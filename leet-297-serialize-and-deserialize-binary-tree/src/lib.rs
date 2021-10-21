mod node;
use node::*;
use std::ops::Deref;

use std::cell::RefCell;
use std::fmt::Write;
use std::iter::repeat;
use std::rc::Rc;

type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

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

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize_helper(node: &TreeNodeRef, output: &mut Vec<Option<i32>>, index: usize) {
        if let Some(node) = node {
            let node = node.borrow();
            if index >= output.len() {
                output.extend(repeat(None).take(output.len() + 1))
            }
            output[index] = Some(node.val);
            Codec::serialize_helper(&node.left, output, index * 2 + 1);
            Codec::serialize_helper(&node.right, output, index * 2 + 2);
        }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut node_list = Vec::new();
        Codec::serialize_helper(&root, &mut node_list, 0);
        let mut result = String::new();
        write!(
            &mut result,
            "[{}]",
            node_list
                .iter()
                .map(|v| match v {
                    Some(value) => value.to_string(),
                    None => String::from("null"),
                })
                .collect::<Vec<_>>()
                .join(",")
        )
        .unwrap();
        result
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn parse_value(chars: &[u8]) -> Option<i32> {
            std::str::from_utf8(chars).unwrap().parse::<i32>().ok()
        }

        fn is_comma(c: &u8) -> bool {
            *c == b','
        }

        match data.as_bytes() {
            [b'[', values @ .., b']'] => new_node(
                &values.split(is_comma).map(parse_value).collect::<Vec<_>>()[..],
                0,
            ),
            _ => panic!("Invalid tree string: must be surrounded by []"),
        }
    }
}

struct Tree {
    root: TreeNodeRef,
}

impl Tree {
    fn new() -> Self {
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

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_example_1_serialize() {
        let serialized = String::from("[1,2,3,null,null,4,5]");
        let tree = Tree::from(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)][..]);
        let codec = Codec::new();
        assert_eq!(serialized, codec.serialize(tree.root));
    }

    #[test]
    fn test_example_1_deserialize() {
        let serialized = String::from("[1,2,3,null,null,4,5]");
        let tree = Tree::from(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)][..]);
        let codec = Codec::new();
        assert_eq!(tree.root, codec.deserialize(serialized));
    }

    #[test]
    fn test_example_2_serialize() {
        let serialized = String::from("[]");
        let tree = Tree::from(&[][..]);
        let codec = Codec::new();
        assert_eq!(serialized, codec.serialize(tree.root));
    }

    #[test]
    fn test_example_2_deserialize() {
        let serialized = String::from("[]");
        let tree = Tree::from(&[][..]);
        let codec = Codec::new();
        assert_eq!(tree.root, codec.deserialize(serialized));
    }

    #[test]
    fn test_example_3_serialize() {
        let serialized = String::from("[1]");
        let tree = Tree::from(&[Some(1)][..]);
        let codec = Codec::new();
        assert_eq!(serialized, codec.serialize(tree.root));
    }

    #[test]
    fn test_example_3_deserialize() {
        let serialized = String::from("[1]");
        let tree = Tree::from(&[Some(1)][..]);
        let codec = Codec::new();
        assert_eq!(tree.root, codec.deserialize(serialized));
    }

    #[test]
    fn test_example_4_serialize() {
        let serialized = String::from("[1,2,null]");
        let tree = Tree::from(&[Some(1), Some(2), None][..]);
        let codec = Codec::new();
        assert_eq!(serialized, codec.serialize(tree.root));
    }

    #[test]
    fn test_example_4_deserialize() {
        let serialized = String::from("[1,2,null]");
        let tree = Tree::from(&[Some(1), Some(2), None][..]);
        let codec = Codec::new();
        assert_eq!(tree.root, codec.deserialize(serialized));
    }
}
