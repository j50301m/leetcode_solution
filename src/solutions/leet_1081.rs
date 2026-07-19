use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let s = s.as_bytes();
        let idx = |b: u8| (b - b'a') as usize;

        let mut last = [0usize; 26];
        for (pos, &b) in s.iter().enumerate() {
            last[idx(b)] = pos;
        }

        let mut in_stack = [false; 26];
        let mut stack: Vec<u8> = Vec::new();
        for (pos, &b) in s.iter().enumerate() {
            if in_stack[idx(b)] {
                continue;
            }
            while let Some(&top) = stack.last() {
                if top > b && last[idx(top)] > pos {
                    stack.pop();
                    in_stack[idx(top)] = false;
                } else {
                    break;
                }
            }
            stack.push(b);
            in_stack[idx(b)] = true;
        }

        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let str = String::from("bcabc");
        let result = Solution::smallest_subsequence(str);
        assert_eq!(result, "abc".to_string())
    }

    #[test]
    fn case_2() {
        let str = String::from("cbacdcbc");
        let result = Solution::smallest_subsequence(str);
        assert_eq!(result, "acdb".to_string())
    }
}
