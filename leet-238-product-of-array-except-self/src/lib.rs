use std::ops::{Index, Mul, MulAssign};

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
    ///
    /// ## Examples
    ///
    /// ### Simple
    /// ```rust
    /// use leet_238_product_of_array_except_self::Solution;
    /// let nums = vec![1, 2, 3, 4];
    /// let expect = vec![24, 12, 8, 6];
    /// assert_eq!(expect, Solution::product_except_self(nums));
    /// ```
    ///
    /// ### With a zero
    /// ```rust
    /// # use leet_238_product_of_array_except_self::Solution;
    /// let nums = vec![-1, 1, 0, -3, 3];
    /// let expect = vec![0, 0, 9, 0, 0];
    /// assert_eq!(expect, Solution::product_except_self(nums));
    /// ```
    ///
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        nums.product_except_self()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn two_elements() {
        let nums = vec![2, 4];
        let expect = vec![4, 2];
        assert_eq!(expect, Solution::product_except_self(nums));
    }
}

trait MultiplicativeIdentity {
    fn multiplicative_identity() -> Self;
}

impl MultiplicativeIdentity for i32 {
    fn multiplicative_identity() -> Self {
        1
    }
}

trait Productable<T, D>:
    IntoIterator<Item = T, IntoIter = D> + FromIterator<T> + Index<usize, Output = T> + Clone
where
    T: Copy + Mul<Output = T> + MulAssign + MultiplicativeIdentity,
    D: DoubleEndedIterator<Item = T>,
{
    fn product_except_self(&self) -> Self {
        // Compute all products of the prefixes and suffixes of nums
        let up_to: Self = Self::products_up_to(self.clone().into_iter());
        let back_from: Self = Self::products_up_to(self.clone().into_iter().rev())
            .into_iter()
            .rev()
            .collect();

        // Each `product_except_self` is the product of the suffix and prefix
        self.clone()
            .into_iter()
            .enumerate()
            .map(|(i, _n)| up_to[i] * back_from[i])
            .collect()
    }

    #[inline]
    /// Return a list of the products of `nums_iterator` up to (but not including) each element
    fn products_up_to<'a, I: Iterator<Item = T>>(nums_iterator: I) -> Self {
        let mut product = T::multiplicative_identity();
        nums_iterator
            .map(|n| {
                let current_product = product;
                product *= n;
                current_product
            })
            .collect()
    }
}

impl<T> Productable<T, std::vec::IntoIter<T>> for Vec<T> where
    T: Copy + Mul<Output = T> + MulAssign + MultiplicativeIdentity
{
}
