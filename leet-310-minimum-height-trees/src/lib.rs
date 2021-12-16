pub struct Solution;

use std::convert::TryInto;

pub struct Graph {
    adjacency_lists: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut adjacency_lists = vec![vec![]; n.try_into().unwrap()];

        for edge in edges {
            let a: usize = edge[0].try_into().unwrap();
            let b: usize = edge[1].try_into().unwrap();
            adjacency_lists[a].push(b);
            adjacency_lists[b].push(a);
        }

        Self { adjacency_lists }
    }

    pub fn height_from(&self, n: usize) -> usize {
        1 + self.adjacency_lists[n]
            .iter()
            .map(|&a| self._height_from(a, n))
            .max()
            .unwrap_or_default()
    }

    fn _height_from(&self, n: usize, parent: usize) -> usize {
        1 + self.adjacency_lists[n]
            .iter()
            .filter_map(|&a| {
                if a == parent {
                    None
                } else {
                    Some(self._height_from(a, n))
                }
            })
            .max()
            .unwrap_or_default()
    }
}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let graph = Graph::new(n, edges);
        let heights: Vec<usize> = (0..n.try_into().unwrap())
            .map(|i| graph.height_from(i))
            .collect();
        let min_height = *heights.iter().min().unwrap();
        heights
            .into_iter()
            .enumerate()
            .filter_map(|(node, height)| {
                if height == min_height {
                    node.try_into().ok()
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn find_min_height_trees(n: i32, edges: Vec<[i32; 2]>) -> Vec<i32> {
        Solution::find_min_height_trees(n, edges.into_iter().map(|pair| pair.to_vec()).collect())
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn example_1() {
        assert_eq!(
            vec![1],
            find_min_height_trees(4, vec![[1, 0], [1, 2], [1, 3]])
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            vec![3, 4],
            find_min_height_trees(6, vec![[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![0], find_min_height_trees(1, vec![]));
    }

    #[test]
    fn example_4() {
        assert_eq!(vec![0, 1], find_min_height_trees(2, vec![[0, 1]]));
    }
}
