struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let bytes: &[u8] = s.as_bytes();
        let mut wild_stack = Vec::with_capacity(s.len());
        let mut stack = Vec::with_capacity(s.len());

        for (idx, &b) in bytes.iter().enumerate() {
            match b {
                b'(' => stack.push(idx),
                b'*' => wild_stack.push(idx),
                _ => {
                    if stack.pop() == None && wild_stack.pop() == None {
                        return false;
                    }
                }
            }
        }

        if stack.is_empty() {
            return true;
        }

        while let Some(idx) = stack.pop() {
            let Some(wild_idx) = wild_stack.pop() else {
                return false;
            };
            if idx > wild_idx {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
    }
}
