pub struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut lengths = [0; 60];
        let mut result = 0;
        for t in time {
            let i = (t % 60) as usize;
            let c = (60 - i) % 60;
            result += lengths[c];
            lengths[i] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn example_1() {
        let time = vec![30, 20, 150, 100, 40];
        assert_eq!(3, Solution::num_pairs_divisible_by60(time));
    }

    #[test]
    fn example_2() {
        let time = vec![60, 60, 60];
        assert_eq!(3, Solution::num_pairs_divisible_by60(time));
    }
}
