struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (0..32).fold(0, |a, i| a + ((1 & x >> i) ^ (1 & y >> i)))
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
        assert_eq!(2, Solution::hamming_distance(1, 4))
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::hamming_distance(3, 1))
    }

    #[test]
    fn my_tests() {
        assert_eq!(0, Solution::hamming_distance(0b11011, 0b11011));
        assert_eq!(1, Solution::hamming_distance(0b11011, 0b11001));
        assert_eq!(2, Solution::hamming_distance(0b10011, 0b11001));
        assert_eq!(4, Solution::hamming_distance(0b100011, 0b11001));
    }
}
