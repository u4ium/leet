struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut result = 0;
        for bit_index in 0..32 {
            let bit_mask = 1 << bit_index;
            if (x & bit_mask) ^ (y & bit_mask) != 0 {
                result += 1;
            }
        }

        result
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
}
