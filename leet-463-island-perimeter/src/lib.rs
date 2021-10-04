struct Solution {}

type Grid = Vec<Vec<i32>>;

struct Island {
    grid: Grid,
}

impl IntoIterator for Island {
    type Item = IslandCell;
    type IntoIter = IslandIterator;
    fn into_iter(self) -> IslandIterator {
        IslandIterator {
            grid: self.grid,
            row_index: 0,
            column_index: 0,
        }
    }
}

struct IslandIterator {
    grid: Grid,
    row_index: usize,
    column_index: usize,
}

struct IslandCell {
    coastline_length: i32,
}

impl IslandCell {
    fn new(grid: &Grid, row_index: usize, column_index: usize) -> Self {
        let max_row_index = grid.len() - 1;
        let max_column_index = grid[0].len() - 1;
        let mut coastline_length = 4;
        if row_index > 0 {
            coastline_length -= grid[row_index - 1][column_index];
        }
        if row_index < max_row_index {
            coastline_length -= grid[row_index + 1][column_index];
        }
        if column_index > 0 {
            coastline_length -= grid[row_index][column_index - 1];
        }
        if column_index < max_column_index {
            coastline_length -= grid[row_index][column_index + 1];
        }
        IslandCell { coastline_length }
    }

    fn get_coastline_length(&self) -> i32 {
        self.coastline_length
    }
}

impl Iterator for IslandIterator {
    type Item = IslandCell;

    fn next(&mut self) -> Option<Self::Item> {
        let max_row_index = self.grid.len() - 1;
        let max_column_index = self.grid[0].len() - 1;
        loop {
            if self.column_index > max_column_index {
                if self.row_index == max_row_index {
                    return None;
                }
                self.row_index += 1;
                self.column_index = 0;
            }
            if self.grid[self.row_index][self.column_index] == 1 {
                break;
            }
            self.column_index += 1;
        }
        let ret = IslandCell::new(&self.grid, self.row_index, self.column_index);
        self.column_index += 1;
        Some(ret)
    }
}

/// Determine the perimeter of the island represented by the given row x col grid where:
///     - grid[i][j] = 1 represents land and
///     - grid[i][j] = 0 represents water.
///     - Grid cells are connected horizontally/vertically (not diagonally).
///     - The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
///     - The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island.
///     - One cell is a square with side length 1.
///     - The grid is rectangular, width and height don't exceed 100.
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        Island { grid }
            .into_iter()
            .map(|island| island.get_coastline_length())
            .sum()
    }
}

#[cfg(test)]
mod solution_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0],
            ]),
            16
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
    }
}
