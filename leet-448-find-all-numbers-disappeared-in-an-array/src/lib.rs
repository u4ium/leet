struct Solution {}

impl Solution {
    /// Given an array nums of n integers where nums[i] is in the range [1, n],
    /// return an array of all the integers in the range [1, n] that do not appear in nums.
    ///
    /// REQ:
    ///  -> n == nums.length
    ///  -> 1 <= n <= 10^5
    ///  -> 1 <= nums[i] <= n
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        // Invert sign of nums[i] for each i where i+1 is in nums
        for index in 0..nums.len() {
            let num = if nums[index] < 0 {
                -nums[index]
            } else {
                nums[index]
            };

            let num_index = num as usize - 1;
            if nums[num_index] > 0 {
                nums[num_index] *= -1;
            }
        }

        // Return i+1 for each nums[i] which is still positive (not found)
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, a)| if a > 0 { Some(i as i32 + 1) } else { None })
            .collect()
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
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![2], Solution::find_disappeared_numbers(vec![1, 1]));
    }
}
