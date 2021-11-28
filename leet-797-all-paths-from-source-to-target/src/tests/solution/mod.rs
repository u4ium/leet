use super::Solution;

pub struct TestParameters {
    pub graph: Vec<Vec<usize>>,
    pub expect: Vec<Vec<usize>>,
}

mod runner {
    use super::{Solution, TestParameters};

    use std::collections::HashSet;

    pub struct Test {
        graph: Vec<Vec<i32>>,
        expect: HashSet<Vec<i32>>,
    }

    impl Test {
        pub fn new(params: TestParameters) -> Self {
            fn to_i32(list: Vec<usize>) -> Vec<i32> {
                list.into_iter().map(|n| n.try_into().unwrap()).collect()
            }
            Test {
                graph: params.graph.into_iter().map(to_i32).collect(),
                expect: params.expect.into_iter().map(to_i32).collect(),
            }
        }

        pub fn run(self) {
            assert_eq!(
                self.expect,
                Solution::all_paths_source_target(self.graph)
                    .into_iter()
                    .collect()
            )
        }
    }
}

mod examples;
mod extra;
