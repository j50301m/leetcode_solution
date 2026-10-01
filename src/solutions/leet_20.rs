struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let bytes = s.as_bytes();

        for &b in bytes {
            let char = b as char;

            match char {
                '(' | '[' | '{' => stack.push(char),
                _ => {
                    let Some(top) = stack.pop() else {
                        return false;
                    };
                    if (top == '(' && char == ')')
                        || (top == '[' && char == ']')
                        || (top == '{' && char == '}')
                    {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }
}
