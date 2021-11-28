pub struct Solution;

#[cfg(test)]
mod tests;

pub mod graph;
use graph::Graph;

impl Solution {
    /// Find all possible paths from node `0` to node `n - 1` and return them in any order,
    /// given a directed acyclic graph (DAG) of n nodes labeled from `0` to `n - 1`.
    ///
    /// The graph is given as follows:
    ///     - `graph[i]` is a list of all nodes you can visit from node `i`
    ///     - i.e. there is a directed edge from node `i` to node `graph[i][j]`
    ///
    /// # Panics
    /// If any of the node indices are out of bounds
    ///
    /// ## Constraints
    ///  - `n == graph.length`
    ///  - `2 <= n <= 15`
    ///  - `0 <= graph[i][j] < n`
    ///  - `graph[i][j] != i` (i.e., there will be no self-loops).
    ///  - All the elements of `graph[i]` are unique.
    ///  - The input graph is guaranteed to be a DAG.
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Graph::from(graph)
            .all_paths_in_dag()
            .into_iter()
            .map(|list| list.into_iter().map(|n| n.try_into().unwrap()).collect())
            .collect()
    }
}
