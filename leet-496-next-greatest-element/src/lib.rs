struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut next_greater: HashMap<i32, i32> = HashMap::new();
        let mut none_greater_stack: Vec<i32> = Vec::new();
        for num in nums2.into_iter() {
            while let Some(&prev) = none_greater_stack.last() {
                if prev > num {
                    // NOTE: the stack is always in descending order
                    break;
                }
                none_greater_stack.pop();
                next_greater.insert(prev, num);
            }
            none_greater_stack.push(num);
        }
        nums1
            .iter()
            .map(|n| *next_greater.get(n).unwrap_or(&-1))
            .collect()
        // let find_next_greater_in_nums2 = |num: i32| -> i32 {
        //     let mut next_greater = -1;
        //     let start_index = nums2.iter().position(|&n| n == num).unwrap(); // SAFE
        //     for index in start_index + 1..nums2.len() {
        //         if nums2[index] < num {
        //             continue; // not greater
        //         }
        //         next_greater = nums2[index];
        //         break;
        //     }
        //     next_greater
        // };
        // nums1.into_iter().map(find_next_greater_in_nums2).collect()
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
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let expect = vec![-1, 3, -1];
        assert_eq!(expect, Solution::next_greater_element(nums1, nums2));
    }

    #[test]
    fn example_2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let expect = vec![3, -1];
        assert_eq!(expect, Solution::next_greater_element(nums1, nums2));
    }
}
