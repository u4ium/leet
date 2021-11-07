struct Solution {}

impl Solution {
    pub fn arrange_coins(mut n: i32) -> i32 {
        // Gauss's summation formula:
        // for any integer i, the sum from 1..=i is (i(i + 1)) / 2 = m
        // we want to find the minimum integer m >= n and return i
        let n_double = 2 * n as u64;
        let mut guess = (n_double as f32).sqrt().floor() as u64;
        loop {
            let m_double = guess * (guess + 1);
            let m_less_double = guess * (guess - 1);
            if m_double < n_double {
                guess += 1;
                continue;
            }
            if m_less_double >= n_double {
                guess -= 1;
                continue;
            }
            if m_double == n_double {
                return guess as i32;
            } else {
                return guess as i32 - 1;
            };
        }
        // let mut result = 0;
        // let mut row = 1;
        // while n >= row {
        //     n -= row;
        //     result += 1;
        //     row += 1;
        // }
        // result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!(1, Solution::arrange_coins(1));
        assert_eq!(1, Solution::arrange_coins(2));
        assert_eq!(2, Solution::arrange_coins(3));
        assert_eq!(2, Solution::arrange_coins(5));
        assert_eq!(3, Solution::arrange_coins(6));
        assert_eq!(3, Solution::arrange_coins(7));
        assert_eq!(3, Solution::arrange_coins(9));
        assert_eq!(4, Solution::arrange_coins(10));
        assert_eq!(4, Solution::arrange_coins(11));
        assert_eq!(4, Solution::arrange_coins(14));
        assert_eq!(5, Solution::arrange_coins(15));
        assert_eq!(5, Solution::arrange_coins(16));
        assert_eq!(5, Solution::arrange_coins(20));
        assert_eq!(6, Solution::arrange_coins(21));
        assert_eq!(60070, Solution::arrange_coins(1804289383));
    }
}
