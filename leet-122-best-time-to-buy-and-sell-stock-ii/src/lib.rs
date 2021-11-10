struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        0
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
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(7, Solution::max_profit(prices));
        // Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
        // Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
        // Total profit is 4 + 3 = 7.
    }

    #[test]
    fn example_2() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(4, Solution::max_profit(prices));
        // Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
        // Total profit is 4.
    }

    #[test]
    fn example_3() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(0, Solution::max_profit(prices));
        // Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0.
    }
}
