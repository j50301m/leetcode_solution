struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let bytes = s.as_bytes();
        let mut stack = Vec::with_capacity(s.len());
        let mut result = String::new();
        for (i, &b) in bytes.iter().enumerate() {
            match b {
                b'(' => {
                    stack.push(i);
                }
                _ => {
                    let pre_idx = stack.pop().unwrap();
                    if !stack.is_empty() {
                        continue;
                    }

                    result += &s[pre_idx + 1..i];
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
    }
}
