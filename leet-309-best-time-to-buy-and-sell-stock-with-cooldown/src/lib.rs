struct Solution {}

impl Solution {
    /// Find the maximum profit you can achieve, given an array prices
    /// where prices[i] is the price of a given stock on the ith day.
    ///
    /// You may complete as many transactions as you like (i.e., buy one and sell one share
    /// of the stock multiple times) with the following restrictions:
    ///     - After you sell your stock, you cannot buy stock on
    ///         the next day (i.e., cooldown one day).
    ///
    /// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell
    /// the stock before you buy again).
    ///
    /// REQ:
    ///     - 1 <= prices.length <= 5000
    ///     - 0 <= prices[i] <= 1000
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut current = i32::MIN;
        // let mut max_difference = 0;
        // for current in prices.into_iter().rev() {
        //      let difference = max_difference - current;
        //      if difference > max_difference {
        //          max_difference = difference;
        //      }
        //      if current > max_difference{
        //          max_difference = current;
        //      }
        // }
        // max_difference
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
        assert_eq!(3, Solution::max_profit(vec![1, 2, 3, 0, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::max_profit(vec![1]));
    }
}
