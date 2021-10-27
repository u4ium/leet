struct Solution {}

use std::convert::TryInto;
use std::iter::repeat;

impl Solution {
    /// Given an array nums with n objects colored red, white, or blue,
    /// sort them in-place so that objects of the same color are adjacent,
    /// with the colors in the order red, white, and blue.
    ///
    /// We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
    ///
    /// You must solve this problem without using the library's sort function.
    ///
    /// REQ:
    ///     - n == nums.length
    ///     - 1 <= n <= 300
    ///     - nums[i] is 0, 1, or 2.
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // bucket sort
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
        let mut input = vec![2, 0, 2, 1, 1, 0];
        let output = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut input);
        assert_eq!(output, input);
    }

    #[test]
    fn example_2() {
        let mut input = vec![2, 0, 1];
        let output = vec![0, 1, 2];
        Solution::sort_colors(&mut input);
        assert_eq!(output, input);
    }

    #[test]
    fn example_3() {
        let mut input = vec![0];
        let output = vec![0];
        Solution::sort_colors(&mut input);
        assert_eq!(output, input);
    }

    #[test]
    fn example_4() {
        let mut input = vec![1];
        let output = vec![1];
        Solution::sort_colors(&mut input);
        assert_eq!(output, input);
    }
}
