use leet_380_insert_delete_get_random_constant_time::RandomizedSet;

fn main() {
    let mut obj = RandomizedSet::new();
    let val = 42;
    let ret_1: bool = obj.insert(val);
    let ret_2: bool = obj.remove(val);
    let ret_3: i32 = obj.get_random();
}
