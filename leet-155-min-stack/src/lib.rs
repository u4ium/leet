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

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

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
}
