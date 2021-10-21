mod node;
use node::*;

use std::cell::RefCell;
use std::fmt::Write as FormatWrite;
use std::rc::Rc;

struct Codec {}

fn serialize_helper(node: &TreeNodeRef) -> String {
    match node {
        Some(root_node) => {
            let root_node = root_node.borrow();
            let mut result = String::new();
            write!(
                result,
                "({},{},{})",
                root_node.val,
                serialize_helper(&root_node.left),
                serialize_helper(&root_node.right)
            )
            .unwrap();
            result
        }
        None => String::from(""),
    }
}

fn deserialize_helper(data: &[u8]) -> TreeNodeRef {
    match data {
        [] => None,
        [b'(', middle @ .., b')'] => {
            let values = middle.split(|&c| c == b',').collect::<Vec<_>>();
            if values.len() != 3 {
                println!("{:?}", values);
                panic!("Bad tree repr element: Cannot deserialize")
            }
            let val = unsafe {
                std::str::from_utf8_unchecked(values[0])
                    .parse()
                    .expect("Bad tree repr value: Cannot deserialize")
            };
            let left = values[1];
            let right = values[2];
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: deserialize_helper(left),
                right: deserialize_helper(right),
            })))
        }
        _ => panic!("Bad tree repr: Cannot deserialize"),
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        serialize_helper(&root)
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        deserialize_helper(data.as_bytes())
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
        let serialized = String::from("(1,(2,,),(3,(4,,),(5,,)))");
        let tree = Tree::from(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)][..]);
        let codec = Codec::new();
        assert_eq!(serialized, codec.serialize(tree.root));
    }

    #[test]
    fn test_example_1_deserialize() {
        let serialized = String::from("(1,(2,,),(3,(,4,,),(5,,)))");
        let tree = Tree::from(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)][..]);
        let codec = Codec::new();
        assert_eq!(tree.root, codec.deserialize(serialized));
    }

    #[test]
    fn test_example_2_serialize() {
        let serialized = String::from("");
        let tree = Tree::from(&[][..]);
        let codec = Codec::new();
        assert_eq!(serialized, codec.serialize(tree.root));
    }

    #[test]
    fn test_example_2_deserialize() {
        let serialized = String::from("");
        let tree = Tree::from(&[][..]);
        let codec = Codec::new();
        assert_eq!(tree.root, codec.deserialize(serialized));
    }

    #[test]
    fn test_example_3_serialize() {
        let serialized = String::from("(1,,)");
        let tree = Tree::from(&[Some(1)][..]);
        let codec = Codec::new();
        assert_eq!(serialized, codec.serialize(tree.root));
    }

    #[test]
    fn test_example_3_deserialize() {
        let serialized = String::from("(1,,)");
        let tree = Tree::from(&[Some(1)][..]);
        let codec = Codec::new();
        assert_eq!(tree.root, codec.deserialize(serialized));
    }

    #[test]
    fn test_example_4_serialize() {
        let serialized = String::from("(1,(2,,),)");
        let tree = Tree::from(&[Some(1), Some(2), None][..]);
        let codec = Codec::new();
        assert_eq!(serialized, codec.serialize(tree.root));
    }

    #[test]
    fn test_example_4_deserialize() {
        let serialized = String::from("(1,(2,,),)");
        let tree = Tree::from(&[Some(1), Some(2), None][..]);
        let codec = Codec::new();
        assert_eq!(tree.root, codec.deserialize(serialized));
    }
}
