struct Solution {}

use std::collections::HashMap;

impl Solution {
    /// Given a string paragraph and a string array of the banned words banned,
    /// return the most frequent word that is not banned.
    ///
    /// It is guaranteed there is at least one word that is not banned, and that the answer is unique.
    /// The words in paragraph are case-insensitive and the answer should be returned in lowercase.
    ///
    /// REQ:
    ///     - 1 <= paragraph.length <= 1000
    ///     - paragraph consists of English letters, space ' ', or one of the symbols: "!?',;.".
    ///     - 0 <= banned.length <= 100
    ///     - 1 <= banned[i].length <= 10
    ///     - banned[i] consists of only lowercase English letters.
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut hashmap: HashMap<String, u32> = HashMap::new();
        for word in paragraph
            .split(|c: char| !c.is_alphabetic())
            .map(|w| w.to_lowercase())
        {
            if banned.contains(&word) || word.len() == 0 {
                continue;
            }
            if let Some(value) = hashmap.get_mut(&word) {
                *value += 1;
            } else {
                hashmap.insert(word, 1);
            }
        }
        let mut occurrences = 0;
        let mut ret = String::new();
        for (word, &value) in hashmap.iter() {
            if value > occurrences {
                occurrences = value;
                ret = word.clone();
            }
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
        let paragraph = String::from("Bob hit a ball, the hit BALL flew far after it was hit.");
        let banned = vec![String::from("hit")];
        assert_eq!("ball", Solution::most_common_word(paragraph, banned));
    }

    #[test]
    fn example_2() {
        let paragraph = String::from("a.");
        let banned = vec![];
        assert_eq!("a", Solution::most_common_word(paragraph, banned));
    }

    #[test]
    fn extra() {
        let paragraph = String::from("a, a, a, a, b,b,b,c, c");
        let banned = vec![String::from("a")];
        assert_eq!("b", Solution::most_common_word(paragraph, banned));
    }
}
