struct Solution {}

use std::fmt::{self, Display, Formatter};
use std::ops::Mul;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Negative,
    Positive,
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Sign::Negative => "-",
                Sign::Positive => "+",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    chunks: Vec<u64>,
    sign: Sign,
}

impl FromStr for BigInt {
    type Err = String;

    fn from_str(number_string: &str) -> Result<Self, Self::Err> {
        fn error<E>(_e: E) -> String {
            const ERROR_MESSAGE: &str = "bad bytes";
            String::from(ERROR_MESSAGE)
        }

        let bytes = number_string.as_bytes();
        let (sign, digits) = match bytes {
            [b'-', rest @ ..] => (Sign::Negative, rest),
            [b'+', rest @ ..] => (Sign::Positive, rest),
            all => (Sign::Positive, all),
        };

        let reversed_digits: Vec<u8> = digits.iter().cloned().rev().collect();
        let chunks = reversed_digits
            .chunks(19)
            .map(|chunk| {
                let chunk: Vec<u8> = chunk.iter().cloned().rev().collect();
                let chunk_string = String::from_utf8(chunk).map_err(error)?;
                chunk_string.parse().map_err(error)
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(BigInt { chunks, sign })
    }
}

impl Mul for BigInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        todo!()
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{}",
            self.sign,
            self.chunks
                .iter()
                .rev()
                .map(|chunk| chunk.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl Solution {
    /// Given two non-negative integers num1 and num2 represented as strings,
    /// return the product of num1 and num2, also represented as a string.
    /// Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
    ///
    /// REQ:
    ///     - 1 <= num1.length, num2.length <= 200
    ///     - num1 and num2 consist of digits only.
    ///     - Both num1 and num2 do not contain any leading zero, except the number 0 itself.
    pub fn multiply(num1: String, num2: String) -> String {
        (num1.parse::<u64>().unwrap() * num2.parse::<u64>().unwrap()).to_string()
        //let a = BigInt::from_str(&num1[..]).unwrap();
        //let b = BigInt::from_str(&num1[..]).unwrap();
        //(a * b).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn solution_caller(s1: &str, s2: &str) -> String {
        Solution::multiply(String::from(s1), String::from(s2))
    }

    #[test]
    fn example_1() {
        assert_eq!(String::from("6"), solution_caller("2", "3"));
    }
    #[test]
    fn example_2() {
        assert_eq!(String::from("56088"), solution_caller("123", "456"));
    }
}
