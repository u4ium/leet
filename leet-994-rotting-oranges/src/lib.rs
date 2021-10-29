struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        -1
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
        let input = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(4, Solution::oranges_rotting(input));
    }

    #[test]
    fn example_2() {
        let input = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(-1, Solution::oranges_rotting(input));
    }

    #[test]
    fn example_3() {
        let input = vec![vec![0, 2]];
        assert_eq!(0, Solution::oranges_rotting(input));
    }
}
