pub struct Solution {}

use std::cmp::max;

/// A closed interval
#[derive(Clone, Copy)]
struct Interval {
    /// The start of this interval
    start: i32,
    /// The end of this interval
    end: i32,
}

impl Interval {
    /// Create a new interval out of a vector of two elements
    ///
    /// # Panics
    /// If `v` does not have two elements
    fn new(v: Vec<i32>) -> Interval {
        assert_eq!(v.len(), 2);
        Interval {
            start: v[0],
            end: v[1],
        }
    }
}

impl Solution {
    /// Return the intersection of these two interval lists, given two lists of closed intervals:
    ///  - `first_list`, where `first_list[i]` = [startᵢ, endᵢ]
    ///  - and `second_list`, where `second_list[j]` = [startⱼ, endⱼ].
    ///
    /// ## Constraints
    ///  - first_list.length + second_list.length >= 1
    ///  - 0 <= first_list.length, second_list.length <= 1000
    ///  - ∀i, 0 <= startᵢ <= endᵢ <= 10⁹
    ///  - ∀j, 0 <= startⱼ <= endⱼ <= 10⁹
    ///  - Each list of intervals is pairwise disjoint and in sorted order:
    ///    - ∀i, endᵢ < startᵢ₊₁
    ///    - ∀j, endⱼ < startⱼ₊₁
    ///
    /// # Panics
    /// If any interval in `first_list` or `second_list` does not contain two values.
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        // These types exist for naming convenience:
        // (a, b) denote the (first, second)_list respectively

        struct Iterators<I: Iterator<Item = Interval>> {
            a: I,
            b: I,
        }

        struct CurrentIntervals {
            a: Option<Interval>,
            b: Option<Interval>,
        }

        // Initialize the iterators and current intervals
        let mut iterators = Iterators {
            a: first_list.into_iter().map(Interval::new),
            b: second_list.into_iter().map(Interval::new),
        };
        let mut current_intervals = CurrentIntervals {
            a: iterators.a.next(),
            b: iterators.b.next(),
        };

        // Initialize the result
        let mut result = vec![]; // RETURNED

        // While both lists have remaining intervals:
        while let CurrentIntervals {
            a: Some(a),
            b: Some(b),
        } = current_intervals
        {
            // If the end of one interval is before the start of the other, skip it.
            if a.end < b.start {
                current_intervals.a = iterators.a.next();
                continue;
            }
            if b.end < a.start {
                current_intervals.b = iterators.b.next();
                continue;
            }

            // Since the end of one interval is >= the start of the other, they overlap.
            let overlap_start = max(b.start, a.start);

            // Regardless of whichever starts first, the interval that ends first
            // cannot overlap with any later interval.
            let overlap_end = if a.end < b.end {
                current_intervals.a = iterators.a.next();
                a.end
            } else {
                current_intervals.b = iterators.b.next();
                b.end
            };

            // Add this overlap to the result set
            result.push(vec![overlap_start, overlap_end]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    struct TestInput {
        first_list: Vec<[u32; 2]>,
        second_list: Vec<[u32; 2]>,
        expect: Vec<[u32; 2]>,
    }

    struct Test {
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
        expect: Vec<Vec<i32>>,
    }

    impl Test {
        fn new(input: TestInput) -> Self {
            fn convert(list: Vec<[u32; 2]>) -> Vec<Vec<i32>> {
                list.into_iter()
                    .map(|[a, b]| vec![a.try_into().unwrap(), b.try_into().unwrap()])
                    .collect()
            }
            fn are_valid_intervals(list: &Vec<[u32; 2]>) -> bool {
                for index in 1..list.len() {
                    match list[index] {
                        [start, _] if start <= list[index - 1][1] => return false,
                        [start, end] if start > end => return false,
                        _ => (),
                    }
                }
                true
            }

            assert!(input.first_list.len() + input.second_list.len() >= 1);
            assert!(input.first_list.len() <= 1000);
            assert!(input.second_list.len() <= 1000);
            assert!(are_valid_intervals(&input.first_list));
            assert!(are_valid_intervals(&input.second_list));
            assert!(are_valid_intervals(&input.expect));

            Test {
                first_list: convert(input.first_list),
                second_list: convert(input.second_list),
                expect: convert(input.expect),
            }
        }

        fn run(self) {
            assert_eq!(
                self.expect,
                Solution::interval_intersection(self.first_list, self.second_list)
            );
        }
    }

    #[test]
    fn example_1() {
        Test::new(TestInput {
            first_list: vec![[0, 2], [5, 10], [13, 23], [24, 25]],
            second_list: vec![[1, 5], [8, 12], [15, 24], [25, 26]],
            expect: vec![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]],
        })
        .run()
    }

    #[test]
    fn example_2() {
        Test::new(TestInput {
            first_list: vec![[1, 3], [5, 9]],
            second_list: vec![],
            expect: vec![],
        })
        .run()
    }

    #[test]
    fn example_3() {
        Test::new(TestInput {
            first_list: vec![],
            second_list: vec![[4, 8], [10, 12]],
            expect: vec![],
        })
        .run()
    }

    #[test]
    fn example_4() {
        Test::new(TestInput {
            first_list: vec![[1, 7]],
            second_list: vec![[3, 10]],
            expect: vec![[3, 7]],
        })
        .run()
    }

    #[test]
    fn edgy() {
        Test::new(TestInput {
            first_list: vec![[1, 2], [4, 4], [11, 12]],
            second_list: vec![[4, 10], [12, 14], [15, 17]],
            expect: vec![[4, 4], [12, 12]],
        })
        .run()
    }
}
