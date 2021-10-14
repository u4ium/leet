struct Solution();

impl Solution {
    /// Return true if the given array can be rearranged to form an arithmetic progression.
    ///
    /// REQ:
    ///    -> 2 <= arr.length <= 1000
    ///    -> -10^6 <= arr[i] <= 10^6
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort();
        let difference = arr[1] - arr[0];
        for (idx, &elem) in arr.iter().enumerate().skip(1) {
            if elem != arr[idx - 1] + difference {
                return false;
            }
        }
        true
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
            true,
            Solution::can_make_arithmetic_progression(vec![3, 5, 1])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::can_make_arithmetic_progression(vec![1, 2, 4])
        );
    }
}
