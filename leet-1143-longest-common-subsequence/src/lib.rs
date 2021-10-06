struct Solution {}

impl Solution {
    /// Given two strings text1 and text2, return the length of their longest common subsequence.
    /// If there is no common subsequence, return 0.
    ///
    /// REQ:
    ///     - 1 <= text1.length, text2.length <= 1000
    ///     - text1 and text2 consist of only lowercase English characters.
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut max_using = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for (index_1, char_1) in text1.as_bytes().iter().enumerate() {
            for (index_2, char_2) in text2.as_bytes().iter().enumerate() {
                if char_1 == char_2 {
                    max_using[index_1 + 1][index_2 + 1] = max_using[index_1][index_2] + 1;
                } else {
                    max_using[index_1 + 1][index_2 + 1] = std::cmp::max(
                        max_using[index_1][index_2 + 1],
                        max_using[index_1 + 1][index_2],
                    );
                }
            }
        }
        max_using[text1.len()][text2.len()]
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
        assert_eq!(
            Solution::longest_common_subsequence(String::from("abcde"), String::from("ace")),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_common_subsequence(String::from("abc"), String::from("abc")),
            3
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::longest_common_subsequence(String::from("abc"), String::from("def")),
            0
        );
    }
}
