mod bitset;
use bitset::BitSet;

pub type SudokuBoard = Vec<Vec<char>>;

pub struct SudokuState {
    rows: [BitSet; 9],
    columns: [BitSet; 9],
    squares: [BitSet; 9],
}

macro_rules! sq_index {
    ($i: expr, $j: expr) => {
        3 * ($i / 3) + $j / 3
    };
}

impl SudokuState {
    pub fn new(board: &SudokuBoard) -> SudokuState {
        let mut rows = [BitSet::new(); 9];
        let mut columns = [BitSet::new(); 9];
        let mut squares = [BitSet::new(); 9];
        for (i, row) in board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                match cell {
                    '.' => (),
                    '1'..='9' => {
                        let value = cell as u8 - '1' as u8;
                        rows[i].set(value);
                        columns[j].set(value);
                        squares[sq_index!(i, j)].set(value);
                    }
                    _ => panic!("ERROR: invalid cell in given board"),
                }
            }
        }
        SudokuState {
            rows,
            columns,
            squares,
        }
    }

    pub fn is_complete(&self) -> bool {
        const COMPLETE: usize = 0b111_111_111;
        for row in self.rows.iter() {
            if !row.is(COMPLETE) {
                return false;
            }
        }
        for column in self.columns.iter() {
            if !column.is(COMPLETE) {
                return false;
            }
        }
        for square in self.squares.iter() {
            if !square.is(COMPLETE) {
                return false;
            }
        }
        true
    }

    pub fn has_value(&self, i: usize, j: usize, v: u8) -> bool {
        self.rows[i].has(v) || self.columns[j].has(v) || self.squares[sq_index!(i, j)].has(v)
    }

    pub fn set_value(&mut self, i: usize, j: usize, v: u8) {
        self.rows[i].set(v);
        self.columns[j].set(v);
        self.squares[sq_index!(i, j)].set(v);
    }

    pub fn unset_value(&mut self, i: usize, j: usize, v: u8) {
        self.rows[i].unset(v);
        self.columns[j].unset(v);
        self.squares[sq_index!(i, j)].unset(v);
    }
}

#[test]
fn test_square_index() {
    assert_eq!(sq_index!(0, 0), 0);
    assert_eq!(sq_index!(0, 1), 0);
    assert_eq!(sq_index!(0, 2), 0);
    assert_eq!(sq_index!(1, 0), 0);
    assert_eq!(sq_index!(1, 1), 0);
    assert_eq!(sq_index!(1, 2), 0);
    assert_eq!(sq_index!(2, 0), 0);
    assert_eq!(sq_index!(2, 1), 0);
    assert_eq!(sq_index!(2, 2), 0);

    assert_eq!(sq_index!(0, 3), 1);
    assert_eq!(sq_index!(0, 4), 1);
    assert_eq!(sq_index!(0, 5), 1);
    assert_eq!(sq_index!(1, 3), 1);
    assert_eq!(sq_index!(1, 4), 1);
    assert_eq!(sq_index!(1, 5), 1);
    assert_eq!(sq_index!(2, 3), 1);
    assert_eq!(sq_index!(2, 4), 1);
    assert_eq!(sq_index!(2, 5), 1);

    assert_eq!(sq_index!(0, 6), 2);
    assert_eq!(sq_index!(0, 7), 2);
    assert_eq!(sq_index!(0, 8), 2);
    assert_eq!(sq_index!(1, 6), 2);
    assert_eq!(sq_index!(1, 7), 2);
    assert_eq!(sq_index!(1, 8), 2);
    assert_eq!(sq_index!(2, 6), 2);
    assert_eq!(sq_index!(2, 7), 2);
    assert_eq!(sq_index!(2, 8), 2);

    assert_eq!(sq_index!(3, 0), 3);
    assert_eq!(sq_index!(3, 1), 3);
    assert_eq!(sq_index!(3, 2), 3);
    assert_eq!(sq_index!(4, 0), 3);
    assert_eq!(sq_index!(4, 1), 3);
    assert_eq!(sq_index!(4, 2), 3);
    assert_eq!(sq_index!(5, 0), 3);
    assert_eq!(sq_index!(5, 1), 3);
    assert_eq!(sq_index!(5, 2), 3);

    assert_eq!(sq_index!(3, 3), 4);
    assert_eq!(sq_index!(3, 4), 4);
    assert_eq!(sq_index!(3, 5), 4);
    assert_eq!(sq_index!(4, 3), 4);
    assert_eq!(sq_index!(4, 4), 4);
    assert_eq!(sq_index!(4, 5), 4);
    assert_eq!(sq_index!(5, 3), 4);
    assert_eq!(sq_index!(5, 4), 4);
    assert_eq!(sq_index!(5, 5), 4);

    assert_eq!(sq_index!(3, 6), 5);
    assert_eq!(sq_index!(3, 7), 5);
    assert_eq!(sq_index!(3, 8), 5);
    assert_eq!(sq_index!(4, 6), 5);
    assert_eq!(sq_index!(4, 7), 5);
    assert_eq!(sq_index!(4, 8), 5);
    assert_eq!(sq_index!(5, 6), 5);
    assert_eq!(sq_index!(5, 7), 5);
    assert_eq!(sq_index!(5, 8), 5);

    assert_eq!(sq_index!(6, 0), 6);
    assert_eq!(sq_index!(6, 1), 6);
    assert_eq!(sq_index!(6, 2), 6);
    assert_eq!(sq_index!(7, 0), 6);
    assert_eq!(sq_index!(7, 1), 6);
    assert_eq!(sq_index!(7, 2), 6);
    assert_eq!(sq_index!(8, 0), 6);
    assert_eq!(sq_index!(8, 1), 6);
    assert_eq!(sq_index!(8, 2), 6);

    assert_eq!(sq_index!(6, 3), 7);
    assert_eq!(sq_index!(6, 4), 7);
    assert_eq!(sq_index!(6, 5), 7);
    assert_eq!(sq_index!(7, 3), 7);
    assert_eq!(sq_index!(7, 4), 7);
    assert_eq!(sq_index!(7, 5), 7);
    assert_eq!(sq_index!(8, 3), 7);
    assert_eq!(sq_index!(8, 4), 7);
    assert_eq!(sq_index!(8, 5), 7);

    assert_eq!(sq_index!(6, 6), 8);
    assert_eq!(sq_index!(6, 7), 8);
    assert_eq!(sq_index!(6, 8), 8);
    assert_eq!(sq_index!(7, 6), 8);
    assert_eq!(sq_index!(7, 7), 8);
    assert_eq!(sq_index!(7, 8), 8);
    assert_eq!(sq_index!(8, 6), 8);
    assert_eq!(sq_index!(8, 7), 8);
    assert_eq!(sq_index!(8, 8), 8);
}
