struct Solution {}

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut stack = Vec::new();

        stack.push(0);
        for &b in bytes {
            match b {
                b'(' => {
                    stack.push(0);
                }
                _ => {
                    let val = stack.pop().unwrap();
                    let score = if val == 0 { 1 } else { val * 2 };
                    *stack.last_mut().unwrap() += score;
                }
            }
        }

        stack[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6);
    }
}
