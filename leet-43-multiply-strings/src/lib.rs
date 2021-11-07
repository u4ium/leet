struct Solution {}

use std::fmt::{self, Display, Formatter};
use std::iter::repeat;
use std::ops::{Add, Mul};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    data: String,
}

impl BigInt {
    pub fn new() -> Self {
        BigInt {
            data: String::from("0"),
        }
    }
}

impl FromStr for BigInt {
    type Err = String;

    fn from_str(number_string: &str) -> Result<Self, Self::Err> {
        match number_string.find(|c: char| !c.is_digit(10)) {
            Some(index) => Err(format!("Non digit character at position {}", index)),
            None if number_string.len() == 0 => {
                Err(String::from("Cannot make BigInt from empty string"))
            }
            None => Ok(BigInt {
                data: number_string.to_owned(),
            }),
        }
    }
}

impl Add for BigInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        fn char_add(c1: u8, c2: u8, carry_in: bool) -> (bool, u8) {
            let sum = (c1 - b'0') + (c2 - b'0') + carry_in as u8;
            if sum >= 10 {
                (true, sum - 10 + b'0')
            } else {
                (false, sum + b'0')
            }
        }
        let lhs = self.data.bytes();
        let rhs = rhs.data.bytes();
        let [longest, shortest] = if lhs.len() >= rhs.len() {
            [lhs, rhs]
        } else {
            [rhs, lhs]
        };
        let mut digits = vec![];
        let mut carry = false;
        for (c1, c2) in longest.rev().zip(shortest.rev().chain(repeat(b'0'))) {
            let (c, d) = char_add(c1, c2, carry);
            carry = c;
            digits.push(d);
        }
        if carry {
            digits.push(b'1');
        }
        let digits: Vec<_> = digits.into_iter().rev().collect();
        let data = unsafe { String::from_utf8_unchecked(digits) };
        BigInt { data }
    }
}

impl Mul for BigInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        fn char_multiply(c1: u8, c2: u8, offset: usize) -> BigInt {
            let product = match (c1, c2) {
                (b'0', _) | (_, b'0') => return BigInt::new(),
                (a @ b'1'..=b'9', b @ b'1'..=b'9') => ((a - b'0') * (b - b'0')),
                _ => unreachable!(),
            };
            let zeroes = unsafe { String::from_utf8_unchecked(vec![b'0'; offset]) };
            let data = format!("{}{}", product, zeroes);
            BigInt { data }
        }

        let mut sum: BigInt = BigInt::new();
        for (i1, c1) in self.data.bytes().rev().enumerate() {
            for (i2, c2) in rhs.data.bytes().rev().enumerate() {
                sum = sum + char_multiply(c1, c2, i1 + i2);
            }
        }
        sum
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.data)
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
        let num1: BigInt = num1.parse().unwrap();
        let num2: BigInt = num2.parse().unwrap();
        (num1 * num2).to_string()
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

    #[test]
    fn example_3() {
        assert_eq!(String::from("81"), solution_caller("9", "9"));
    }
}
