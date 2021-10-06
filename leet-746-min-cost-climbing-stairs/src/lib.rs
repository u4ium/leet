struct Solution {}

use std::cmp::min;

impl Solution {
    /// Return the minimum cost to reach the top of the floor, where:
    ///     - given an integer array cost where cost[i] is the cost of ith step on a staircase
    ///     - once you pay the cost, you can either climb one or two steps
    ///     - start from the step with index 0, or the step with index 1
    ///
    /// REQ:
    ///     - 2 <= cost.length <= 1000
    //      - 0 <= cost[i] <= 999
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = [cost[0], cost[1]];
        for index in 2..cost.len() {
            dp[(index % 2) as usize] = min(dp[0], dp[1]) + cost[index];
        }
        min(dp[0], dp[1])
        //let mut dp = vec![0; cost.len()];
        //dp[0] = cost[0];
        //dp[1] = cost[1];
        //for index in 2..cost.len() {
        //    dp[index] = min(dp[index - 2], dp[index - 1]) + cost[index];
        //}
        //min(dp[cost.len() - 2], dp[cost.len() - 1])
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
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
