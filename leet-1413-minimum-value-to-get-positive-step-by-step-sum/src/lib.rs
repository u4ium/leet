struct Solution {}

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut minimum = 0;
        let mut current = 0;
        for num in nums {
            current += num;
            if current < minimum {
                minimum = current;
            }
        }
        -minimum + 1
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
        assert_eq!(5, Solution::min_start_value(vec![-3, 2, -3, 4, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::min_start_value(vec![1, 2]));
    }

    #[test]
    fn example_3() {
        assert_eq!(5, Solution::min_start_value(vec![1, -2, -3]));
    }
}
