struct Solution {}

impl Solution {
    /// Given an m x n grid of characters board and a string word,
    /// return true if word exists in the grid.
    ///
    /// The word can be constructed from letters of sequentially adjacent cells,
    /// where adjacent cells are horizontally or vertically neighboring.
    ///
    /// The same letter cell may not be used more than once.
    ///
    /// REQ:
    ///     - m == board.length
    ///     - n = board[i].length
    ///     - 1 <= m, n <= 6
    ///     - 1 <= word.length <= 15
    ///     - board and word consists of only lowercase and uppercase English letters.
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for row in 0..board.len() {
            for column in 0..board[0].len() {
                let mut used = vec![vec![false; board[0].len()]; board.len()];
                if matches(&board, &mut used, &word[..], row, column) {
                    return true;
                }
            }
        }
        false
    }
}

type Board = Vec<Vec<char>>;
type Usage = Vec<Vec<bool>>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

fn matches(board: &Board, used: &mut Usage, word: &str, row: usize, column: usize) -> bool {
    if used[row][column] {
        return false;
    }
    let (first, rest) = word.split_at(1);
    if let Some(first) = first.chars().nth(0) {
        if first == board[row][column] {
            used[row][column] = true;
            if rest == "" {
                return true;
            }
            for direction in DIRECTIONS.iter() {
                let next_coordinates = match direction {
                    Up if row > 0 => Some([row - 1, column]),
                    Down if row < board.len() - 1 => Some([row + 1, column]),
                    Left if column > 0 => Some([row, column - 1]),
                    Right if column < board[0].len() - 1 => Some([row, column + 1]),
                    _ => None,
                };
                if let Some([next_row, next_column]) = next_coordinates {
                    if matches(board, used, rest, next_row, next_column) {
                        return true;
                    }
                }
            }
            used[row][column] = false;
            false
        } else {
            used[row][column] = false;
            false
        }
    } else {
        panic!("word should never be the empty string")
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
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCCED");
        assert_eq!(true, Solution::exist(board, word));
    }

    #[test]
    fn example_2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("SEE");
        assert_eq!(true, Solution::exist(board, word));
    }

    #[test]
    fn example_3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = String::from("ABCB");
        assert_eq!(false, Solution::exist(board, word));
    }

    #[test]
    fn extra() {
        let board = vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'a', 'a', 'a'],
        ];
        let word = String::from("aaaaaaaaaaaaa");
        assert_eq!(false, Solution::exist(board, word));
    }
}
