struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words = s.split_whitespace().rev().collect::<Vec<_>>().join(" ");
        words
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let s = String::from("  hello world  ");
        let result = Solution::reverse_words(s);
        assert_eq!(result, "world hello".to_string());
    }
}
