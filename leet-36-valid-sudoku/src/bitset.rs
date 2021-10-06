#[derive(Debug, Copy, Clone)]
pub struct BitSet(usize);

impl BitSet {
    pub fn new() -> Self {
        BitSet(0)
    }

    pub fn set(&mut self, i: u8) {
        self.0 |= 1 << i;
    }

    pub fn unset(&mut self, i: u8) {
        self.0 &= !(1 << i);
    }

    pub fn has(&self, i: u8) -> bool {
        self.0 & 1 << i != 0
    }

    pub fn is(&self, value: usize) -> bool {
        self.0 == value
    }
}

impl From<usize> for BitSet {
    fn from(value: usize) -> Self {
        BitSet(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let b = BitSet::new();
        for i in 0..16 {
            assert_eq!(b.has(i), false);
        }
        let b = BitSet::from(0b1111_0000_1111);
        for i in 0..16 {
            let expect = match i {
                0..=3 | 8..=11 => true,
                _ => false,
            };
            assert_eq!(b.has(i), expect);
        }
    }

    #[test]
    fn test_set() {
        let mut b = BitSet::new();
        b.set(0);
        assert_eq!(b.0, 1);
        b.set(1);
        assert_eq!(b.0, 3);
        b.set(4);
        assert_eq!(b.0, 19);
    }

    #[test]
    fn test_unset() {
        let mut b = BitSet::new();
        b.set(0);
        b.set(1);
        b.set(4);
        b.unset(1);
        assert_eq!(b.0, 17);
        b.unset(0);
        assert_eq!(b.0, 16);
        b.unset(4);
        assert_eq!(b.0, 0);
    }

    #[test]
    fn test_has() {
        let b = BitSet::from(0b100011);
        for i in 0..16 {
            let expect = match i {
                0 | 1 | 5 => true,
                _ => false,
            };
            assert_eq!(b.has(i), expect);
        }
    }
}
