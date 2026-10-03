struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut stack = Vec::with_capacity(bytes.len() + 1);
        stack.push(-1);

        let mut max_len = 0;
        for (idx, &b) in bytes.iter().enumerate() {
            let idx = idx as i32;
            match b {
                b'(' => {
                    stack.push(idx);
                }
                _ => {
                    if let Some(_) = stack.pop() {
                        if let Some(&last_idx) = stack.last() {
                            max_len = max_len.max(idx - last_idx);
                            continue;
                        }
                        stack.push(idx);
                    }
                }
            }
        }

        max_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::longest_valid_parentheses("(()(()".to_string()), 2);
    }
}
