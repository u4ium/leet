struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }
        if n < 3 {
            return 1;
        }
        let mut dp = [0, 1, 1];
        for _ in 3..=n {
            dp = [dp[1], dp[2], dp[0] + dp[1] + dp[2]];
        }
        dp[2]
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
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
