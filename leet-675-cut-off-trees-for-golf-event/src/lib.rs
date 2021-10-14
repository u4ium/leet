struct Solution {}

type Forest = Vec<Vec<i32>>;

enum Direction {
    East,
    North,
    West,
    South,
}
use Direction::*;

type Coordinates = [usize; 2];

fn get_moves(current: Coordinates, max: Coordinates) -> Vec<Coordinates> {
    const DIRECTIONS: [Direction; 4] = [East, North, West, South];

    DIRECTIONS
        .iter()
        .filter_map(|d| match d {
            East if current[1] < max[1] - 1 => Some([current[0], current[1] + 1]),
            North if current[0] > 0 => Some([current[0] - 1, current[1]]),
            West if current[1] > 0 => Some([current[0], current[1] - 1]),
            South if current[0] < max[0] - 1 => Some([current[0] + 1, current[1]]),
            _ => None,
        })
        .collect()
}

fn find_minimum_tree(forest: &Forest) -> Option<Coordinates> {
    let mut max_tree = 1;
    let mut best = None;
    for (r, row) in forest.iter().enumerate() {
        for (c, &element) in row.iter().enumerate() {
            if element > max_tree {
                max_tree = element;
                best = Some([r, c]);
            }
        }
    }
    best
}

fn find_all_shortest_paths(forest: &Forest) -> Vec<Vec<Vec<Vec<Option<i32>>>>> {
    let [m, n] = [forest.len(), forest[0].len()];
    let v = m * n;
    let mut distance = vec![vec![vec![vec![None; n]; m]; n]; m];

    // for every edge, if forest[start] and forest[end] != 0, distance[start][end] = Some(1), else None

    // for every vertex: v, distance[v][v] = Some(0)

    // for k in 1..v
    //     for i in 1..v
    //         for j in 1..v
    //             if distance[i][k] + distance[k][j] < distance[i][j] {
    //                 distance[i][j] ← distance[i][k] + distance[k][j]
    //             }

    // Floyd-Warshall
    distance
}

impl Solution {
    /// You are asked to cut off all the trees in a forest for a golf event.
    ///
    /// The forest is represented as an m x n matrix. In this matrix:
    ///     - 0 means the cell cannot be walked through.
    ///     - 1 represents an empty cell that can be walked through.
    ///     - A number greater than 1 represents a tree in a cell that can be walked through, and this number is the tree's height.
    ///
    /// In one step, you can walk in any of the four directions: north, east, south, and west.
    /// If you are standing in a cell with a tree, you can choose whether to cut it off.
    ///
    /// You must cut off the trees in order from shortest to tallest.
    /// When you cut off a tree, the value at its cell becomes 1 (an empty cell).
    ///
    /// Starting from the point (0, 0), return the minimum steps you need to walk to cut off all the trees.
    /// If you cannot cut off all the trees, return -1.
    ///
    /// You are guaranteed that no two trees have the same height, and there is at least one tree needs to be cut off.
    ///
    /// REQ:
    ///     -> m == forest.length
    ///     -> n == forest[i].length
    ///     -> 1 <= m, n <= 50
    ///     -> 0 <= forest[i][j] <= 10^9
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let max = [forest.len() - 1, forest[0].len() - 1];
        let mut forest = forest;
        let mut path_length = 0;
        let shortest_paths = find_all_shortest_paths(&forest); // O((m*n)^3)
        let [mut x, mut y] = find_minimum_tree(&forest).unwrap(); // O((m*n)^2)

        // O((m*n)^3)
        while let Some([nx, ny]) = find_minimum_tree(&forest) {
            if let Some(shortest_path) = shortest_paths[x][y][nx][ny] {
                forest[x][y] = 1;
                path_length += shortest_path;
                x = nx;
                y = ny;
            } else {
                return -1;
            }
        }
        path_length
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
        let forest = vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]];
        assert_eq!(6, Solution::cut_off_tree(forest));
        // Explanation: Following the path above allows you to cut off the trees from shortest to tallest in 6 steps.
    }

    #[test]
    fn example_2() {
        let forest = vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5]];
        assert_eq!(-1, Solution::cut_off_tree(forest));
        // Explanation: The trees in the bottom row cannot be accessed as the middle row is blocked.
    }

    #[test]
    fn example_3() {
        let forest = vec![vec![2, 3, 4], vec![0, 0, 5], vec![8, 7, 6]];
        assert_eq!(6, Solution::cut_off_tree(forest));
        // Explanation: You can follow the same path as Example 1 to cut off all the trees.
        // Note that you can cut off the first tree at (0, 0) before making any steps.
    }
}
