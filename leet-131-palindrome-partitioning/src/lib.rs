pub struct Solution;

struct Searcher<'a> {
    /// the string being searched
    s: &'a [u8],
    /// dp[i][j] true if s[i..=j] is a palindrome
    is_palindrome_between: Vec<Vec<bool>>,
    ///
    current_partitioning: Vec<String>,
    /// the result
    palindrome_partitions: Vec<Vec<String>>,
}

impl<'a> Searcher<'a> {
    pub fn search(mut self) -> Vec<Vec<String>> {
        self.dfs(0);
        self.palindrome_partitions
    }

    fn dfs(&mut self, start: usize) {
        if start == self.s.len() {
            self.palindrome_partitions
                .push(self.current_partitioning.clone());
            return;
        }

        // for every possible palindrome starting here
        for end in start..self.s.len() {
            if self.s[start] == self.s[end]
                && (end - start <= 2 || self.is_palindrome_between[start + 1][end - 1])
            {
                self.is_palindrome_between[start][end] = true;
                let palindromic_substring =
                    String::from_utf8_lossy(&self.s[start..=end]).into_owned();

                self.current_partitioning.push(palindromic_substring);
                self.dfs(end + 1);
                self.current_partitioning.pop();
            }
        }
    }
}

impl Solution {
    /// Return all possible palindrome partitionings of s.
    ///
    /// Given a string s, partition s such that every substring of the partition is a palindrome.
    /// A palindrome string is a string that reads the same backward as forward.
    /// Constraints:
    ///  - 1 <= s.length <= 16
    ///  - s contains only lowercase English letters.
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let searcher = Searcher {
            s: s.as_bytes(),
            is_palindrome_between: vec![vec![false; s.len()]; s.len()],
            current_partitioning: vec![],
            palindrome_partitions: vec![],
        };
        searcher.search()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn example_1() {
        let input = String::from("aab");
        let expect = vec![vec!["a", "a", "b"], vec!["aa", "b"]];
        assert_eq!(expect, Solution::partition(input));
    }

    #[test]
    fn example_2() {
        let input = String::from("a");
        let expect = vec![vec!["a"]];
        assert_eq!(expect, Solution::partition(input));
    }

    #[test]
    fn extra() {
        let input = String::from("cdd");
        let expect = vec![vec!["c", "d", "d"], vec!["c", "dd"]];
        assert_eq!(expect, Solution::partition(input));
    }
}
