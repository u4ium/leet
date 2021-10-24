struct Solution {}

fn is_palindrome(chars: &[u8]) -> bool {
    match chars {
        [a, rest @ .., b] if a == b => is_palindrome(rest),
        [_] | [] => true,
        _ => false,
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest = [0, 0];
        let mut current_len = 0;
        let b = s.as_bytes();
        for i in (0..b.len()) {
            for j in (i..b.len()).rev() {
                if current_len < j - i && is_palindrome(&b[i..=j]) {
                    longest = [i, j];
                    current_len = j - i;
                }
            }
        }
        unsafe { std::str::from_utf8_unchecked(&b[longest[0]..=longest[1]]).to_owned() }
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
        let input = String::from("babad");
        let outputs = vec![String::from("aba"), String::from("bab")];
        assert!(outputs.contains(&Solution::longest_palindrome(input)));
    }

    #[test]
    fn example_2() {
        let input = String::from("cbbd");
        let output = String::from("bb");
        assert_eq!(output, Solution::longest_palindrome(input));
    }

    #[test]
    fn example_3() {
        let input = String::from("a");
        let output = String::from("a");
        assert_eq!(output, Solution::longest_palindrome(input));
    }

    #[test]
    fn example_4() {
        let input = String::from("ac");
        let output = String::from("a");
        assert_eq!(output, Solution::longest_palindrome(input));
    }

    #[test]
    fn example_5() {
        let input = String::from("bb");
        let output = String::from("bb");
        assert_eq!(output, Solution::longest_palindrome(input));
    }
}
