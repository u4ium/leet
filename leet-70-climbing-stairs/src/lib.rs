use std::convert::TryInto;

struct Solution {}

impl Solution {
    /// Given a staircase of n stairs, return the number of different way to climb,
    /// where one or two steps may be taken at a time.
    pub fn climb_stairs(n: i32) -> i32 {
        binet(n + 1) as i32
    }
}

fn binet(n: i32) -> f64 {
    const root_5: f64 = 2.2360679774997;
    const phi: f64 = (1.0 + root_5) / 2.0;
    const psi: f64 = (1.0 - root_5) / 2.0;
    const root_5_rec: f64 = 1.0 / root_5;
    ((phi.powi(n) - psi.powi(n)) * root_5_rec).ceil()
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
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
