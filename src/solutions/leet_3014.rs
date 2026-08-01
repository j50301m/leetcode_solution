struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let n = word.len();
        let mut sum = 0;
        for i in 0..n {
            sum += (i / 8 + 1) as i32
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let s = String::from("abcde");
        let result = Solution::minimum_pushes(s);
        assert_eq!(result, 5);
    }
}
