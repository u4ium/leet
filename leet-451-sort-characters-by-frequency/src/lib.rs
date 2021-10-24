struct Solution {}

use std::collections::BTreeMap;
use std::iter::repeat;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut chars: BTreeMap<char, usize> = BTreeMap::new();
        for c in s.chars() {
            *chars.entry(c).or_default() += 1;
        }
        let mut ret: Vec<(char, usize)> = chars.into_iter().collect();
        ret.sort_by(|(_, n1), (_, n2)| n1.cmp(&n2));
        ret.into_iter()
            .map(|(c, n)| repeat(c).take(n).collect::<String>())
            .rev()
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

    fn is_frequency_sort_correct<const N: usize>(possibilities: [&str; N], input: &str) -> bool {
        possibilities.contains(&Solution::frequency_sort(input.to_owned()).as_str())
    }

    #[test]
    fn example_1() {
        assert!(is_frequency_sort_correct(["eert", "eetr"], "tree"));
    }

    #[test]
    fn example_2() {
        assert!(is_frequency_sort_correct(["cccaaa", "aaaccc"], "cccaaa"));
    }

    #[test]
    fn example_3() {
        assert!(is_frequency_sort_correct(["bbAa", "bbaA"], "Aabb"));
    }
}
