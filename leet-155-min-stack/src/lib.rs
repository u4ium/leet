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
        todo!();
    }

    fn pop(&mut self) {
        todo!();
    }

    fn top(&self) -> i32 {
        todo!();
    }

    fn get_min(&self) -> i32 {
        todo!();
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
            assert_eq!(v, stack.pop())
        }
    }
}
