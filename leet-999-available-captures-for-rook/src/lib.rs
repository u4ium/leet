struct Solution();

type Chessboard = Vec<Vec<char>>;

fn find_rook(board: &Chessboard) -> [usize; 2] {
    for r in 0..8 {
        for c in 0..8 {
            if board[r][c] == 'R' {
                return [r, c];
            }
        }
    }
    panic!("Rook not found");
}

fn get_next_location(
    current_location: [usize; 2],
    direction: Direction,
    distance: usize,
) -> Option<[usize; 2]> {
    Some(match direction {
        Right => {
            let next_y = current_location[1] + distance;
            if next_y >= 8 {
                return None;
            }
            [current_location[0], next_y]
        }
        Up => {
            if distance > current_location[0] {
                return None;
            }
            [current_location[0] - distance, current_location[1]]
        }
        Left => {
            if distance > current_location[1] {
                return None;
            }
            [current_location[0], current_location[1] - distance]
        }
        Down => {
            let next_x = current_location[0] + distance;
            if next_x >= 8 {
                return None;
            }
            [next_x, current_location[1]]
        }
    })
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}
use Direction::*;

impl Solution {
    /// Return the number of available captures for the rook.
    ///
    /// REQ:
    ///     - board.length == 8
    ///     - board[i].length == 8
    ///     - board[i][j] is either 'R', '.', 'B', or 'p'
    ///     - There is exactly one cell with board[i][j] == 'R'
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        const DIRECTIONS: [Direction; 4] = [Right, Up, Left, Down];
        let rook_location = find_rook(&board);

        let mut available_captures = 0;
        for &direction in DIRECTIONS.iter() {
            for distance in 1..=7 {
                if let Some([x, y]) = get_next_location(rook_location, direction, distance) {
                    match board[x][y] {
                        'p' => {
                            available_captures += 1;
                            break;
                        }
                        'B' => {
                            break;
                        }
                        '.' => (),
                        unknown => panic!("Unexpected piece type: {}", unknown),
                    };
                } else {
                    break;
                }
            }
        }
        available_captures
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
        assert_eq!(
            3,
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            3,
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ])
        );
    }
}
