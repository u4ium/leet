mod coordinate;
use coordinate::Coordinate;
mod cell;
use cell::{
    Cell::{self, *},
    Freshness::*,
};
pub mod rot_result;
pub use rot_result::*;

pub struct Grid(Vec<Vec<Cell>>);

impl From<Vec<Vec<i32>>> for Grid {
    fn from(grid: Vec<Vec<i32>>) -> Self {
        Grid(
            grid.iter()
                .map(|row| row.iter().map(|&c| Cell::from(c)).collect())
                .collect(),
        )
    }
}

impl Grid {
    /// Convert fresh Oranges that are immediately adjacent to rotten ones to rotten.
    ///
    /// Return a RotResult indicating completeness (before or after rotting), progress, or failure.
    pub fn rot(&mut self) -> RotResult {
        // count fresh oranges and find coordinates of rotten ones
        let mut num_fresh = 0;
        let mut rotten_coordinates = vec![];
        for (r, row) in self.0.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                match cell {
                    Orange(Rotten) => rotten_coordinates.push(Coordinate { r, c }),
                    Orange(Fresh) => num_fresh += 1,
                    Empty => (),
                }
            }
        }

        if num_fresh == 0 {
            // return result indicating no rotting is left to be done
            return RotResult::Complete;
        }

        // rot each fresh orange that is adjacent to a rotten one
        let mut num_fresh_rotted = 0;
        for rotten_coordinate in rotten_coordinates.iter() {
            for next_coordinate in self.get_adjacent_coordinates(rotten_coordinate) {
                if let Orange(f @ Fresh) = &mut self.0[next_coordinate.r][next_coordinate.c] {
                    *f = Rotten;
                    num_fresh_rotted += 1;
                }
            }
        }

        // return result indicating progress, completion, or failure
        RotResult::Rot(match num_fresh_rotted {
            0 => RottingState::Stuck,
            x if x == num_fresh => RottingState::Done,
            _ => RottingState::Rotting,
        })
    }

    #[inline]
    fn get_adjacent_coordinates(&self, current: &Coordinate) -> Vec<Coordinate> {
        let mut result = vec![];
        if current.r > 0 {
            result.push(current.up());
        }
        if current.r < self.0.len() - 1 {
            result.push(current.down());
        }
        if current.c > 0 {
            result.push(current.left());
        }
        if current.c < self.0[0].len() - 1 {
            result.push(current.right());
        }
        result
    }
}
