struct Solution {}

use std::cmp::{max, min};
use std::convert::TryInto;

impl Solution {
    /// Return the minimum number of jumps needed to get to the last index of nums.
    ///
    /// Given an array of non-negative integers nums, you are initially positioned
    /// at the first index of the array:
    ///  - Each element in the array represents your maximum jump length at that position.
    ///  - Your goal is to reach the last index in the minimum number of jumps.
    ///  - You can assume that you can always reach the last index.
    ///
    /// REQ:
    ///     -> 1 <= nums.length <= 104
    ///     -> 0 <= nums[i] <= 1000
    pub fn jump(nums: Vec<i32>) -> i32 {
        const INACCESSIBLE: Option<i32> = None;
        let length = nums.len();
        let mut dp = vec![INACCESSIBLE; nums.len()];
        dp[0] = Some(0);
        for (index, &num) in nums.iter().enumerate() {
            // TODO
            let jump: usize = num.try_into().unwrap();
            let max_position = min(length - 1, index + jump);
            let current_best = dp[index].unwrap();
            for next_position in index..=max_position {
                match dp[next_position] {
                    Some(best) if best > current_best => {
                        dp[next_position] = Some(current_best + 1);
                    }
                    None => {
                        dp[next_position] = Some(current_best + 1);
                    }
                    _ => (),
                }
            }
        }
        dp[length - 1].unwrap()
    }
}

#[cfg(test)]
mod solution_tests {
    use super::*;
    #[test]
    fn test_example_1() {
        // Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_example_2() {
        // Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    }

    #[test]
    fn test_single() {
        // Explanation: any array with a single element is trivially jumped through.
        assert_eq!(0, Solution::jump(vec![1]));
        assert_eq!(0, Solution::jump(vec![0]));
    }
}
