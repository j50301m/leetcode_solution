struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut stack = Vec::with_capacity(bytes.len());

        for &b in bytes {
            // let char = b as char;

            match b {
                b'a' | b'b' => {
                    stack.push(b);
                }
                _ => {
                    if stack.pop() != Some(b'b') || stack.pop() != Some(b'a') {
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
        assert_eq!(Solution::is_valid("abcabcababcc".to_string()), true);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::is_valid("abccba".to_string()), false);
    }
}
