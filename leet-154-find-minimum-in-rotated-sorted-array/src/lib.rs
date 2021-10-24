struct Solution {}

use std::cmp::Ordering::*;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let middle = (left + right) / 2;
            match nums[middle].cmp(&nums[right]) {
                Less => right = middle,
                Equal => right -= 1,
                Greater => left = middle + 1,
            }
        }
        nums[right]
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
    fn example_0() {
        assert_eq!(0, Solution::find_min(vec![4, 5, 6, 7, 0, 1, 4]));
    }

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::find_min(vec![1, 3, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::find_min(vec![2, 2, 2, 0, 1]));
    }

    #[test]
    fn extra() {
        assert_eq!(1, Solution::find_min(vec![10, 1, 10, 10, 10]));
    }
}
