struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut max_elem = 0;
        for num in nums {
            max_elem += num;
            if max_elem < num {
                max_elem = num;
            }
            if max_elem > max_sum {
                max_sum = max_elem;
            }
        }
        max_sum
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
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::max_sub_array(vec![1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(23, Solution::max_sub_array(vec![5, 4, -1, 7, 8]));
    }

    #[test]
    fn example_extra() {
        assert_eq!(-1, Solution::max_sub_array(vec![-2, -1]))
    }

    #[test]
    fn example_extra_2() {
        assert_eq!(3, Solution::max_sub_array(vec![1, 2, -1, -2, 2, 1, -2, 1]))
    }

    #[test]
    fn example_extra_3() {
        assert_eq!(
            5,
            Solution::max_sub_array(vec![-1, -2, -2, -2, 3, 2, -2, 0])
        )
    }

    #[test]
    fn example_extra_4() {
        assert_eq!(-1, Solution::max_sub_array(vec![-1, -2]))
    }
}
