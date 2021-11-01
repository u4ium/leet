struct Solution {}

fn polyfill<const C: char>(board: &mut Vec<Vec<char>>, coordinate: [usize; 2]) {
    let [x, y] = coordinate;
    let square = &mut board[x][y];
    match square {
        'O' => *square = C,
        _ => return,
    }
    if x > 0 {
        polyfill::<C>(board, [x - 1, y]);
    }
    if x < board.len() - 1 {
        polyfill::<C>(board, [x + 1, y]);
    }
    if y > 0 {
        polyfill::<C>(board, [x, y - 1]);
    }
    if y < board[0].len() - 1 {
        polyfill::<C>(board, [x, y + 1]);
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let max_x = board.len() - 1;
        let max_y = board[0].len() - 1;

        // polyfill 4 edges with 'I'
        for x in 0..=max_x {
            polyfill::<'I'>(board, [x, 0]);
            polyfill::<'I'>(board, [x, max_y]);
        }
        for y in 0..=max_y {
            polyfill::<'I'>(board, [0, y]);
            polyfill::<'I'>(board, [max_x, y]);
        }

        // polyfill interior with 'X'
        for x in 1..max_x {
            for y in 1..max_y {
                polyfill::<'X'>(board, [x, y]);
            }
        }

        // revert all 'I' to 'O'
        for x in 0..=max_x {
            for y in 0..=max_y {
                let square = &mut board[x][y];
                if square == &'I' {
                    *square = 'O';
                }
            }
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
        assert_eq!(2 + 2, 4);
        let mut input = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let output = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut input);
        assert_eq!(output, input);
    }
}
