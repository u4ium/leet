struct Solution();

use std::convert::TryInto;

type BoardStringRepresentation = Vec<String>;

struct Board {
    columns: u16,
    diagonals: u16,
    antidiagonals: u16,
    queens: Vec<u8>,
    n: u8,
}

macro_rules! diagonal {
    ($n: expr, $r: expr, $c: expr) => {
        ($n - 1) + $r - $c
    };
}

macro_rules! antidiagonal {
    ($n: expr, $r: expr, $c: expr) => {
        ($n - 1) + ($n - 1) - $c - $r
    };
}

impl Board {
    pub fn to_string_representation(&self) -> BoardStringRepresentation {
        self.queens
            .iter()
            .map(|&column| {
                (0..self.n)
                    .map(|c| if c == column { "Q" } else { "." })
                    .collect()
            })
            .collect()
    }

    pub fn can_add_queen(&self, row: u8, column: u8) -> bool {
        if self.columns & (1 << column) != 0 {
            return false;
        }
        if self.diagonals & (1 << diagonal!(self.n, row, column)) != 0 {
            return false;
        }
        if self.antidiagonals & (1 << antidiagonal!(self.n, row, column)) != 0 {
            return false;
        }
        true
    }

    pub fn add_queen(&mut self, row: u8, column: u8) {
        self.columns |= 1 << column;
        self.diagonals |= 1 << diagonal!(self.n, row, column);
        self.antidiagonals |= 1 << antidiagonal!(self.n, row, column);

        self.queens[row as usize] = column;
    }

    pub fn remove_queen(&mut self, row: u8, column: u8) {
        self.columns &= !(1 << column);
        self.diagonals &= !(1 << diagonal!(self.n, row, column));
        self.antidiagonals &= !(1 << antidiagonal!(self.n, row, column));
    }
}

impl Solution {
    /// Given an integer n, return all distinct solutions to the n-queens puzzle.
    ///
    /// REQ: 1 <= n <= 9
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = Board {
            columns: 0,
            diagonals: 0,
            antidiagonals: 0,
            queens: vec![0; n.try_into().unwrap()],
            n: n.try_into().unwrap(),
        };

        find_solutions(&mut board, 0)
    }
}

fn find_solutions(board: &mut Board, row: u8) -> Vec<BoardStringRepresentation> {
    if row == board.n {
        return vec![board.to_string_representation()];
    }

    let mut solutions = vec![];
    for column in 0..board.n {
        if board.can_add_queen(row, column) {
            board.add_queen(row, column);
            solutions.extend(find_solutions(board, row + 1));
            board.remove_queen(row, column);
        }
    }

    solutions
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
        let expect = vec![
            vec![
                String::from(".Q.."),
                String::from("...Q"),
                String::from("Q..."),
                String::from("..Q."),
            ],
            vec![
                String::from("..Q."),
                String::from("Q..."),
                String::from("...Q"),
                String::from(".Q.."),
            ],
        ];
        assert_eq!(expect, Solution::solve_n_queens(4));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![vec![String::from("Q")]], Solution::solve_n_queens(1));
    }

    #[test]
    fn impossible_small_boards() {
        let expect: Vec<BoardStringRepresentation> = vec![];
        assert_eq!(expect, Solution::solve_n_queens(2));
        assert_eq!(expect, Solution::solve_n_queens(3));
    }
}
