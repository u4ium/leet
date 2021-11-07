struct Solution {}

use std::convert::TryInto;

#[derive(Copy, Clone, Debug)]
struct BitSet {
    data: u32,
    length: u8,
}

impl BitSet {
    #[inline]
    pub fn new() -> Self {
        BitSet { data: 0, length: 0 }
    }

    #[inline]
    pub fn insert(&mut self, value: u8) -> bool {
        let result = self.data & (1 << value) == 0;
        if result {
            self.data |= 1 << value;
            self.length += 1;
        }
        result
    }
}

struct Coordinate<'a> {
    x: usize,
    y: usize,
    grid: &'a Grid,
}

impl<'a> Coordinate<'a> {
    #[inline]
    pub fn new([x, y]: [usize; 2], grid: &'a Grid) -> Self {
        Coordinate { x, y, grid }
    }

    #[inline]
    pub fn to_value(&self) -> u8 {
        (self.x * self.grid[0].len() + self.y)
            .try_into()
            .expect("Grid too large")
    }

    #[inline]
    pub fn up(&self) -> Option<Self> {
        if self.x > 0 {
            Some(Coordinate {
                x: self.x - 1,
                ..*self
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn right(&self) -> Option<Self> {
        if self.y < self.grid[0].len() - 1 {
            Some(Coordinate {
                y: self.y + 1,
                ..*self
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn down(&self) -> Option<Self> {
        if self.x < self.grid.len() - 1 {
            Some(Coordinate {
                x: self.x + 1,
                ..*self
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn left(&self) -> Option<Self> {
        if self.y > 0 {
            Some(Coordinate {
                y: self.y - 1,
                ..*self
            })
        } else {
            None
        }
    }
}

type Grid = Vec<Vec<i32>>;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        #[inline]
        fn find_start_and_count_destinations(grid: &Grid) -> (Option<Coordinate>, u8) {
            let mut result = (None, 0);
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    match grid[i][j] {
                        1 => {
                            result.0 = Some(Coordinate::new([i, j], grid));
                            result.1 += 1;
                        }
                        0 | 2 => result.1 += 1,
                        -1 => (),
                        _ => unreachable!(),
                    }
                }
            }
            result
        }

        fn count_paths(
            grid: &Grid,
            current: Coordinate,
            mut used: BitSet,
            num_destinations: u8,
        ) -> i32 {
            if !used.insert(current.to_value()) {
                return 0;
            }
            match grid[current.x][current.y] {
                2 if used.length == num_destinations => 1,
                0 | 1 => vec![
                    current.up(),
                    current.right(),
                    current.down(),
                    current.left(),
                ]
                .into_iter()
                .flatten()
                .map(|next| count_paths(grid, next, used, num_destinations))
                .sum(),
                -1 | 2 => 0,
                _ => unreachable!(),
            }
        }

        let (start, num_destinations) = find_start_and_count_destinations(&grid);
        count_paths(
            &grid,
            start.expect("Start not found on grid"),
            BitSet::new(),
            num_destinations,
        )
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
        let input = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
        let output = 2;
        assert_eq!(output, Solution::unique_paths_iii(input));
        // Explanation: We have the following two paths:
        // 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
        // 2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
    }
    #[test]
    fn example_2() {
        let input = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]];
        let output = 4;
        assert_eq!(output, Solution::unique_paths_iii(input));
        // Explanation: We have the following four paths:
        // 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
        // 2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
        // 3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
        // 4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
    }

    #[test]
    fn example_3() {
        let input = vec![vec![0, 1], vec![2, 0]];
        let output = 0;
        assert_eq!(output, Solution::unique_paths_iii(input));
        // Explanation: There is no path that walks over every empty square exactly once.
        // Note that the starting and ending square can be anywhere in the grid.
    }
}
