use std::collections::HashMap;
use std::collections::HashSet;

/// A disjoint set of indices continuous from 0, up to but not including size
#[derive(Debug, PartialEq)]
pub struct DisjointSets {
    /// The representative element of each set (initially the index itself)
    parents: Vec<usize>,
    /// The size of each disjoint set whose representative is at this index
    sizes: Vec<usize>,
}

impl DisjointSets {
    /// Create a new collection of disjoint sets of indices from 0 to size (exclusive)
    pub fn new(size: usize) -> Self {
        DisjointSets {
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    /// Merge the two disjoint sets of which a and b are members
    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.find_representative_and_compress_path(a);
        let b = self.find_representative_and_compress_path(b);
        // If the representatives of a and b are dissimilar,
        if a != b {
            // then join the smaller one onto the larger one.
            if self.sizes[a] < self.sizes[b] {
                self.parents[a] = b;
                self.sizes[b] += self.sizes[a];
            } else {
                self.parents[b] = a;
                self.sizes[a] += self.sizes[b];
            }
        }
    }

    /// Return the size of the largest disjoint set in self
    pub fn max_size(&self) -> usize {
        *self.sizes.iter().max().unwrap()
    }

    /// Find the representative element of this set and point each traversed node directly at it
    fn find_representative_and_compress_path(&mut self, i: usize) -> usize {
        let representative = self.find_representative(i);
        self.compress(i, representative);
        representative
    }

    /// Perform compression along path from->...->representative
    ///
    /// (i.e. point each node along the path directly at the representative)
    #[inline]
    fn compress(&mut self, from: usize, representative: usize) {
        let mut current = self.parents[from];
        while current != representative {
            let next = self.parents[current];
            self.parents[current] = representative;
            // self.sizes[j] = 1; // unneeded for correctness, but maintains RI
            current = next;
        }
    }

    /// Find the representative element of the set containing i
    #[inline]
    fn find_representative(&self, i: usize) -> usize {
        let mut current = i;
        while current != self.parents[current] {
            current = self.parents[current];
        }
        current
    }

    /// Return a list of sets (in some order) that is equivalent to these disjoint sets
    pub fn sets(&self) -> Vec<HashSet<usize>> {
        let mut result: HashMap<usize, HashSet<usize>> = HashMap::new();
        for index in 0..self.parents.len() {
            let representative = self.find_representative(index);
            result.entry(representative).or_default().insert(index);
        }
        result.drain().map(|(_k, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut dj_sets = DisjointSets::new(5);
        dj_sets.union(0, 2);
        dj_sets.union(2, 4);
        assert_eq!(3, dj_sets.max_size());
        let set1: HashSet<usize> = vec![1].into_iter().collect();
        let set2: HashSet<usize> = vec![0, 2, 4].into_iter().collect();
        let set3: HashSet<usize> = vec![3].into_iter().collect();
        let sets = dj_sets.sets();
        assert!(sets.contains(&set1));
        assert!(sets.contains(&set2));
        assert!(sets.contains(&set3));
        assert_eq!(3, sets.len());
    }
}
