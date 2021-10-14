struct Solution {}

fn is_digits(chars: &[u8]) -> bool {
    for c in chars.iter() {
        match c {
            b'0'..=b'9' => (),
            _ => {
                return false;
            }
        }
    }
    chars.len() > 0
}

fn trim_sign(chars: &[u8]) -> &[u8] {
    match chars[0] {
        b'+' | b'-' => &chars[1..],
        _ => chars,
    }
}

fn is_integer(s: &str) -> bool {
    if s.len() == 0 {
        return false;
    }
    let bytes = s.as_bytes();
    let bytes = trim_sign(bytes);
    is_digits(bytes)
}

fn is_decimal_number(s: &str) -> bool {
    if s.len() == 0 {
        return false;
    }
    let bytes = s.as_bytes();
    let bytes = trim_sign(bytes);
    let parts: Vec<&[u8]> = bytes.split(|&b| b == b'.').collect();
    if parts.len() != 2 {
        return false;
    }
    let left = parts[0];
    let right = parts[1];
    let left_is_digits = is_digits(parts[0]);
    let right_is_digits = is_digits(parts[1]);

    const EMPTY: [u8; 0] = [];
    left_is_digits && right_is_digits
        || left_is_digits && right == EMPTY
        || right_is_digits && left == EMPTY
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        const ES: [char; 2] = ['e', 'E'];
        let parts: Vec<&str> = s.split(&ES[..]).collect();
        match parts.len() {
            x if x < 1 => false,
            1 => is_decimal_number(parts[0]) || is_integer(parts[0]),
            2 => (is_decimal_number(parts[0]) || is_integer(parts[0])) && is_integer(parts[1]),
            _ => false,
        }
    }
}

#[cfg(test)]
mod solution_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_number(String::from("0")), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_number(String::from("e")), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::is_number(String::from(".")), false);
    }
    #[test]
    fn example_4() {
        assert_eq!(Solution::is_number(String::from(".1")), true);
    }

    #[test]
    fn valid_number_1() {
        assert_eq!(Solution::is_number(String::from("2")), true);
    }

    #[test]
    fn valid_number_2() {
        assert_eq!(Solution::is_number(String::from("0089")), true);
    }

    #[test]
    fn valid_number_3() {
        assert_eq!(Solution::is_number(String::from("-0.1")), true);
    }

    #[test]
    fn valid_number_4() {
        assert_eq!(Solution::is_number(String::from("+3.14")), true);
    }

    #[test]
    fn valid_number_5() {
        assert_eq!(Solution::is_number(String::from("4.")), true);
    }

    #[test]
    fn valid_number_6() {
        assert_eq!(Solution::is_number(String::from("-.9")), true);
    }

    #[test]
    fn valid_number_7() {
        assert_eq!(Solution::is_number(String::from("2e10")), true);
    }

    #[test]
    fn valid_number_8() {
        assert_eq!(Solution::is_number(String::from("-90E3")), true);
    }

    #[test]
    fn valid_number_9() {
        assert_eq!(Solution::is_number(String::from("3e+7")), true);
    }

    #[test]
    fn valid_number_10() {
        assert_eq!(Solution::is_number(String::from("+6e-1")), true);
    }

    #[test]
    fn valid_number_11() {
        assert_eq!(Solution::is_number(String::from("53.5e93")), true);
    }

    #[test]
    fn valid_number_12() {
        assert_eq!(Solution::is_number(String::from("-123.456e789")), true);
    }

    #[test]
    fn invalid_number_1() {
        assert_eq!(Solution::is_number(String::from("abc")), false);
    }

    #[test]
    fn invalid_number_2() {
        assert_eq!(Solution::is_number(String::from("1a")), false);
    }

    #[test]
    fn invalid_number_3() {
        assert_eq!(Solution::is_number(String::from("1e")), false);
    }

    #[test]
    fn invalid_number_4() {
        assert_eq!(Solution::is_number(String::from("e3")), false);
    }

    #[test]
    fn invalid_number_5() {
        assert_eq!(Solution::is_number(String::from("99e2.5")), false);
    }

    #[test]
    fn invalid_number_6() {
        assert_eq!(Solution::is_number(String::from("--6")), false);
    }

    #[test]
    fn invalid_number_7() {
        assert_eq!(Solution::is_number(String::from("-+3")), false);
    }

    #[test]
    fn invalid_number_8() {
        assert_eq!(Solution::is_number(String::from("95a54e53")), false);
    }

    #[test]
    fn extra() {
        assert_eq!(Solution::is_number(String::from("i.1")), false);
    }
}

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn test_is_digits_empty() {
        assert_eq!(is_digits(&[]), false);
    }

    #[test]
    fn test_is_digits_single_number() {
        for n in b'0'..b'9' {
            assert_eq!(is_digits(&[n]), true);
        }
    }

    #[test]
    fn test_is_digits_single_letter() {
        for n in b'a'..b'z' {
            assert_eq!(is_digits(&[n]), false);
        }
    }
}
