struct Solution {}

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
        let max_row_index = grid.len() - 1;
        let max_column_index = grid[0].len() - 1;
        let mut result = 0;
        for row_index in 0..=max_row_index {
            for column_index in 0..=max_column_index {
                result += match grid[row_index][column_index] {
                    1 => {
                        4 - (if row_index > 0 {
                            grid[row_index - 1][column_index]
                        } else {
                            0
                        } + if row_index < max_row_index {
                            grid[row_index + 1][column_index]
                        } else {
                            0
                        } + if column_index > 0 {
                            grid[row_index][column_index - 1]
                        } else {
                            0
                        } + if column_index < max_column_index {
                            grid[row_index][column_index + 1]
                        } else {
                            0
                        })
                    }
                    _ => 0,
                };
            }
        }
        result
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
