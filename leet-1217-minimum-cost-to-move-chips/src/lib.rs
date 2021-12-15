pub struct Solution;

impl Solution {
    /// Return the minimum cost needed to move all the chips to the same position.
    ///
    /// We have n chips, where the position of the ith chip is position[i].
    /// We need to move all the chips to the same position.
    /// In one step, we can change the position of the ith chip from position[i] to:
    ///  - position[i] + 2 or position[i] - 2 with cost = 0.
    ///  - position[i] + 1 or position[i] - 1 with cost = 1.
    ///
    /// ## Constraints
    ///  - 1 <= position.length <= 100
    ///  - 1 <= position[i] <= 10⁹
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut num_odd = 0;
        let mut num_even = 0;
        for chip in position {
            match chip % 2 {
                0 => num_even += 1,
                1 => num_odd += 1,
                _ => unreachable!(),
            }
        }
        std::cmp::min(num_even, num_odd)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 2, 3]));
    }
    // Explanation: First step: Move the chip at position 3 to position 1 with cost = 0.
    // Second step: Move the chip at position 2 to position 1 with cost = 1.
    // Total cost is 1.

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]));
    }
    // Explanation: We can move the two chips at position  3 to position 2. Each move has cost = 1. The total cost = 2.

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 1000000000]));
    }

    #[test]
    fn mine() {
        assert_eq!(
            2,
            Solution::min_cost_to_move_chips(vec![1, 2, 4, 5, 5, 5, 7])
        );
    }
}
