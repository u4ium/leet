use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitSet {
    data: u32,
}

pub struct SubsetIterator {
    set: u32,
    mask: u32,
}

impl Iterator for SubsetIterator {
    type Item = BitSet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            return None;
        }
        let subset = self.mask;
        self.mask = (self.mask - 1) & self.set;
        Some(BitSet { data: subset })
    }
}

impl BitSet {
    pub fn new() -> Self {
        BitSet { data: 0 }
    }

    pub fn insert(&mut self, value: u8) {
        self.data |= 1 << (value - b'a');
    }

    pub fn contains(&self, value: u8) -> bool {
        self.data & (1 << (value - b'a')) != 0
    }

    pub fn difference(&self, other: &Self) -> BitSet {
        BitSet {
            data: self.data & !(self.data & other.data),
        }
    }

    pub fn len(&self) -> usize {
        (b'a'..=b'z')
            .filter(|value| self.data & (1 << (value - b'a')) != 0)
            .count()
    }

    pub fn subsets(&self) -> SubsetIterator {
        SubsetIterator {
            set: self.data,
            mask: self.data,
        }
    }
}

impl FromIterator<u8> for BitSet {
    fn from_iter<I: IntoIterator<Item = u8>>(iterator: I) -> Self {
        BitSet {
            data: iterator.into_iter().fold(0, |a, i| a | (1 << (i - b'a'))),
        }
    }
}
