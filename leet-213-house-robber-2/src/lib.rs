struct Solution {}

use std::cmp::max;

impl Solution {
    /// Return the maximum amount of money you can rob tonight without alerting the police,
    /// given an integer array nums representing the amount of money of each house.
    ///
    /// You are a professional robber planning to rob houses along a street.
    /// Each house has a certain amount of money stashed.
    ///
    /// All houses at this place are arranged in a circle.
    /// That means the first house is the neighbor of the last one.
    ///
    /// Meanwhile, adjacent houses have a security system connected,
    /// and it will automatically contact the police
    /// if two adjacent houses were broken into on the same night.
    ///
    /// REQ:
    ///     -> 1 <= nums.length <= 100
    ///     -> 0 <= nums[i] <= 1000
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return max(nums[0], nums[1]);
        }

        let mut dp_first: [i32; 2] = [nums[0], max(nums[0], nums[1])];
        let mut dp_last: [i32; 2] = [0, nums[1]];
        for i in 2..nums.len() - 1 {
            dp_first[i % 2] = max(dp_first[(i % 2) ^ 1], dp_first[i % 2] + nums[i]);
            dp_last[i % 2] = max(dp_last[(i % 2) ^ 1], dp_last[i % 2] + nums[i]);
        }

        let max_dp_first = max(dp_first[0], dp_first[1]);
        let i = nums.len() - 1;
        let last = i % 2;
        let other = 1 ^ last;
        let max_dp_last = max(dp_last[other], dp_last[last] + nums[i]);
        max(max_dp_first, max_dp_last)
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
        assert_eq!(3, Solution::rob(vec![2, 3, 2]));
        // You cannot rob house 1 (money = 2) and then rob house 3 (money = 2),
        // because they are adjacent houses.
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        // Rob house 1 (money = 1) and then rob house 3 (money = 3).
        // Total amount you can rob = 1 + 3 = 4.
    }

    #[test]
    fn example_3() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        // Rob house 1 (money = 1) and then rob house 3 (money = 3).
        // Total amount you can rob = 1 + 3 = 4.
    }
}
