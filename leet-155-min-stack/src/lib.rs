use std::iter::FromIterator;

#[derive(Default)]
struct MinStack {
    data: Vec<i32>,
    minimums: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, val: i32) {
        if let Some(&minimum) = self.minimums.last() {
            if val <= minimum {
                self.minimums.push(val);
            }
        } else {
            self.minimums.push(val);
        }
        self.data.push(val);
    }

    fn pop(&mut self) {
        let popped = self.data.pop().unwrap();
        let minimum = self.minimums.last().unwrap();
        if popped == *minimum {
            self.minimums.pop().unwrap();
        }
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.minimums.last().unwrap()
    }
}

impl FromIterator<i32> for MinStack {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = i32>,
    {
        let mut stack = MinStack::new();
        for elem in iterable {
            stack.push(elem);
        }
        stack
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
        let mut minStack = MinStack::new();
        minStack.push(-2);
        minStack.push(0);
        minStack.push(-3);
        assert_eq!(-3, minStack.get_min());
        minStack.pop();
        assert_eq!(0, minStack.top());
        assert_eq!(-2, minStack.get_min());
    }

    #[test]
    fn test_initialize() {
        let mut stack = MinStack::from_iter(1..5);
        for v in (1..5).rev() {
            assert_eq!(v, stack.top());
            stack.pop();
        }
    }
}
