pub struct Solution;

pub mod primes;
use primes::Sieve;

pub mod disjoint_sets;
use disjoint_sets::DisjointSets;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    /// Return the size of the largest connected component in the graph,
    /// given an integer array of unique positive integers nums.
    ///
    /// Consider the following graph:
    ///  - There are nums.length nodes, labeled nums[0] to nums[nums.length - 1],
    ///  - There is an undirected edge between nums[i] and nums[j]
    ///         if nums[i] and nums[j] share a common factor greater than 1.
    ///
    ///
    /// Constraints:
    ///  + 1 <= nums.length <= 2 * 10^4
    ///  + 1 <= nums[i] <= 10^5
    ///  + All the values of nums are unique.
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        // Initialize a sieve of Eratosthenes for numbers up to the max of nums
        let max_num = *nums.iter().max().unwrap();
        let sieve = Sieve::new(max_num.try_into().unwrap());

        // Initialize a disjoint set for each index of nums
        let mut disjoint_sets = DisjointSets::new(nums.len());

        // Initialize a map of prime factors to set representatives (of disjoint sets)
        let mut factor_to_representative = HashMap::new();

        for (index, num) in nums.into_iter().enumerate() {
            // Use the sieve to find prime factors of each num in nums
            for prime_factor in sieve.prime_factors(num.try_into().unwrap()) {
                match factor_to_representative.entry(prime_factor) {
                    // Merge all disjoint sets that share this prime factor
                    Entry::Occupied(entry) => {
                        disjoint_sets.union(*entry.get(), index);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(index);
                    }
                }
            }
        }

        // Return the size of the largest disjoint set
        disjoint_sets.max_size() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn example_1() {
        let nums = vec![4, 6, 15, 35];
        assert_eq!(4, Solution::largest_component_size(nums));
    }

    #[test]
    fn example_2() {
        let nums = vec![20, 50, 9, 63];
        assert_eq!(2, Solution::largest_component_size(nums));
    }

    #[test]
    fn example_3() {
        let nums = vec![2, 3, 6, 7, 4, 12, 21, 39];
        assert_eq!(8, Solution::largest_component_size(nums));
    }

    #[test]
    fn extra_2() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(6, Solution::largest_component_size(nums));
    }

    mod nums;
    #[test]
    fn extra() {
        assert_eq!(8798, Solution::largest_component_size(nums::EXTRA.to_vec()));
    }
}
