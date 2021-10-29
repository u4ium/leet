pub struct Coordinate {
    pub r: usize,
    pub c: usize,
}

impl Coordinate {
    #[inline]
    pub fn up(&self) -> Coordinate {
        Coordinate {
            r: self.r - 1,
            c: self.c,
        }
    }
    #[inline]
    pub fn down(&self) -> Coordinate {
        Coordinate {
            r: self.r + 1,
            c: self.c,
        }
    }
    #[inline]
    pub fn left(&self) -> Coordinate {
        Coordinate {
            r: self.r,
            c: self.c - 1,
        }
    }
    #[inline]
    pub fn right(&self) -> Coordinate {
        Coordinate {
            r: self.r,
            c: self.c + 1,
        }
    }
}
