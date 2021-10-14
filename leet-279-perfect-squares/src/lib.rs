struct Solution {}

use std::cmp::min;
use std::convert::TryInto;

impl Solution {
    /// Return the least number of perfect square numbers that sum to n.
    ///
    /// REQ: 1 <= n <= 10000
    pub fn num_squares(n: i32) -> i32 {
        let n_usize: usize = n.try_into().unwrap();
        let squares: Vec<usize> = (1..).map(|i| i * i).take_while(|&s| s <= n_usize).collect();
        let mut dp: Vec<i32> = vec![n + 1; n_usize + 1_usize];

        dp[0] = 0;
        for i in 1..=n_usize {
            for &s in squares.iter().take_while(|&s| s <= &i) {
                dp[i] = min(dp[i], dp[i - s] + 1)
            }
        }

        dp[n_usize]
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
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::num_squares(13));
    }
}
