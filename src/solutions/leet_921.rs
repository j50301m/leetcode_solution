struct Solution {}

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut stack = Vec::with_capacity(s.len());
        let mut accumulate = 0;
        for &b in bytes {
            match b {
                b'(' => {
                    stack.push(b'(');
                }
                _ => {
                    if stack.pop() == None {
                        accumulate += 1;
                    }
                }
            }
        }

        accumulate + stack.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
    }
}
