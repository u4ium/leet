struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn itw_works() {
        assert_eq!(2 + 2, 4);
    }

    fn check_solution(input: Vec<i32>, output: Vec<Vec<i32>>) {
        let mut test_output = Solution::three_sum(input);
        assert_eq!(test_output.len(), output.len());
        let mut output = output;
        for index in 0..output.len() {
            output[index].sort();
            test_output[index].sort();
        }
        output.sort();
        test_output.sort();
        assert_eq!(output, test_output);
    }

    #[test]
    fn example_1() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(output, Solution::three_sum(input));
    }

    #[test]
    fn example_2() {
        let input = vec![];
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(output, Solution::three_sum(input));
    }

    #[test]
    fn example_3() {
        let input = vec![0];
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(output, Solution::three_sum(input));
    }
}
