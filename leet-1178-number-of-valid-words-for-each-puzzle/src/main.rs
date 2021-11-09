use leet_1178_number_of_valid_words_for_each_puzzle::bitset::BitSet;
use std::ops::RangeInclusive;

fn main() {
    println!("hello");
    const ALL_LETTERS: RangeInclusive<u8> = b'a'..=b'z';
    let bitset: BitSet = ALL_LETTERS.collect();
    for letter in ALL_LETTERS {
        println!("BitSet contains {}: {}", letter, bitset.contains(letter))
    }
}
