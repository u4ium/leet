struct Solution {}

use std::collections::{HashMap, HashSet};
use std::str::Chars;

type Board = Vec<Vec<char>>;

enum Direction {
    Right,
    Up,
    Left,
    Down,
}
use Direction::*;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    pub fn next(&self, board: &Board, direction: &Direction) -> Option<Coordinates> {
        let num_rows = board.len();
        let num_columns = board[0].len();
        match (self, direction) {
            (Coordinates { column, row }, Right) if *column < num_columns => Some(Coordinates {
                row: *row,
                column: column + 1,
            }),
            (Coordinates { column, row }, Up) if *row > 0 => Some(Coordinates {
                row: row - 1,
                column: *column,
            }),
            (Coordinates { column, row }, Left) if *column > 0 => Some(Coordinates {
                row: *row,
                column: column - 1,
            }),
            (Coordinates { column, row }, Down) if *row < num_rows => Some(Coordinates {
                row: row + 1,
                column: *column,
            }),
            _ => None,
        }
    }
}

type Trie = Vec<u32>;

fn find_word_helper(
    board: &Board,
    trie: &Trie,
    chars: &[u8],
    used: &mut HashSet<Coordinates>,
    coordinates: Coordinates,
) -> bool {
    const DIRECTIONS: [Direction; 4] = [Right, Up, Left, Down];
    match chars {
        [] => true,
        [first, rest @ ..] => {
            if used.contains(&coordinates) {
                return false;
            }
            // TODO: trie move
            // trie = match trie.get_child(first) {
            //     Some(child) => child,
            //     None => return false
            // };
            used.insert(coordinates.clone());
            for direction in DIRECTIONS.iter() {
                if let Some(next_coordinates) = coordinates.next(board, direction) {
                    if find_word_helper(board, trie, rest, used, next_coordinates) {
                        return true;
                    }
                }
            }
            used.remove(&coordinates);
            false
        }
    }
}

fn find_word(board: &Board, trie: &Trie, chars: &[u8]) -> bool {
    let num_rows = board.len();
    let num_columns = board[0].len();
    for row in 0..num_rows {
        for column in 0..num_columns {
            let mut used: HashSet<Coordinates> = HashSet::new();
            let position = Coordinates { row, column };
            if find_word_helper(&board, &trie, chars, &mut used, position) {
                return true;
            }
        }
    }
    false
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut ret = vec![];
        let trie = vec![]; // TODO
        for word in words.into_iter() {
            if find_word(&board, &trie, word.as_bytes()) {
                ret.push(word);
            }
        }
        ret.sort();
        ret
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
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let dictionary = vec![
            String::from("oath"),
            String::from("pea"),
            String::from("eat"),
            String::from("rain"),
        ];
        let expect = vec![String::from("eat"), String::from("oath")];
        assert_eq!(expect, Solution::find_words(board, dictionary));
    }

    #[test]
    fn example_2() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let dictionary = vec![String::from("abcd")];
        let expect: Vec<String> = vec![];
        assert_eq!(expect, Solution::find_words(board, dictionary));
    }
}
