struct Solution {}

pub mod bitset;
use bitset::LetterBitSet;

use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut letter_permutation_counts: HashMap<LetterBitSet, i32> = HashMap::new();
        for word in words.into_iter() {
            let letters = word.bytes().collect();
            *letter_permutation_counts.entry(letters).or_default() += 1;
        }
        puzzles
            .into_iter()
            .map(|word| {
                let first = word.bytes().next().unwrap();
                word.bytes()
                    .collect::<LetterBitSet>()
                    .subsets()
                    .filter_map(|subset| {
                        if !subset.contains(first) {
                            return None;
                        }
                        letter_permutation_counts.get(&subset)
                    })
                    .sum()
            })
            .collect()
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
        assert_eq!(2 + 2, 4);
        let words = vec![
            String::from("aaaa"),
            String::from("asas"),
            String::from("able"),
            String::from("ability"),
            String::from("actt"),
            String::from("actor"),
            String::from("access"),
        ];
        let puzzles = vec![
            String::from("aboveyz"),
            String::from("abrodyz"),
            String::from("abslute"),
            String::from("absoryz"),
            String::from("actresz"),
            String::from("gaswxyz"),
        ];
        assert_eq!(
            vec![1, 1, 3, 2, 4, 0],
            Solution::find_num_of_valid_words(words, puzzles)
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(2 + 2, 4);
        let words = vec![
            String::from("apple"),
            String::from("pleas"),
            String::from("please"),
        ];
        let puzzles = vec![
            String::from("aelwxyz"),
            String::from("aelpxyz"),
            String::from("aelpsxy"),
            String::from("saelpxy"),
            String::from("xaelpsy"),
        ];
        assert_eq!(
            vec![0, 1, 3, 2, 0],
            Solution::find_num_of_valid_words(words, puzzles)
        )
    }
}
