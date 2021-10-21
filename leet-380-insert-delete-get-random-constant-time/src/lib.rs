use rand::{rngs::ThreadRng, Rng};
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FromIterator;

// A set of i32 with insert and remove operations as well as uniform random access in O(1) time.
pub struct RandomizedSet {
    index_map: HashMap<i32, usize>,
    values: Vec<i32>,
    rng: RefCell<ThreadRng>,
}

impl RandomizedSet {
    /// Create a new RandomizedSet instance
    pub fn new() -> Self {
        RandomizedSet {
            index_map: HashMap::new(),
            values: Vec::new(),
            rng: RefCell::new(rand::thread_rng()),
        }
    }

    /// Insert val into the randomized set and return true iff it is not already present
    pub fn insert(&mut self, val: i32) -> bool {
        if self.index_map.contains_key(&val) {
            return false;
        }
        self.index_map.insert(val, self.values.len());
        self.values.push(val);
        true
    }

    /// Remove val from the randomized set and return true iff it was present
    pub fn remove(&mut self, val: i32) -> bool {
        match self.index_map.remove(&val) {
            Some(index) => {
                let last_value = self.values.pop().unwrap(); // SAFE
                if index < self.values.len() {
                    self.values[index] = last_value;
                    self.index_map.insert(last_value, index);
                }
                true
            }
            None => false,
        }
    }

    /// Get a random value that is present in the set with uniform probability.
    ///
    /// Panic if there are no items in the set.
    pub fn get_random(&self) -> i32 {
        self.values[self.rng.borrow_mut().gen_range(0..self.values.len())]
    }

    /// Return true iff this set contains val
    fn contains(&self, val: i32) -> bool {
        self.index_map.contains_key(&val)
    }
}

impl FromIterator<i32> for RandomizedSet {
    fn from_iter<I: IntoIterator<Item = i32>>(iterable: I) -> Self {
        let mut new_set = RandomizedSet::new();
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
        let set = RandomizedSet::new();
        for i in 1..1000 {
            assert_eq!(false, set.contains(i));
        }

        let set = RandomizedSet::from_iter(-100..=100);
        for i in -100..=100 {
            assert_eq!(true, set.contains(i));
        }
        for i in 101..200 {
            assert_eq!(false, set.contains(i));
        }
    }

    #[test]
    fn test_insert() {
        let mut set = RandomizedSet::new();
        for i in -100..100 {
            assert_eq!(false, set.contains(i));
            assert_eq!(true, set.insert(i));
            assert_eq!(true, set.contains(i));
            assert_eq!(false, set.insert(i));
            assert_eq!(true, set.contains(i));
        }
    }

    #[test]
    fn test_remove() {
        let mut set = RandomizedSet::from_iter(0..=20);
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
        let set = RandomizedSet::from_iter(0..10);
        let mut occurrences = [0; 10];
        for i in 0..10_000 {
            let random_value = set.get_random();
            occurrences[random_value as usize] += 1;
        }
        for i in 0..10 {
            assert!(1000 - occurrences[i] < 200);
        }
    }

    #[test]
    fn test_failed_example() {
        let mut set = RandomizedSet::new();
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
        let mut set = RandomizedSet::new();
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
