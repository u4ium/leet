pub struct Solution;

use std::collections::HashSet;
use std::convert::TryInto;

pub struct Graph {
    num_nodes: usize,
    adjacency_lists: Vec<HashSet<usize>>,
}

impl Graph {
    pub fn new<E, T>(n: T, edges: Vec<Vec<T>>) -> Self
    where
        E: std::fmt::Debug,
        T: Copy + TryInto<usize, Error = E>,
    {
        let mut adjacency_lists = vec![HashSet::new(); n.try_into().unwrap()];
        for edge in edges {
            let a: usize = edge[0].try_into().unwrap();
            let b: usize = edge[1].try_into().unwrap();
            adjacency_lists[a].insert(b);
            adjacency_lists[b].insert(a);
        }

        let num_nodes = n.try_into().unwrap();
        Self {
            num_nodes,
            adjacency_lists,
        }
    }

    pub fn len(&self) -> usize {
        self.num_nodes
    }

    /// Return the indices of the nodes most central to this tree
    pub fn find_centroids<E, T>(&mut self) -> Vec<T>
    where
        E: std::fmt::Debug,
        T: TryFrom<usize, Error = E>,
    {
        let mut leaves = self.get_leaves();
        while self.len() > 2 {
            leaves = self.cull(leaves);
        }
        leaves.into_iter().map(|n| n.try_into().unwrap()).collect()
    }

    /// Return the leaf nodes' indices
    fn get_leaves(&self) -> Vec<usize> {
        self.adjacency_lists
            .iter()
            .enumerate()
            .filter_map(|(index, neighbours)| {
                if neighbours.len() <= 1 {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Remove leaves from the graph and return new leaves
    fn cull(&mut self, leaves: Vec<usize>) -> Vec<usize> {
        self.num_nodes -= leaves.len();

        let mut new_leaves = vec![];
        for leaf in leaves {
            let leaf_candidate = self.adjacency_lists[leaf].drain().next().unwrap();
            self.adjacency_lists[leaf_candidate].remove(&leaf);

            if self.adjacency_lists[leaf_candidate].len() == 1 {
                new_leaves.push(leaf_candidate);
            }
        }

        new_leaves
    }
}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        Graph::new(n, edges).find_centroids()
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

    #[test]
    fn extra() {
        assert_eq!(vec![0], find_min_height_trees(3, vec![[0, 1], [0, 2]]))
    }
}
