use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    /// Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice.
    /// Find the two elements that appear only once. You can return the answer in any order.
    ///
    /// You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.
    ///
    /// REQ:
    ///     - 2 <= nums.length <= 3 * 10^4
    //      - -2^31 <= nums[i] <= 2^31 - 1
    //      - Each integer in nums will appear twice, only two integers will appear once.
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums.iter() {
            if set.contains(num) {
                set.remove(num);
            } else {
                set.insert(*num);
            }
        }
        set.drain().collect()
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
        let input = vec![1, 2, 1, 3, 2, 5];
        let expect: HashSet<i32> = HashSet::from_iter(vec![3, 5]);
        let actual: HashSet<i32> = Solution::single_number(input).into_iter().collect();
        assert_eq!(expect, actual);
    }
    #[test]
    fn example_2() {
        let input = vec![-1, 0];
        let expect: HashSet<i32> = HashSet::from_iter(vec![-1, 0]);
        let actual: HashSet<i32> = Solution::single_number(input).into_iter().collect();
        assert_eq!(expect, actual);
    }
    #[test]
    fn example_3() {
        let input = vec![0, 1];
        let expect: HashSet<i32> = HashSet::from_iter(vec![1, 0]);
        let actual: HashSet<i32> = Solution::single_number(input).into_iter().collect();
        assert_eq!(expect, actual);
    }
}
