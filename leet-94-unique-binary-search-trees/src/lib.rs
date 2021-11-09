struct Solution {}

#[inline]
pub fn catalan(n: i32) -> i32 {
    let mut product = 1.0;
    for k in 2..=n {
        product *= (n + k) as f64 / k as f64;
    }
    product.round() as i32
}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        catalan(n)
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
        assert_eq!(1, Solution::num_trees(1));
    }

    #[test]
    fn example_2() {
        assert_eq!(5, Solution::num_trees(3));
    }

    #[test]
    fn example_3() {
        assert_eq!(42, Solution::num_trees(5));
    }
}
