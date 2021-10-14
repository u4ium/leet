struct Solution {}

use std::cmp::{max, min};

impl Solution {
    /// Return the knight's minimum initial health so that he can rescue the princes, where:
    ///
    /// - The knight has an initial health point represented by a positive integer.
    /// - If at any point his health point drops to 0 or below, he dies immediately.
    /// - The knight loses or gains health in each dungeon room according to the number.
    /// - The knight starts at (0, 0) and must reach the other corner
    /// - The knight may only move rightward and downward
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let num_rows = dungeon.len();
        let num_columns = dungeon[0].len();

        let mut dp = vec![vec![0; num_columns]; num_rows];
        let m = num_rows - 1;
        let n = num_columns - 1;
        dp[m][n] = max(0, -dungeon[m][n]);

        for c in (0..n).rev() {
            dp[m][c] = max(0, dp[m][c + 1] - dungeon[m][c]);
        }
        for r in (0..m).rev() {
            dp[r][n] = max(0, dp[r + 1][n] - dungeon[r][n]);
        }

        for r in (0..m).rev() {
            for c in (0..n).rev() {
                dp[r][c] = max(0, min(dp[r + 1][c], dp[r][c + 1]) - dungeon[r][c]);
            }
        }

        1 + dp[0][0]
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
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ]),
            7
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![0]]), 1);
    }
}
