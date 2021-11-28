use std::collections::VecDeque;
use std::convert::TryInto;
use std::fmt::Debug;
use std::iter::FromIterator;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BitSet {
    set: u16,
}

impl BitSet {
    #[inline]
    pub fn has(&self, v: usize) -> bool {
        self.set & Self::bitmask(v) != 0
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        (0..16).filter(|&n| self.has(n))
    }

    #[inline]
    fn bitmask(v: usize) -> u16 {
        1 << (15 - v)
    }
}

impl<T, E> FromIterator<T> for BitSet
where
    T: TryInto<usize, Error = E>,
    E: Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self {
        Self {
            set: values
                .into_iter()
                .map(|n| n.try_into().unwrap())
                .fold(0_u16, |a, v| a | Self::bitmask(v)),
        }
    }
}

pub struct Graph {
    matrix: Vec<BitSet>,
}

impl<T, E> From<Vec<Vec<T>>> for Graph
where
    T: TryInto<usize, Error = E>,
    E: Debug,
{
    fn from(adjacency_list: Vec<Vec<T>>) -> Self {
        Self {
            matrix: adjacency_list
                .into_iter()
                .map(|list| list.into_iter().collect())
                .collect(),
        }
    }
}

type Path = Vec<usize>;

impl Graph {
    pub fn all_paths_in_dag(&self) -> Vec<Path> {
        let n = self.matrix.len();
        let last = n - 1;
        let mut paths = vec![vec![]; n];
        paths[last].push(VecDeque::from(vec![last]));
        for (node_index, neighbours) in self.ordered_neighbours() {
            for neighbour in neighbours.iter() {
                for mut path in paths[neighbour].clone() {
                    path.push_front(node_index);
                    paths[node_index].push(path);
                }
            }
        }
        println!("{:?}", paths);
        paths[0]
            .drain(..)
            .map(|deque| deque.into_iter().collect())
            .collect()
    }

    fn ordered_neighbours(&self) -> impl Iterator<Item = (usize, &BitSet)> + '_ {
        let mut n = self
            .matrix
            .iter()
            .enumerate()
            .skip(1) // TODO: filter out start and end instead
            .rev()
            .skip(1)
            .collect::<Vec<_>>();
        n.sort_by(|(_ia, a), (_ib, b)| a.cmp(b));
        n.into_iter().chain(self.matrix.iter().enumerate().take(1))
    }
}
