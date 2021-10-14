struct Solution {}

use std::cmp::max;

impl Solution {
    /// Return the maximum number of points you can earn by applying the given some number of times:
    ///     - Pick any nums[i] and delete it to earn nums[i] points.
    ///     - Afterwards, you must delete every element equal to nums[i] - 1
    ///         and every element equal to nums[i] + 1.
    ///
    /// REQ:
    ///     - 1 <= nums.length <= 2 * 10^4
    ///     - 1 <= nums[i] <= 10^4
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut nums = nums;
        nums.sort();

        let mut neighbours = vec![nums[0]];
        let mut prev = nums[0];
        let mut i = 0;

        for &num in &nums[1..] {
            if num == prev {
                neighbours[i] += num;
            } else {
                if num != prev + 1 {
                    neighbours.push(0);
                    i += 1;
                }
                neighbours.push(num);
                i += 1;
                prev = num;
            }
        }

        let mut dp: [i32; 2] = [neighbours[0], max(neighbours[0], neighbours[1])];
        for i in 2..neighbours.len() {
            dp[i % 2] = max(dp[(i % 2) ^ 1], dp[i % 2] + neighbours[i])
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
        // Explanation: You can perform the following operations:
        //  - Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
        //  - Delete 2 to earn 2 points. nums = [].
        // You earn a total of 6 points.
        assert_eq!(6, Solution::delete_and_earn(vec![3, 4, 2]));
    }

    #[test]
    fn example_2() {
        // You can perform the following operations:
        //  - Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
        //  - Delete a 3 again to earn 3 points. nums = [3].
        //  - Delete a 3 once more to earn 3 points. nums = [].
        // You earn a total of 9 points.
        assert_eq!(9, Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]));
    }

    #[test]
    fn extra() {
        assert_eq!(4, Solution::delete_and_earn(vec![3, 1]));
    }
}
