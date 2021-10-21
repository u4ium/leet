struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
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
    fn test_example_1() {
        let input = String::from("the sky is blue");
        let expect = String::from("blue is sky the");
        assert_eq!(expect, Solution::reverse_words(input));
    }

    #[test]
    fn test_example_2() {
        let input = String::from("  hello world  ");
        let expect = String::from("world hello");
        assert_eq!(expect, Solution::reverse_words(input));
    }

    #[test]
    fn test_example_3() {
        let input = String::from("a good   example");
        let expect = String::from("example good a");
        assert_eq!(expect, Solution::reverse_words(input));
    }

    #[test]
    fn test_example_4() {
        let input = String::from("  Bob    Loves  Alice   ");
        let expect = String::from("Alice Loves Bob");
        assert_eq!(expect, Solution::reverse_words(input));
    }

    #[test]
    fn test_example_5() {
        let input = String::from("Alice does not even like bob");
        let expect = String::from("bob like even not does Alice");
        assert_eq!(expect, Solution::reverse_words(input));
    }
}
