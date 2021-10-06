struct Solution {}

mod state;
use state::{SudokuBoard, SudokuState};

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut state = SudokuState::new(board);
        Solution::solve_sudoku_recursive_helper(board, &mut state, 0);
        if !state.is_complete() {
            panic!("Cannot solve given Sudoku problem");
        }
    }

    fn solve_sudoku_recursive_helper(
        board: &mut SudokuBoard,
        state: &mut SudokuState,
        index: usize,
    ) -> bool {
        if index == 81 {
            return true;
        }
        let i = index / 9;
        let j = index % 9;
        match board[i][j] {
            '.' => {
                for v in 0..9 {
                    if state.has_value(i, j, v) {
                        continue;
                    }
                    state.set_value(i, j, v);
                    board[i][j] = (v + '1' as u8) as char;
                    if Solution::solve_sudoku_recursive_helper(board, state, index + 1) {
                        return true;
                    }
                    state.unset_value(i, j, v);
                }
                board[i][j] = '.';
                false
            }
            _ => Solution::solve_sudoku_recursive_helper(board, state, index + 1),
        }
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
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let expected_board = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, expected_board);
    }
}
