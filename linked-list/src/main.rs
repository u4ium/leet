use linked_list::*;
fn main() {
    let list: LinkedList<i32> = vec![1, 2, 3].into_iter().collect();
    println!("{}", list);
}
