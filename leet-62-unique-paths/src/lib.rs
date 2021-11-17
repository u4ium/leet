struct Solution {}

use std::cmp::min;
use std::convert::TryInto;

#[inline]
pub fn choose(mut n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }

    (1..=k).fold(1, |mut r, d| {
        r *= n;
        n -= 1;
        r / d
    })
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let vertical_steps = (m - 1).try_into().unwrap();
        let horizontal_steps = (n - 1).try_into().unwrap();
        let total_steps = vertical_steps + horizontal_steps;
        let shorter_steps = min(vertical_steps, horizontal_steps);
        choose(total_steps, shorter_steps).try_into().unwrap()
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
        assert_eq!(28, Solution::unique_paths(3, 7))
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::unique_paths(3, 2))
        // Explanation:
        // From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
        // 1. Right -> Down -> Down
        // 2. Down -> Down -> Right
        // 3. Down -> Right -> Down
    }

    #[test]
    fn example_3() {
        assert_eq!(28, Solution::unique_paths(7, 3))
    }

    #[test]
    fn example_4() {
        assert_eq!(6, Solution::unique_paths(3, 3))
    }

    #[test]
    fn extra_big() {
        assert_eq!(4496388, Solution::unique_paths(36, 7))
    }

    #[test]
    fn test_choose_poker() {
        assert_eq!(2_598_960, choose(52, 5));
    }

    #[test]
    fn extra_2() {
        assert_eq!(1_916_797_311, Solution::unique_paths(51, 9));
    }

    #[test]
    fn extra_3() {
        assert_eq!(1, Solution::unique_paths(1, 100));
    }
}
