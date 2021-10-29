#[derive(Debug)]
pub enum Freshness {
    Rotten,
    Fresh,
}
use Freshness::*;

#[derive(Debug)]
pub enum Cell {
    Empty,
    Orange(Freshness),
}
use Cell::*;

impl From<i32> for Cell {
    #[inline]
    fn from(n: i32) -> Self {
        match n {
            0 => Empty,
            1 => Orange(Fresh),
            2 => Orange(Rotten),
            _ => panic!("Unknown value for cell: {}", n),
        }
    }
}
