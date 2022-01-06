pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    /// # Panics
    /// If trip is not exactly 3 elements long
    fn to_triplet(trip: Vec<i32>) -> [i32; 3] {
        trip.try_into().unwrap()
    }

    /// Return true if it is possible to pick up and drop off all passengers for all the given trips,
    /// or false otherwise.
    ///
    /// There is a car with capacity empty seats.
    /// The vehicle only drives east (i.e., it cannot turn around and drive west).
    /// You are given the integer capacity and an array trips where
    ///  - `tripᵢ` = `[num_passengersᵢ, fromᵢ, toᵢ]` indicates that the `ith` trip has:
    ///    - `num_passengersᵢ` passengers and
    ///    - the locations to pick them up and drop them off are:
    ///      - `fromᵢ` and
    ///      - `toᵢ`, respectively.
    ///  - the locations are given as the number of kilometers due east from the car's initial location.
    ///  
    /// # Constraints:
    ///  - 1 <= `trips.length`` <= 1000
    ///  - `trips[i].length` == 3
    ///  - 1 <= `num_passengersᵢ` <= 100
    ///  - 0 <= `fromᵢ` < `toᵢ` <= 1000
    ///  - 1 <= `capacity` <= 10⁵
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut stops: BTreeMap<i32, i32> = BTreeMap::new();
        for [num_passengers, from, to] in trips.into_iter().map(Self::to_triplet) {
            *stops.entry(from).or_default() += num_passengers;
            *stops.entry(to).or_default() -= num_passengers;
        }

        let mut current_occupancy = 0;
        for passenger_delta in stops.values() {
            current_occupancy += passenger_delta;
            if current_occupancy > capacity {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    fn can_car_pool(trips: Vec<[u16; 3]>, capacity: u32) -> bool {
        let trips = trips
            .into_iter()
            .map(|trip| trip.into_iter().map(|i| i.try_into().unwrap()).collect())
            .collect();
        let capacity = capacity.try_into().unwrap();
        Solution::car_pooling(trips, capacity)
    }

    #[test]
    fn example_1() {
        let trips = vec![[2, 1, 5], [3, 3, 7]];
        let capacity = 4;
        assert_eq!(false, can_car_pool(trips, capacity));
    }

    #[test]
    fn example_2() {
        let trips = vec![[2, 1, 5], [3, 3, 7]];
        let capacity = 5;
        assert_eq!(true, can_car_pool(trips, capacity));
    }
}
