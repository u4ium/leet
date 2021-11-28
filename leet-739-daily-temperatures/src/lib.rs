struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut hottest = 0;

        for (current_day, &temperature) in temperatures.iter().enumerate().rev() {
            if temperature >= hottest {
                hottest = temperature;
            } else {
                let mut days = 1;
                loop {
                    let previous_day = current_day + days as usize;
                    if temperatures[previous_day] > temperature {
                        break;
                    }
                    days += result[previous_day];
                }
                result[current_day] = days;
            }
        }

        result
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
