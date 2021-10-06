struct Solution {}

use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Knight {
    minimum_initial_health: i32,
    location: (usize, usize),
    current_health: i32,
}

type Dungeon = Vec<Vec<i32>>;

impl Knight {
    fn update_heath(mut self, dungeon: &Dungeon) -> Self {
        let room_value = dungeon[self.location.0][self.location.1];
        self.current_health += room_value;
        if self.current_health < 1 {
            self.minimum_initial_health += 1 - self.current_health;
            self.current_health = 1;
        }
        self
    }

    fn move_right(&self, dungeon: &Dungeon) -> Self {
        Knight {
            location: (self.location.0, self.location.1 + 1),
            ..*self
        }
        .update_heath(dungeon)
    }

    fn move_down(&self, dungeon: &Dungeon) -> Self {
        Knight {
            location: (self.location.0 + 1, self.location.1),
            ..*self
        }
        .update_heath(dungeon)
    }
}

impl Solution {
    /// Return the knight's minimum initial health so that he can rescue the princes, where:
    ///
    /// - The knight has an initial health point represented by a positive integer.
    /// - If at any point his health point drops to 0 or below, he dies immediately.
    /// - The knight loses or gains health in each dungeon room according to the number.
    /// - The knight starts at (0, 0) and must reach the other corner
    /// - The knight may only move rightward and downward
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let end = (dungeon.len() - 1, dungeon[0].len() - 1);
        let mut pq = BinaryHeap::new();
        pq.push(Reverse(Knight {
            minimum_initial_health: 1 + max(0, -dungeon[0][0]),
            location: (0, 0),
            current_health: 1 + max(0, dungeon[0][0]),
        }));
        while let Some(Reverse(knight)) = pq.pop() {
            if knight.location == end {
                return knight.minimum_initial_health;
            }
            if knight.location.0 < end.0 {
                pq.push(Reverse(knight.move_down(&dungeon)));
            }
            if knight.location.1 < end.1 {
                pq.push(Reverse(knight.move_right(&dungeon)));
            }
        }
        panic!("Path not found")
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
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ]),
            7
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![0]]), 1);
    }
}
