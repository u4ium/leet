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

pub enum RottingState {
    Rotting,
    Stuck,
    Done,
}

pub enum RotResult {
    Rot(RottingState),
    Complete,
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

struct Coordinate {
    r: usize,
    c: usize,
}

impl Grid {
    /// Convert fresh Oranges that are immediately adjacent to rotten ones to rotten.
    ///
    /// Return true iff at least one orange is converted.
    pub fn rot(&mut self) -> RotResult {
        let mut num_fresh = 0;
        let mut rotten = vec![];
        for (r, row) in self.0.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if let Orange(freshness) = cell {
                    match freshness {
                        Rotten => rotten.push(Coordinate { r, c }),
                        Fresh => num_fresh += 1,
                    }
                }
            }
        }

        if num_fresh == 0 {
            return RotResult::Complete;
        }

        let mut num_fresh_rotted = 0;
        for coordinate in rotten {
            for next_coordinate in self.get_next_cells(coordinate) {
                let next_orange = &mut self.0[next_coordinate.r][next_coordinate.c];
                if let Orange(Fresh) = next_orange {
                    *next_orange = Orange(Rotten);
                    num_fresh_rotted += 1;
                }
            }
        }

        RotResult::Rot(match num_fresh_rotted {
            0 => RottingState::Stuck,
            x if x == num_fresh => RottingState::Done,
            _ => RottingState::Rotting,
        })
    }

    #[inline]
    fn get_next_cells(&self, current: Coordinate) -> Vec<Coordinate> {
        let mut result = vec![];
        if current.r > 0 {
            result.push(Coordinate {
                r: current.r - 1,
                ..current
            });
        }
        if current.r < self.0.len() - 1 {
            result.push(Coordinate {
                r: current.r + 1,
                ..current
            });
        }
        if current.c > 0 {
            result.push(Coordinate {
                c: current.c - 1,
                ..current
            });
        }
        if current.c < self.0[0].len() - 1 {
            result.push(Coordinate {
                c: current.c + 1,
                ..current
            });
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
