use std::cmp::{max, min};
use std::convert::TryInto;

struct Solution {}

impl Solution {
    /// Given an integer array nums,
    ///  - that you start initially positioned at the array's first index,
    ///  - and that each element in the array represents your maximum jump length at that position:
    /// return true if you can reach the last index, or false otherwise.
    ///
    /// REQ:
    ///     - 1 <= nums.length <= 104
    ///     - 0 <= nums[i] <= 105
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let length = nums.len();
        let mut last_reachable = 0;
        for (num_index, &num) in nums.iter().enumerate() {
            if num_index > last_reachable {
                break;
            }
            let jump_size: usize = num.try_into().unwrap();
            last_reachable = max(last_reachable, min(num_index + jump_size, length - 1));
        }
        last_reachable == length - 1
    }
}

#[cfg(test)]
mod solution_tests {
    use super::*;
    #[test]
    fn test_example_1() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::can_jump(vec![1]), true);
        assert_eq!(Solution::can_jump(vec![0]), true);
    }

    #[test]
    fn test_double_failure() {
        assert_eq!(Solution::can_jump(vec![0, 1]), false);
    }

    #[test]
    fn test_double_success() {
        assert_eq!(Solution::can_jump(vec![1, 0]), true);
        assert_eq!(Solution::can_jump(vec![2, 0]), true);
        assert_eq!(Solution::can_jump(vec![3, 3]), true);
    }

    #[test]
    #[should_panic]
    fn test_panics_with_empty_list() {
        Solution::can_jump(vec![]);
    }

    #[test]
    #[should_panic]
    fn test_panics_with_negative_number() {
        Solution::can_jump(vec![-1]);
    }

    #[test]
    #[should_panic]
    fn test_panics_with_some_negative_number() {
        Solution::can_jump(vec![1, 2, 0, 0, 0, -1, 6]);
    }
}
