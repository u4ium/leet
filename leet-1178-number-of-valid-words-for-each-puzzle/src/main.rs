use leet_1178_number_of_valid_words_for_each_puzzle::bitset::LetterBitSet;
use std::ops::RangeInclusive;

fn main() {
    println!("hello");
    const ALL_LETTERS: RangeInclusive<u8> = b'a'..=b'z';
    let mut bitset: LetterBitSet = ALL_LETTERS.collect();
    for vowel in [b'a', b'e', b'i', b'o', b'u'] {
        bitset.remove(vowel);
    }
    for letter in bitset {
        println!("BitSet contains consonant: {:?}", letter)
    }
}
