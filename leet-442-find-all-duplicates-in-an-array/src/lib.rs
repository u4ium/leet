use std::collections::HashSet;

struct Solution {}

impl Solution {
    /// Find all the duplicate elements in the given array
    ///
    /// REQ:
    ///     - n == nums.length
    ///     - 1 <= n <= 10^5
    ///     - 1 <= nums[i] <= n
    ///     - Each element in nums appears once or twice
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut duplicates = vec![];
        let mut seen = HashSet::new();
        for num in nums.iter() {
            if seen.contains(num) {
                duplicates.push(*num);
            } else {
                seen.insert(num);
            }
        }
        duplicates.sort();
        duplicates
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
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::find_duplicates(vec![1]), vec![]);
    }
}
