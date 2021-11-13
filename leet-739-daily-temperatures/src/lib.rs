struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack: Vec<usize> = Vec::new();

        for (index, &temperature) in temperatures.iter().enumerate() {
            while {
                match stack.last() {
                    Some(&previous_day) if temperatures[previous_day] < temperature => true,
                    _ => false,
                }
            } {
                let previous_day = stack.pop().unwrap();
                result[previous_day] = index - previous_day;
            }

            stack.push(index)
        }

        result.into_iter().map(|v| v as i32).collect()
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
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expect = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(expect, Solution::daily_temperatures(temperatures));
    }

    #[test]
    fn example_2() {
        let temperatures = vec![30, 40, 50, 60];
        let expect = vec![1, 1, 1, 0];
        assert_eq!(expect, Solution::daily_temperatures(temperatures));
    }

    #[test]
    fn example_3() {
        let temperatures = vec![30, 60, 90];
        let expect = vec![1, 1, 0];
        assert_eq!(expect, Solution::daily_temperatures(temperatures));
    }
}
