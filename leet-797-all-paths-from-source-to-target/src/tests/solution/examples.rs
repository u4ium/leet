use super::{runner::Test, TestParameters};

#[test]
fn example_2() {
    Test::new(TestParameters {
        graph: vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]],
        expect: vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ],
    })
    .run();
}

#[test]
fn example_1() {
    Test::new(TestParameters {
        graph: vec![vec![1, 2], vec![3], vec![3], vec![]],
        expect: vec![vec![0, 1, 3], vec![0, 2, 3]],
    })
    .run();
}

#[test]
fn example_3() {
    Test::new(TestParameters {
        graph: vec![vec![1], vec![]],
        expect: vec![vec![0, 1]],
    })
    .run();
}

#[test]
fn example_4() {
    Test::new(TestParameters {
        graph: vec![vec![1, 2, 3], vec![2], vec![3], vec![]],
        expect: vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]],
    })
    .run();
}

#[test]
fn example_5() {
    Test::new(TestParameters {
        graph: vec![vec![1, 3], vec![2], vec![3], vec![]],
        expect: vec![vec![0, 1, 2, 3], vec![0, 3]],
    })
    .run();
}
