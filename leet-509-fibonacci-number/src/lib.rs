struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut nums = (1, 1);
        for _ in 2..n {
            nums = (nums.1, nums.0 + nums.1);
        }
        nums.1
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
    fn example_0() {
        assert_eq!(Solution::fib(0), 0);
    }

    #[test]
    fn example_1() {
        assert_eq!(Solution::fib(1), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn example_5() {
        assert_eq!(Solution::fib(5), 5);
    }

    #[test]
    fn example_6() {
        assert_eq!(Solution::fib(6), 8);
    }

    #[test]
    fn example_7() {
        assert_eq!(Solution::fib(7), 13);
    }
}
