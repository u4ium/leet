use leet_208_prefix_trie::Trie;

fn main() {
    let word = String::from("ello");
    let mut obj = Trie::new();
    obj.insert(word.clone());
    obj.search(word);
    obj.starts_with(String::from("hell"));
}
