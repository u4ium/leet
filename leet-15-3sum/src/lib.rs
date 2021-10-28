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
