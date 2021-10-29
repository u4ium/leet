struct Solution {}

mod grid;
use grid::*;

impl Solution {
    /// Return the minimum number of minutes that must elapse until no cell has a fresh orange.
    /// If this is impossible, return -1.
    ///
    /// Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
    /// You are given an m x n grid where each cell can have one of three values:
    ///     - 0 representing an empty cell,
    ///     - 1 representing a fresh orange, or
    ///     - 2 representing a rotten orange.
    ///
    /// REQ:
    ///     - m == grid.length
    ///     - n == grid[i].length
    ///     - 1 <= m, n <= 10
    ///     - grid[i][j] is 0, 1, or 2.
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut oranges = Grid::from(grid);
        let mut minutes = 0;
        while let RotResult::Rot(rotting_state) = oranges.rot() {
            match rotting_state {
                RottingState::Stuck => return -1,
                RottingState::Done => return minutes + 1,
                RottingState::Rotting => minutes += 1,
            }
        }
        minutes
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
