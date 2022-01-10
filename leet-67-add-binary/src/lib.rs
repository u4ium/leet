pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let result_length = max(a.len(), b.len()) + 1;
        let mut result_characters = vec![b'0'; result_length];

        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut carry = false;

        let last_bit_index = result_length - 1;
        let last_a_index = a.len() - 1;
        let last_b_index = b.len() - 1;

        for index in 0..result_length {
            let bit_index = last_bit_index - index;
            match (index > last_a_index, index > last_b_index) {
                (true, true) => {
                    result_characters[bit_index] = if carry { b'1' } else { b'0' };
                    // carry = false;
                    break;
                }
                (true, false) => {
                    let b_index = last_b_index - index;
                    result_characters[bit_index] = match (b[b_index], carry) {
                        (b'0', false) => b'0',
                        (b'1', false) => b'1',
                        (b'0', true) => {
                            carry = false;
                            b'1'
                        }
                        (b'1', true) => b'0',
                        _ => unreachable!(),
                    };
                }
                (false, true) => {
                    let a_index = last_a_index - index;
                    result_characters[bit_index] = match (a[a_index], carry) {
                        (b'0', false) => b'0',
                        (b'1', false) => b'1',
                        (b'0', true) => {
                            carry = false;
                            b'1'
                        }
                        (b'1', true) => b'0',
                        _ => unreachable!(),
                    };
                }
                (false, false) => {
                    let a_index = last_a_index - index;
                    let b_index = last_b_index - index;
                    result_characters[bit_index] = match (a[a_index], b[b_index], carry) {
                        (b'0', b'0', false) => b'0',
                        (b'0', b'1', false) => b'1',
                        (b'1', b'0', false) => b'1',
                        (b'1', b'1', false) => {
                            carry = true;
                            b'0'
                        }
                        (b'0', b'0', true) => {
                            carry = false;
                            b'1'
                        }
                        (b'0', b'1', true) => b'0',
                        (b'1', b'0', true) => b'0',
                        (b'1', b'1', true) => b'1',
                        _ => unreachable!(),
                    }
                }
            }
        }

        // strip leading zeroes
        while result_characters.len() > 1 && result_characters[0] == b'0' {
            result_characters.remove(0); // TODO: pointer arithmetic
        }

        unsafe { String::from_utf8_unchecked(result_characters) }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn add_binary(a: &'static str, b: &'static str) -> String {
        Solution::add_binary(a.to_owned(), b.to_owned())
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn example_1() {
        let a = "11";
        let b = "1";
        let expect = "100";
        assert_eq!(expect, &add_binary(a, b)[..])
    }

    #[test]
    fn example_2() {
        let a = "1010";
        let b = "1011";
        let expect = "10101";
        assert_eq!(expect, &add_binary(a, b)[..])
    }

    #[test]
    fn mine() {
        let a = "001010";
        let b = "000001011";
        let expect = "10101";
        assert_eq!(expect, &add_binary(a, b)[..])
    }
}
