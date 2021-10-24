struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };
        sign * x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or_default()
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
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn example_2() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn example_3() {
        assert_eq!(21, Solution::reverse(120));
    }

    #[test]
    fn example_4() {
        assert_eq!(0, Solution::reverse(0));
    }
}
