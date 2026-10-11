use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut memo = HashMap::new();
        let mut max_len = 0;
        let mut sum = 0;
        let mut ptr: usize = 0;
        for b in bytes.iter() {
            *memo.entry(b).or_insert(0) += 1;
            sum += 1;
            while let Some(&val) = memo.get(b) {
                if val <= 1 {
                    break;
                }
                *memo.get_mut(&bytes[ptr]).unwrap() -= 1;
                ptr += 1;
                sum -= 1;
            }
            max_len = max_len.max(sum);
        }

        max_len.max(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbbb".to_string()),
            1
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::length_of_longest_substring("S".to_string()), 1);
    }
}
