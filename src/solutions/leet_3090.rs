use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut map = HashMap::<u8, i32>::new();
        let mut left = 0;
        let mut best = 0;
        for right in 0..s.len() {
            *map.entry(bytes[right]).or_insert(0) += 1;

            while map[&bytes[right]] > 2 {
                *map.get_mut(&bytes[left]).unwrap() -= 1;
                left += 1;
            }

            best = best.max(right + 1 - left);
        }
        best as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::maximum_length_substring("bcbbbcba".to_string()),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::maximum_length_substring("aaaaa".to_string()), 2);
    }
}
