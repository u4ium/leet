struct Solution {}

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    /// Return all unique (up to order) triplets of numbers from nums which add to 0.
    ///
    /// REQ:
    ///     - 0 <= nums.length <= 3000
    ///     - 10^5 <= nums[i] <= 10^5
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut two_sums: HashMap<i32, Vec<[usize; 2]>> = HashMap::new();
        for i in 0..n {
            for j in i + 1..n {
                let remainder = nums[i] + nums[j];
                two_sums.entry(-remainder).or_default().push([i, j]);
            }
        }

        let mut results: HashSet<Vec<i32>> = HashSet::new();
        for (k, n) in nums.iter().enumerate() {
            if let Some(pairs) = two_sums.get(n) {
                for [i, j] in pairs {
                    if k != *i && k != *j {
                        let mut result = vec![*n, nums[*i], nums[*j]];
                        result.sort();
                        results.insert(result);
                    }
                }
            }
        }

        results.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn check_solution(input: Vec<i32>, output: Vec<Vec<i32>>) {
        let mut output = output;
        output.sort();
        for triplet in output.iter_mut() {
            triplet.sort();
        }

        let mut test_output = Solution::three_sum(input);
        test_output.sort();
        for triplet in test_output.iter_mut() {
            triplet.sort();
        }

        assert_eq!(output, test_output);
    }

    #[test]
    fn example_1() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        check_solution(input, output);
    }

    #[test]
    fn example_2() {
        let input = vec![];
        let output: Vec<Vec<i32>> = vec![];
        check_solution(input, output);
    }

    #[test]
    fn example_3() {
        let input = vec![0];
        let output: Vec<Vec<i32>> = vec![];
        check_solution(input, output);
    }

    #[test]
    fn example_4() {
        let input = vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4];
        let output = vec![
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-3, -1, 4],
            vec![-3, 0, 3],
            vec![-3, 1, 2],
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-1, -1, 2],
            vec![-1, 0, 1],
        ];
        check_solution(input, output);
    }
}
