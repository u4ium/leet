use super::{runner::Test, TestParameters};

#[test]
fn extra() {
    Test::new(TestParameters {
        graph: vec![vec![2], vec![3], vec![1], vec![]],
        expect: vec![vec![0, 2, 1, 3]],
    })
    .run();
}
