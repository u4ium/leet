struct Solution {}

use std::cmp::max;

impl Solution {
    /// Return the maximum amount of money you can rob tonight without alerting the police,
    /// given an integer array nums representing the amount of money of each house.
    ///
    /// You are a professional robber planning to rob houses along a street.
    /// Each house has a certain amount of money stashed,
    /// the only constraint stopping you from robbing each of them is that adjacent houses
    /// have security systems connected and it will automatically contact the police
    /// if two adjacent houses were broken into on the same night.
    ///
    /// REQ:
    ///     -> 1 <= nums.length <= 100
    ///     -> 0 <= nums[i] <= 400
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp: [i32; 2] = [nums[0], max(nums[0], nums[1])];
        for i in 2..nums.len() {
            dp[i % 2] = max(dp[(i % 2) ^ 1], dp[i % 2] + nums[i])
        }

        max(dp[0], dp[1])
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
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        // Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
        // Total amount you can rob = 1 + 3 = 4.
    }

    #[test]
    fn example_2() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
        // Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
        // Total amount you can rob = 2 + 9 + 1 = 12.
    }
}
