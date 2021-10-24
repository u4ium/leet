use rand::{seq::SliceRandom, thread_rng};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

// split off an arbitrary element from a (non-empty) set
#[inline]
pub fn pop<T>(set: &mut HashSet<T>) -> T
where
    T: Eq + Clone + std::hash::Hash,
{
    let elt = set.iter().next().cloned().unwrap();
    set.remove(&elt);
    elt
}

// A set of i32 with insert and remove operations as well as uniform random access in O(1) time.
#[derive(Default)]
pub struct RandomizedCollection {
    // RI:
    //  IF a value is in the set:
    //    - index_map[value] == Some(index_set)
    //    - index_set contains exactly the indices of all the occurrences of value in values
    //  ELSE:
    //    - index_map[value] == None
    //    - values does not contain value
    index_map: HashMap<i32, HashSet<usize>>,
    values: Vec<i32>,
}

impl RandomizedCollection {
    /// Create a new RandomizedCollection instance
    pub fn new() -> Self {
        Default::default()
    }

    /// Insert a copy of val into the multiset and return true iff it is not already present
    pub fn insert(&mut self, val: i32) -> bool {
        let index_set = self.index_map.entry(val).or_default();
        let ret = index_set.is_empty();
        index_set.insert(self.values.len());
        self.values.push(val);
        ret
    }

    /// Remove a copy of val from the multiset and return true iff it was present
    pub fn remove(&mut self, val: i32) -> bool {
        let result = match self.index_map.get_mut(&val) {
            Some(indices_list) => {
                let last_value = self.values.pop().unwrap(); // SAFE
                let last_index = self.values.len();
                let index = pop(indices_list);
                // If the value removed is not at the end of the list,
                if index < last_index {
                    // replace it with the value at the end of the list
                    self.values[index] = last_value;
                    let index_set_of_last = self.index_map.get_mut(&last_value).unwrap();
                    index_set_of_last.remove(&last_index);
                    index_set_of_last.insert(index);
                }
                true
            }
            None => false,
        };

        // Remove map entries that point to empty sets
        if result && self.index_map.get(&val).unwrap().is_empty() {
            self.index_map.remove(&val);
        }
        result
    }

    /// Get a random value that is present in the set with uniform probability.
    ///
    /// Panic if there are no items in the set.
    pub fn get_random(&self) -> i32 {
        *self
            .values
            .choose(&mut thread_rng())
            .expect("ERROR: Cannot call get_random on empty set")
    }

    /// Return true iff this set contains val
    pub fn contains(&self, val: i32) -> bool {
        self.index_map.contains_key(&val)
    }
}

impl FromIterator<i32> for RandomizedCollection {
    fn from_iter<I: IntoIterator<Item = i32>>(iterable: I) -> Self {
        let mut new_set = RandomizedCollection::new();
        for elem in iterable.into_iter() {
            new_set.insert(elem);
        }
        new_set
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
    fn test_initialize() {
        let set = RandomizedCollection::new();
        for i in 1..1000 {
            assert_eq!(false, set.contains(i));
        }

        let set = RandomizedCollection::from_iter(-100..=100);
        for i in -100..=100 {
            assert_eq!(true, set.contains(i));
        }
        for i in 101..200 {
            assert_eq!(false, set.contains(i));
        }
    }

    #[test]
    fn test_insert() {
        let mut set = RandomizedCollection::new();
        for i in -100..100 {
            assert_eq!(false, set.contains(i));
            assert_eq!(true, set.insert(i));
            assert_eq!(true, set.contains(i));
            assert_eq!(false, set.insert(i));
            assert_eq!(true, set.contains(i));
        }
    }

    #[test]
    fn test_multi_insert() {
        let mut set = RandomizedCollection::new();
        assert_eq!(true, set.insert(1));
        assert_eq!(false, set.insert(1));
        assert_eq!(false, set.insert(1));
        assert_eq!(vec![1, 1, 1], set.values);
    }

    fn check_set_integrity(set: &RandomizedCollection, tests: Vec<(usize, i32)>) {
        for (expect_occurrences, value) in tests {
            // Check that value occurs the correct number of times in set.values
            assert_eq!(
                expect_occurrences,
                set.values
                    .clone()
                    .into_iter()
                    .filter(|&e| e == value)
                    .count()
            );

            match (expect_occurrences, set.index_map.get(&value)) {
                (0, None) => (),
                (_, Some(index_set_for_given_value)) => {
                    // Check that the correct number of indices are in the set.index_map for this value
                    assert_eq!(expect_occurrences, index_set_for_given_value.len());
                    // Check that each index  in set.index_map for this value points to this value in self.values
                    for &index in index_set_for_given_value {
                        assert_eq!(value, set.values[index])
                    }
                }
                (_, None) => panic!("Empty index set with values expected"),
            }
        }
    }

    #[test]
    fn test_multi_remove() {
        let mut set = RandomizedCollection::from_iter([1, 2, 2, 3, 3, 3]);
        assert_eq!(false, set.remove(4));
        assert_eq!(true, set.remove(2));
        check_set_integrity(&set, vec![(1, 1), (1, 2), (3, 3)]);
        assert_eq!(true, set.remove(3));
        check_set_integrity(&set, vec![(1, 1), (1, 2), (2, 3)]);
        assert_eq!(true, set.remove(1));
        check_set_integrity(&set, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(true, set.remove(2));
        check_set_integrity(&set, vec![(0, 1), (0, 2), (2, 3)]);
        assert_eq!(true, set.remove(3));
        assert_eq!(true, set.remove(3));
        check_set_integrity(&set, vec![(0, 1), (0, 2), (0, 3), (0, 4)]);
        assert_eq!(HashMap::new(), set.index_map);
        assert_eq!(Vec::<i32>::new(), set.values);
    }

    #[test]
    fn test_remove() {
        let mut set = RandomizedCollection::from_iter(0..=20);
        for i in 0..=20 {
            assert_eq!(true, set.contains(i));
            assert_eq!(true, set.remove(i));
            assert_eq!(false, set.contains(i));
            assert_eq!(false, set.remove(i));
            assert_eq!(false, set.contains(i));
        }
        for i in 20..=30 {
            assert_eq!(false, set.contains(i));
            assert_eq!(false, set.remove(i));
            assert_eq!(false, set.contains(i));
        }
    }

    #[test]
    fn test_get_random() {
        let set = RandomizedCollection::from_iter(0..10);
        let mut occurrences = [0; 10];
        for _ in 0..10_000 {
            let random_value = set.get_random();
            occurrences[random_value as usize] += 1;
        }
        for i in 0..10 {
            assert!(1000 - occurrences[i] < 200);
        }
    }

    #[test]
    fn test_failed_example() {
        let mut set = RandomizedCollection::new();
        set.insert(0);
        set.remove(0);
        set.insert(-1);
        set.remove(0);
        assert_eq!(false, set.contains(0));
        println!("index_map: {:?}", set.index_map);
        println!("values: {:?}", set.values);
        for _ in 1..10 {
            assert_eq!(-1, set.get_random());
        }
    }

    #[test]
    fn test_failed_example_2() {
        let mut set = RandomizedCollection::new();
        set.insert(0);
        set.insert(1);
        set.remove(0);
        set.insert(2);
        set.remove(1);
        assert_eq!(false, set.contains(0));
        assert_eq!(false, set.contains(1));
        assert_eq!(true, set.contains(2));
        println!("index_map: {:?}", set.index_map);
        println!("values: {:?}", set.values);
        for _ in 1..10 {
            assert_eq!(2, set.get_random());
        }
    }
}
