pub struct Solution {}

impl Solution {
    /// Given an integer array `nums`,
    /// return an array `answer` such that `answer[i]` is equal to
    /// the product of all the elements of `nums` except `nums[i]`.
    ///
    /// # Panics
    /// If the product of any prefix or suffix of nums does not fit in a 32-bit integer.
    ///
    /// ## Constraints
    ///  - `2 <= nums.length <= 10⁵`
    ///  - `-30 <= nums[i] <= 30`
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        #[inline]
        fn products_not_including<'a, I: Iterator<Item = &'a i32>>(nums_iterator: I) -> Vec<i32> {
            let mut product = 1;
            nums_iterator
                .map(|n| {
                    let current_product = product;
                    product *= n;
                    current_product
                })
                .collect()
        }

        let up_to: Vec<i32> = products_not_including(nums.iter());
        let back_from: Vec<i32> = products_not_including(nums.iter().rev());

        let last_index = nums.len() - 1;
        nums.into_iter()
            .enumerate()
            .map(|(i, _n)| {
                let before = up_to[i];
                let after = back_from[last_index - i];
                before * after
            })
            .collect()
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
        let nums = vec![1, 2, 3, 4];
        let expect = vec![24, 12, 8, 6];
        assert_eq!(expect, Solution::product_except_self(nums));
    }

    #[test]
    fn example_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expect = vec![0, 0, 9, 0, 0];
        assert_eq!(expect, Solution::product_except_self(nums));
    }
}
