struct TrieNodeData {
    children: [TrieNode; 26],
    terminal: bool,
}

type TrieNodeDataRef = Box<TrieNodeData>;

impl TrieNodeData {
    #[inline]
    fn new() -> TrieNodeDataRef {
        const EMPTY: TrieNode = TrieNode::Empty;
        Box::new(TrieNodeData {
            children: [EMPTY; 26],
            terminal: false,
        })
    }
}

enum TrieNode {
    Empty,
    Full(TrieNodeDataRef),
}

impl TrieNode {
    #[inline]
    pub fn new() -> Self {
        TrieNode::Full(TrieNodeData::new())
    }

    pub fn insert(&mut self, chars: &[u8]) {
        fn insert_helper(data: &mut TrieNodeData, chars: &[u8]) {
            if let Some((&first, rest)) = chars.split_first() {
                let index = TrieNode::char_to_index(first);
                data.children[index].insert(rest);
            } else {
                data.terminal = true;
            }
        }

        match self {
            TrieNode::Empty => {
                let mut new_data = TrieNodeData::new();
                insert_helper(&mut new_data, chars);
                *self = TrieNode::Full(new_data);
            }
            TrieNode::Full(data) => insert_helper(data, chars),
        }
    }

    pub fn find(&self, chars: &[u8]) -> Option<bool> {
        match self {
            TrieNode::Empty => None,
            TrieNode::Full(data) => match chars {
                [] => Some(data.terminal),
                [first, rest @ ..] => {
                    let index = TrieNode::char_to_index(*first);
                    data.children[index].find(rest)
                }
            },
        }
    }

    #[inline]
    fn char_to_index(c: u8) -> usize {
        (c - b'a').into()
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    #[inline]
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        self.root.insert(word.as_bytes());
    }

    pub fn search(&self, word: String) -> bool {
        self.root.find(word.as_bytes()).unwrap_or(false)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.root.find(prefix.as_bytes()).is_some()
    }
}

impl From<&[&str]> for Trie {
    fn from(words: &[&str]) -> Self {
        let mut ret = Trie::new();
        for word in words {
            ret.insert(word.to_string());
        }
        ret
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
        let mut trie = Trie::from(&["apple"][..]);
        assert_eq!(true, trie.search("apple".to_string()));
        assert_eq!(false, trie.search("app".to_string()));
        assert_eq!(true, trie.starts_with("app".to_string()));

        trie.insert("app".to_string());
        assert_eq!(true, trie.search("app".to_string()));
    }
}
