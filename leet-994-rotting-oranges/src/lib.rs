struct Solution {}

pub enum Freshness {
    Rotten,
    Fresh,
}
use Freshness::*;

pub enum Cell {
    Empty,
    Orange(Freshness),
}
use Cell::*;

impl From<i32> for Cell {
    fn from(n: i32) -> Self {
        match n {
            0 => Empty,
            1 => Orange(Fresh),
            2 => Orange(Rotten),
            _ => panic!("Unknown value for cell: {}", n),
        }
    }
}

pub struct Grid(Vec<Vec<Cell>>);

impl From<Vec<Vec<i32>>> for Grid {
    fn from(grid: Vec<Vec<i32>>) -> Self {
        Grid(
            grid.iter()
                .map(|row| row.iter().map(|&c| Cell::from(c)).collect())
                .collect(),
        )
    }
}

impl Grid {
    /// Return true iff at least one orange in the grid is still fresh.
    pub fn has_fresh(&self) -> bool {
        for row in self.0.iter() {
            for cell in row.iter() {
                if let Orange(Fresh) = cell {
                    return true;
                }
            }
        }
        false
    }

    /// Convert fresh Oranges that are immediately adjacent to rotten ones to rotten.
    ///
    /// Return true iff at least one orange is converted.
    pub fn rot(&mut self) -> bool {
        let mut rotten = vec![];
        for (r, row) in self.0.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if let Orange(Rotten) = cell {
                    rotten.push([r, c]);
                }
            }
        }

        let mut found_fresh_adjacent = false;
        for [r, c] in rotten {
            for [n_r, n_c] in self.get_next_cells([r, c]) {
                let next_orange = &mut self.0[n_r][n_c];
                if let Orange(Fresh) = next_orange {
                    *next_orange = Orange(Rotten);
                    found_fresh_adjacent = true;
                }
            }
        }

        found_fresh_adjacent
    }

    #[inline]
    fn get_next_cells(&self, current: [usize; 2]) -> Vec<[usize; 2]> {
        let mut result = vec![];
        if current[0] > 0 {
            result.push([current[0] - 1, current[1]]);
        }
        if current[0] < self.0.len() - 1 {
            result.push([current[0] + 1, current[1]]);
        }
        if current[1] > 0 {
            result.push([current[0], current[1] - 1]);
        }
        if current[1] < self.0[0].len() - 1 {
            result.push([current[0], current[1] + 1]);
        }
        result
    }
}

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
        let mut grid = Grid::from(grid);
        let mut minutes = 0;
        while grid.has_fresh() {
            if !grid.rot() {
                return -1;
            }
            minutes += 1;
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
