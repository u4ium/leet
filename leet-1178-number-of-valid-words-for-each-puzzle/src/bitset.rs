use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::RangeInclusive;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

trait GetBits {
    fn get_bits() -> Self;
}

impl GetBits for u32 {
    fn get_bits() -> Self {
        Self::BITS
    }
}

trait UnsignedInt:
    Clone + PartialEq + Eq + Hash + BitOr + BitAnd + BitAndAssign + BitOrAssign + Not + GetBits
{
    // TODO: make generic D for data of BitSet supporting operations on unsigned integers (up to 256 bits)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitSet<const S: u8, const E: u8> {
    data: u32,
}

pub struct SubsetIterator<const S: u8, const E: u8> {
    set: u32,
    mask: u32,
}

impl<const S: u8, const E: u8> Iterator for SubsetIterator<S, E> {
    type Item = BitSet<S, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            return None;
        }
        let subset = self.mask;
        self.mask = (self.mask - 1) & self.set;
        Some(BitSet { data: subset })
    }
}

impl<const S: u8, const E: u8> BitSet<S, E> {
    #[inline]
    pub fn new() -> Self {
        BitSet { data: 0 }
    }

    #[inline]
    fn bitmask(value: u8) -> u32 {
        assert!(
            value >= S,
            "Cannot insert to BitSet: value ({}) less than min ({})",
            value,
            S
        );
        assert!(
            value <= E,
            "Cannot insert to BitSet: value ({}) greater than max ({})",
            value,
            E
        );
        1 << (value - S)
    }

    pub fn insert(&mut self, value: u8) {
        self.data |= Self::bitmask(value);
    }

    pub fn remove(&mut self, value: u8) {
        self.data &= !Self::bitmask(value)
    }

    pub fn contains(&self, value: u8) -> bool {
        self.data & Self::bitmask(value) != 0
    }

    pub fn difference(&self, other: &Self) -> BitSet<S, E> {
        BitSet {
            data: self.data & !(self.data & other.data),
        }
    }

    pub fn union(&self, other: &Self) -> BitSet<S, E> {
        BitSet {
            data: self.data | other.data,
        }
    }

    pub fn intersection(&self, other: &Self) -> BitSet<S, E> {
        BitSet {
            data: self.data & other.data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.count_ones() as usize
    }

    pub fn subsets(&self) -> SubsetIterator<S, E> {
        SubsetIterator {
            set: self.data,
            mask: self.data,
        }
    }
}

pub struct BitSetIterator<const S: u8, const E: u8> {
    range: RangeInclusive<u8>,
    data: BitSet<S, E>,
}

impl<const S: u8, const E: u8> Iterator for BitSetIterator<S, E> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(value) = self.range.next() {
            if self.data.contains(value) {
                return Some(value);
            }
        }
        None
    }
}

impl<const S: u8, const E: u8> IntoIterator for BitSet<S, E> {
    type Item = u8;
    type IntoIter = BitSetIterator<S, E>;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.data.trailing_zeros() as u8 + S;
        let end = S + (u32::get_bits() - 1 - self.data.leading_zeros()) as u8;
        BitSetIterator {
            range: start..=end,
            data: self,
        }
    }
}

impl<const S: u8, const E: u8> FromIterator<u8> for BitSet<S, E> {
    fn from_iter<I: IntoIterator<Item = u8>>(iterator: I) -> Self {
        BitSet {
            data: iterator
                .into_iter()
                .map(Self::bitmask)
                .reduce(u32::bitor)
                .unwrap_or(0),
        }
    }
}

pub type LetterBitSet = BitSet<b'a', b'z'>;
pub type DigitBitSet = BitSet<b'0', b'9'>;
pub type NaturalNumberBitSet = BitSet<0, 31>;
