struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Reverse<(i32, Vec<i32>)>> = BinaryHeap::new();
        for point in points {
            let distance = point[0] * point[0] + point[1] * point[1];
            heap.push(Reverse((distance, point)));
        }
        let mut closest = Vec::new();
        while let Some(Reverse((_, point))) = heap.pop() {
            closest.push(point);
            if closest.len() == k as usize {
                break;
            }
        }
        closest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
