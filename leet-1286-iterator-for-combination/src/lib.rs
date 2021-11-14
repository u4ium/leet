use std::cmp::Ordering;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::BitOr;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetterBitSet {
    data: u32,
}

pub struct SubsetIterator {
    set: u32,
    mask: u32,
}

impl Iterator for SubsetIterator {
    type Item = LetterBitSet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            return None;
        }
        let data = self.mask;
        self.mask -= 1;
        self.mask &= self.set;
        Some(LetterBitSet { data })
    }
}

impl LetterBitSet {
    const S: u8 = b'a';
    const E: u8 = b'z';

    #[inline]
    pub fn new() -> Self {
        LetterBitSet { data: 0 }
    }

    #[inline]
    fn bit_mask(value: u8) -> u32 {
        assert!(
            value >= Self::S,
            "Cannot insert to BitSet: value ({}) less than min ({})",
            value,
            Self::S
        );
        assert!(
            value <= Self::E,
            "Cannot insert to BitSet: value ({}) greater than max ({})",
            value,
            Self::E
        );
        1 << (value - Self::S)
    }

    pub fn contains(&self, value: u8) -> bool {
        self.data & Self::bit_mask(value) != 0
    }

    pub fn len(&self) -> usize {
        self.data.count_ones() as usize
    }

    pub fn subsets(&self) -> SubsetIterator {
        SubsetIterator {
            set: self.data,
            mask: self.data,
        }
    }

    pub fn iter(&self) -> BitSetIterator {
        self.clone().into_iter()
    }
}

pub struct BitSetIterator {
    range: RangeInclusive<u8>,
    data: LetterBitSet,
}

impl Iterator for BitSetIterator {
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

impl IntoIterator for LetterBitSet {
    type Item = u8;
    type IntoIter = BitSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.data.trailing_zeros() as u8 + LetterBitSet::S;
        let end = LetterBitSet::S + (31 - self.data.leading_zeros()) as u8;
        BitSetIterator {
            range: start..=end,
            data: self,
        }
    }
}

impl FromIterator<u8> for LetterBitSet {
    fn from_iter<I: IntoIterator<Item = u8>>(iterator: I) -> Self {
        LetterBitSet {
            data: iterator
                .into_iter()
                .map(Self::bit_mask)
                .reduce(u32::bitor)
                .unwrap_or(0),
        }
    }
}

struct CombinationIterator {
    combinations: Vec<String>,
}

impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let all_characters: LetterBitSet = characters.as_bytes().iter().cloned().collect();
        let mut combinations: Vec<_> = all_characters
            .subsets()
            .filter(|subset| subset.len() == combination_length as usize)
            .map(|subset| unsafe { String::from_utf8_unchecked(subset.iter().collect()) })
            .collect();
        combinations.sort();
        combinations.reverse();
        CombinationIterator { combinations }
    }

    fn next(&mut self) -> String {
        self.combinations.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        self.combinations.len() > 0
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
        let mut combinations = CombinationIterator::new("abc".into(), 2);
        assert_eq!(true, combinations.has_next());
        assert_eq!("ab".to_owned(), combinations.next());
        assert_eq!(true, combinations.has_next());
        assert_eq!("ac".to_owned(), combinations.next());
        assert_eq!(true, combinations.has_next());
        assert_eq!("bc".to_owned(), combinations.next());
        assert_eq!(false, combinations.has_next());
        assert_eq!(false, combinations.has_next());
    }

    #[test]
    fn extra_1() {
        let mut combinations = CombinationIterator::new("bvwz".into(), 2);
        assert_eq!(true, combinations.has_next());
        assert_eq!("bv".to_owned(), combinations.next());
        assert_eq!(true, combinations.has_next());
        assert_eq!("bw".to_owned(), combinations.next());
        assert_eq!(true, combinations.has_next());
        assert_eq!("bz".to_owned(), combinations.next());
        assert_eq!(true, combinations.has_next());
        assert_eq!(true, combinations.has_next());
    }

    #[test]
    fn extra_2() {
        let mut combinations = CombinationIterator::new("bvwx".into(), 2);
        assert_eq!(true, combinations.has_next());
        assert_eq!("bv".to_owned(), combinations.next());
        assert_eq!(true, combinations.has_next());
        assert_eq!("bw".to_owned(), combinations.next());
        assert_eq!(true, combinations.has_next());
        assert_eq!("bx".to_owned(), combinations.next());
        assert_eq!(true, combinations.has_next());
        assert_eq!(true, combinations.has_next());
    }
}
